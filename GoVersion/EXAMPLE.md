## 这里是使用 MMCLL 启动游戏的一个小示例：

```go
// 测试 MMCLL 启动游戏~
func TestMMCLL(t *testing.T) {
    account := mmcll.NewLaunchAccountOffline("aooooo", "1234567980abcdef1234567890abcdef")
    gp := "D:/mc/testmc/.minecraft/versions/1.1"
    options := mmcll.NewLaunchOption(account, "D:/Languages/Java/jdk1.8.0_311/jre/bin/java.exe", "D:/mc/testmc/.minecraft", gp, gp)
    err := mmcll.LaunchGame(*options, true, func(back []string) {
        cmd := exec.Command(back[0], back[1:]...)
        stdout, _ := cmd.StdoutPipe()
        err := cmd.Start()
        if err != nil {
            t.Fatal(err)
            return
        }
        scanner := bufio.NewScanner(stdout)
        for scanner.Scan() {
            fmt.Println(scanner.Text())
        }
        err = cmd.Wait()
        if err != nil {
            t.Fatal(err)
            return
        }
    })
    if err != nil {
        t.Error(err)
    }
}
```

back 的 第一个元素即是 Java 路径，剩余全部元素都是 参数！

## 这里是使用 MMCLL 登录微软账号的一个小示例：

```go
// TestMicrosoftLogin 测试微软登录，测试通过！！
func TestMicrosoftLogin(t *testing.T) {
    // 下列字段将文件被忽略，你需要填入自己的 Client ID！
    clientId := launcher.ClientId()
    login := mmcll.NewAccountLogin(clientId)
    uc, dc, err := login.GetUserCode()
    if err != nil {
        t.Error(err)
    }
    fmt.Println(uc)
    for {
        time.Sleep(5 * time.Second)
        s, err2 := login.LoginMicrosoft(dc)
        if err2 != nil {
            var err3 mmcll.ErrorMMCLL
            errors.As(err2, &err3)
            // 这里需要忽略的错误码是 -6，如果请求出来的结果不是 -6，则直接break！或者你也可以打印Error（
            if err3.Code() != -6 {
                break
            }
        } else {
            fmt.Println(s.Name())
            fmt.Println(s.Uuid())
            fmt.Println(s.AccessToken())
            t.Log("Login success!")
            break
        }
    }
}
```

## 使用 MMCLL 登录第三方账号的一个示例！

- 目前 MMCLL 支持 OAuth 登录，只需要将各个网址和 Client ID 填入即可！

```go
func TestThirdOAuthLogin(t *testing.T) {
    // 下列字段被忽略，你需要填入自己的 Client ID
    // 可能每个皮肤站的 Client ID 都不一样，你需要自行做处理！
    // 下面将以 LittleSkin 作为示例
    clientId := launcher.LittleSkinKey()
    login := mmcll.NewAccountLogin(clientId)
    // 以下需要填入你的 device_code 请求地址
    uc, dc, err := login.GetUserCode("https://open.littleskin.cn/oauth/device_code")
    if err != nil {
        t.Error(err)
    }
    fmt.Println(uc)
    for {
        time.Sleep(5 * time.Second)
        // 以下需要填入你的 token 请求地址：
        s, err2 := login.LoginThirdPartyOAuth("https://open.littleskin.cn/oauth/token", dc)
        if err2 != nil {
            var err3 mmcll.ErrorMMCLL
            errors.As(err2, &err3)
            // 这里需要忽略的错误码是 -106，如果请求出来的结果不是 -106，则直接break！或者你也可以打印Error（
            if err3.Code() != -106 {
                break
            }
        } else {
            fmt.Println(s.Name())
            fmt.Println(s.Uuid())
            fmt.Println(s.AccessToken())
            t.Log("Login success!")
            break
        }
    }
}
```

- MMCLL 的第三方账号密码登录：

```go
func TestThirdLogin(t *testing.T) {
    // 下列字段需要你让用户手动输入服务器
    server := mmcll.NewAccountLogin("https://littleskin.cn/api/yggdrasil")
    // 下列字段需要你让用户手动输入账号密码
    login, err := mmcll.LoginThirdParty(username, password)
    if err != nil {
        painc(err)
    }
    fmt.Println(login.Name())
    fmt.Println(login.Uuid())
    fmt.Println(login.AccessToken())
    t.Log("Login success!")
    break
}
```

## 使用 MMCLL 下载单个文件示例：

```go
func TestDownloadSingleFile(t *testing.T) {
    // 由于 单文件下载 没做重试，因此最后一个参数只需要填入 0 即可！
    // 第二个参数是最大线程数量！
    downloader := mmcll.NewDownloadSingle("https://example.com/file1.zip", "D:\\file1.zip", 32, 0)
    // 文件下载是以闭包形式传递的！
    // 第一个参数 url 是下载原网址
    // 第二个参数原本是要填入重试次数的，但由于是下载单文件，因此暂时不需要填。
    // 第三个参数是返回的错误信息，如果出错了才会显示，否则不会显示！
    // 第四个参数是返回的状态码，如果没错的话，一般返回200。
    // 第五个参数是总体进度。【如果请求时，网址头未附带 Range=bytes，则这里默认的下载是文件总体大小。（2GB 大小的文件一般是2147483647）】（如果文件已经附带了 Range=bytes，那么此时这里应该编程你上面填入的最大线程数量！）
    // 第六个参数是当前下载总量，各位可以每秒钟获取一次，将current保存到上一个变量里，随后以当前current - 上一次的current得到下载速度。
    downloader.StartDownload(func (url string, _ int, errmessage string, status int, all int64, current int64) {
        fmt.Println(url, errmessage, status, all, current)
    })
}
```

- 上述下载单个文件只是理论上最小，当然如果你需要填入【headers、cookies、代理Proxy】，那还得填入别的参数！具体所有参数在 MMCLL 里面已经说的很清楚了！
- 比如 你需要模拟浏览器的下载，那么 headers 就必须填入 User-Agent: Mozilla/5.0