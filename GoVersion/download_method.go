package mmcll

import (
	"errors"
	"fmt"
	"io"
	"math"
	"math/rand"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"sync"
)

// ProxyType 代理设置
//
// 如果有代理的话，用户名为空字符串则没有校验。url为空字符串时，将不设置代理。
type ProxyType struct {
	Url      string // 代理网址
	Port     string // 代理端口
	Username string // 验证用户名
	Password string // 验证密码
	IsHttps  bool   // 是否是 https 代理
	IsSocks  bool   // 是否是 Socks5 代理（此优先级比 Https 高）
}

// Downloader 一个简单的接口，用于处理同样的下载单文件和多文件的情况。
//
// 内函数无需返回值，只需要判断闭包内是否有错即可！
//
// 实际上所有的请求错误都在 闭包 函数里面给你们了~
//
// 闭包参数如下：
//  1. 当前网址
//  2. 重试次数
//  3. 请求返回错误信息（如果为空则没错）
//  4. 请求状态码
//  5. 总的下载量
//  6. 当前下载量
//
// 各位也可以实现自己的下载器，只需要实现 StartDownload 方法即可。
type Downloader interface {
	StartDownload(callback func(string, int, string, int, int64, int64))
}

// DownloadSingle 下载单文件~
//
// 默认会采取切片下载的方式，将会尝试进行切片下载，如果不成功的话，则只会调用单线程下载。。
//
// MaximumThread 最大线程
// RetryCount 重试次数
// Proxy 代理设置
type DownloadSingle struct {
	Url           string
	SavePath      string
	MaximumThread int
	RetryCount    int
	Proxy         ProxyType
	Headers       map[string]string
	Cookies       map[string]string
}

// DownloadMultiFile 下载很多个文件，其中 Urls 和 SavePaths 需要一一对应
//
// 如果不对应， Urls 比 SavePaths 多，则会将 最后的几个 Urls 下载到最后一个 SavePaths 里。
//
// 当 SavePaths 比 Urls 多的话，则会对应所有的 Urls 保存到对应的 SavePaths 里。其余的 SavePaths 将丢弃
//
// 默认从不采取文件切片下载，因为当下载 MC 的 Assets 的时候，对于 3000 多个大文件下载，
// 如果尝试切片下载的话，会导致请求很多次。可能对网络要求较高。。
type DownloadMultiFile struct {
	Urls          []string
	SavePaths     []string
	MaximumThread int
	RetryCount    int
	Proxy         ProxyType
}

func NewProxy(url string, port string, username string, password string, isHttp bool, isSocks bool) ProxyType {
	return ProxyType{
		Url:      url,
		Port:     port,
		Username: username,
		Password: password,
		IsHttps:  isHttp,
		IsSocks:  isSocks,
	}
}
func NewDownloadSingle(url string, savePath string, biggestThread int, retryCount int) DownloadSingle {
	return DownloadSingle{
		Url:           url,
		SavePath:      savePath,
		MaximumThread: biggestThread,
		RetryCount:    retryCount,
		Proxy:         NewProxy("", "", "", "", false, false),
		Headers:       make(map[string]string),
		Cookies:       make(map[string]string),
	}
}
func NewDownloadSingleWithProxy(url string, savePath string, biggestThread int, retryCount int, proxy ProxyType) DownloadSingle {
	return DownloadSingle{
		Url:           url,
		SavePath:      savePath,
		MaximumThread: biggestThread,
		RetryCount:    retryCount,
		Proxy:         proxy,
		Headers:       make(map[string]string),
		Cookies:       make(map[string]string),
	}
}
func NewDownloadWithHeadersCookies(url string, savePath string, biggestThread int, retryCount int, proxy ProxyType, headers map[string]string, cookies map[string]string) DownloadSingle {
	return DownloadSingle{
		Url:           url,
		SavePath:      savePath,
		MaximumThread: biggestThread,
		RetryCount:    retryCount,
		Proxy:         proxy,
		Headers:       headers,
		Cookies:       cookies,
	}
}

// NewHttpClient 创建统一代理客户端
func NewHttpClient(proxy ProxyType) *http.Client {
	return &http.Client{
		Transport: &http.Transport{
			Proxy: func(_ *http.Request) (*url.URL, error) {
				if proxy.Url == "" || proxy.Port == "" {
					return nil, nil
				}
				scheme := "http://"
				if proxy.IsSocks {
					scheme = "socks5://"
				} else if proxy.IsHttps {
					scheme = "https://"
				}
				auth := ""
				if proxy.Username != "" && proxy.Password != "" {
					auth = proxy.Username + ":" + proxy.Password + "@"
				}
				return url.Parse(fmt.Sprintf("%s%s%s:%s", scheme, auth, proxy.Url, proxy.Port))
			},
		},
	}
}

// GetFileSize 获取文件长度以及是否可以切片下载（用于下载单文件时能进行切片下载）
//
// 返回值1：文件大小
// 返回值2：是否允许切片下载
// 返回值3：请求状态码返回值
func GetFileSize(url string) (int64, bool, int, error) {
	client := &http.Client{}
	resp, err := client.Head(url)
	if err != nil {
		return -1, false, 500, err
	}
	defer resp.Body.Close()
	// 这里简单判断了一下，如果返回的是bytes的话，则返回true。。
	canCut := resp.Header.Get("Accept-Ranges") == "bytes"
	return resp.ContentLength, canCut, resp.StatusCode, nil
}

// DownloadRange 从网络上获取 并附带 Range 请求头。如果可以的话，还必须附上 Proxy 代理
func DownloadRange(urlParse string, start int64, end int64, proxy ProxyType, headers map[string]string, cookies map[string]string) ([]byte, error) {
	req, err := http.NewRequest(http.MethodGet, urlParse, nil)
	if err != nil {
		return nil, err
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}
	for k, v := range cookies {
		req.AddCookie(&http.Cookie{
			Name:  k,
			Value: v,
		})
	}
	req.Header.Set("Content-Type", "application/octet-stream")
	req.Header.Set("User-Agent", LauncherUserAgent)
	req.Header.Set("Range", fmt.Sprintf("bytes=%d-%d", start, end))
	client := NewHttpClient(proxy)
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, errors.New("unexpected status: " + resp.Status)
	}
	return io.ReadAll(resp.Body)
}

// StartDownload 单文件开始下载
//
// 实现起来的话，其实无需判断返回值，只需要判断闭包里是否有错就可以了~
//
// 对于单文件的下载来说，返回的闭包稍微有点不同：
//  1. 下载网址（一直都是你的网址）
//  2. 重试次数（一直都是0，不用判断，下载单文件时无法重试）
//  3. 返回错误信息（一般是错误描述）
//  4. 返回状态码（如果正常一般返回200）
//  5. 总的下载量（如果该网站允许多线程下载，则这个的值是最大线程）
//  6. 当前下载量（如果该网址允许多线程下载，则这个值是当前完成的线程数量）
func (d DownloadSingle) StartDownload(callback func(string, int, string, int, int64, int64)) {
	fileSize, canCut, status, err := GetFileSize(d.Url)
	exception := func(msg string) {
		callback(d.Url, 0, msg, 500, 0, 0)
	}
	if err != nil || status < 200 || status >= 300 {
		msg := "Web Status Is Not 200"
		if err != nil {
			msg = err.Error()
		}
		callback(d.Url, 0, msg, status, 0, 0)
		return
	}
	if fileSize > 0 && canCut {
		// 初始化一个随机数，用于在下载时不要随机到别的数字。
		randInt := strconv.FormatInt(rand.Int63(), 10)
		var wg sync.WaitGroup
		completed := 0
		success := true
		fileAvg := int64(math.Floor(float64(fileSize) / float64(d.MaximumThread)))
		downProc := func(url string, id int, start int64, end int64) {
			defer wg.Done()
			tempPath := filepath.Join(os.TempDir(), randInt+strconv.Itoa(id)+".tmp")
			stt, err := DownloadRange(url, start, end, d.Proxy, d.Headers, d.Cookies)
			completed++
			if err != nil {
				success = false
				callback(d.Url, 0, err.Error(), 500, int64(d.MaximumThread), int64(completed))
			} else {
				_ = SetBFile(tempPath, stt)
				callback(d.Url, 0, "", 200, int64(d.MaximumThread), int64(completed))
			}
		}
		for i := 0; i < d.MaximumThread; i++ {
			wg.Add(1)
			start := fileAvg * int64(i)
			end := fileAvg*int64(i+1) - 1
			if i == d.MaximumThread-1 {
				end = fileSize
			}
			go downProc(d.Url, i, start, end)
		}
		wg.Wait()
		if !success {
			for i := 0; i < d.MaximumThread; i++ {
				_ = os.Remove(filepath.Join(os.TempDir(), randInt+strconv.Itoa(i)+".tmp"))
			}
			return
		}
		out, err := os.Create(d.SavePath)
		if err != nil {
			exception(err.Error())
			return
		}
		defer out.Close()
		for i := 0; i < d.MaximumThread; i++ {
			in, err := os.Open(filepath.Join(os.TempDir(), randInt+strconv.Itoa(i)+".tmp"))
			if err != nil {
				exception(err.Error())
				continue
			}
			_, err = io.Copy(out, in)
			in.Close()
			if err != nil {
				exception(err.Error())
				continue
			}
			_ = os.Remove(filepath.Join(os.TempDir(), randInt+strconv.Itoa(i)+".tmp"))
		}
	} else if fileSize > 0 {
		request, err := http.NewRequest(http.MethodGet, d.Url, nil)
		if err != nil {
			exception(err.Error())
			return
		}
		for k, v := range d.Headers {
			request.Header.Set(k, v)
		}
		for k, v := range d.Cookies {
			request.AddCookie(&http.Cookie{
				Name:  k,
				Value: v,
			})
		}
		request.Header.Set("Content-Type", "application/octet-stream")
		request.Header.Set("User-Agent", LauncherUserAgent)
		client := NewHttpClient(d.Proxy)
		response, err := client.Do(request)
		if err != nil {
			exception(err.Error())
			return
		}
		defer response.Body.Close()
		if response.StatusCode < 200 && response.StatusCode >= 300 {
			exception("Status Code Is Not 200")
			return
		}
		f, err := os.Create(d.SavePath)
		if err != nil {
			exception(err.Error())
			return
		}
		defer f.Close()
		var downloaded int64 = 0
		buffer := make([]byte, 1024*1024)
		for {
			n, err := response.Body.Read(buffer)
			if err != nil && err != io.EOF {
				exception(err.Error())
				return
			}
			if n == 0 {
				break
			}
			_, err = f.Write(buffer[:n])
			if err != nil {
				exception(err.Error())
				return
			}
			downloaded += int64(n)
			callback(d.Url, 0, "", 200, fileSize, downloaded)
		}
	} else {
		exception("Not Support Download Method")
	}
}
