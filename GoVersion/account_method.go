package mmcll

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
)

type UrlMethod struct {
	// url 需要请求的网址
	url string
}

// NewUrlMethod 生成一个UrlMethod实例
func NewUrlMethod(url string) *UrlMethod {
	return &UrlMethod{url: url}
}

// baseRequest 基础的请求函数
func (url *UrlMethod) baseRequest(reqMethod string, header map[string]string, data []byte) ([]byte, error) {
	if reqMethod == "" {
		reqMethod = http.MethodGet
	}
	if header == nil {
		header = make(map[string]string)
		header["Content-Type"] = "application/x-www-form-urlencoded"
	}
	header["User-Agent"] = LauncherUserAgent
	req, err := http.NewRequest(reqMethod, url.url, bytes.NewBuffer(data))
	if err != nil {
		return nil, err
	}
	for k, v := range header {
		req.Header.Set(k, v)
	}
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	return body, nil
}

// Post 请求
func (url *UrlMethod) Post(body string, isJSON bool) (string, error) {
	header := make(map[string]string)
	if isJSON {
		header["Content-Type"] = "application/json;charset=UTF-8"
		header["Accept"] = "application/json"
	} else {
		header["Content-Type"] = "application/x-www-form-urlencoded"
	}
	res, err := url.baseRequest(http.MethodPost, header, []byte(body))
	return string(res), err
}

// Get 请求，附带一个验证key的请求
func (url *UrlMethod) Get(authorization string) (string, error) {
	res, err := url.baseRequest(http.MethodGet, map[string]string{"Authorization": "Bearer " + authorization}, []byte(""))
	return string(res), err
}

// GetDefault 默认请求，不带Authorization的请求，返回[]byte二进制数据，如果需要字符串转换，请自行添加string(res)
func (url *UrlMethod) GetDefault() ([]byte, error) {
	res, err := url.baseRequest(http.MethodGet, nil, []byte(""))
	return res, err
}

type AccountResult struct {
	name         string
	uuid         string
	accessToken  string
	refreshToken string
	clientToken  string
	base         string
}

func (a *AccountResult) Name() string {
	return a.name
}

func (a *AccountResult) SetName(name string) {
	a.name = name
}

func (a *AccountResult) Uuid() string {
	return a.uuid
}

func (a *AccountResult) SetUuid(uuid string) {
	a.uuid = uuid
}

func (a *AccountResult) AccessToken() string {
	return a.accessToken
}

func (a *AccountResult) SetAccessToken(accessToken string) {
	a.accessToken = accessToken
}

func (a *AccountResult) RefreshToken() string {
	return a.refreshToken
}

func (a *AccountResult) SetRefreshToken(refreshToken string) {
	a.refreshToken = refreshToken
}

func (a *AccountResult) ClientToken() string {
	return a.clientToken
}

func (a *AccountResult) SetClientToken(clientToken string) {
	a.clientToken = clientToken
}

func (a *AccountResult) Base() string {
	return a.base
}

func (a *AccountResult) SetBase(base string) {
	a.base = base
}

// AccountLogin key值在正版登录时，请输入client_id，如果是外置登录，请输入请求根网址【需要精确到/api/yggdrasil】。
type AccountLogin struct {
	key string
}

func NewAccountLogin(key string) *AccountLogin {
	return &AccountLogin{key: key}
}

// GetUserCode 获取微软登录代码，如果正常的话会返回一个user_code和一个device_code。
func (account *AccountLogin) GetUserCode() (string, string, error) {
	const URL = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode?mkt=zh-CN"
	method := NewUrlMethod(URL)
	k1 := fmt.Sprintf("client_id=%s&scope=XboxLive.signin%%20offline_access", account.key)
	res, err := method.Post(k1, false)
	if err != nil {
		return "", "", NewMMCLLError(-1, err.Error())
	}
	var resJSON map[string]any
	err = json.Unmarshal([]byte(res), &resJSON)
	if err != nil {
		return "", "", NewMMCLLError(-2, err.Error())
	}
	userCode := Safe(resJSON, "", "user_code").(string)
	deviceCode := Safe(resJSON, "", "device_code").(string)
	if userCode == "" || deviceCode == "" {
		return "", "", NewMMCLLError(-3, "Cannot convert user code or device code to string")
	}
	return userCode, deviceCode, nil
}

// microsoft 登录的时候是怎么登录
func (account *AccountLogin) microsoft(loginCode string) (*AccountResult, error) {
	const XboxLive = "https://user.auth.xboxlive.com/user/authenticate"
	const XstsLive = "https://xsts.auth.xboxlive.com/xsts/authorize"
	const McLive = "https://api.minecraftservices.com/authentication/login_with_xbox"
	const Verify = "https://api.minecraftservices.com/minecraft/profile"
	k2 := fmt.Sprintf("{\"Properties\":{\"AuthMethod\":\"RPS\",\"SiteName\":\"user.auth.xboxlive.com\",\"RpsTicket\":\"d=%s\"},\"RelyingParty\":\"http://auth.xboxlive.com\",\"TokenType\":\"JWT\"}", loginCode)
	u2 := NewUrlMethod(XboxLive)
	t2, err := u2.Post(k2, true)
	if err != nil {
		return nil, NewMMCLLError(-11, err.Error())
	}
	var j2 map[string]any
	if err = json.Unmarshal([]byte(t2), &j2); err != nil {
		return nil, NewMMCLLError(-12, err.Error())
	}
	w2 := Safe(j2, "", "Token")
	if w2 == "" {
		return nil, NewMMCLLError(-13, "Cannot get Token Value!")
	}
	uhsXbox := Safe(j2, "", "DisplayClaims", "xui", 0, "uhs")
	if uhsXbox == "" {
		return nil, NewMMCLLError(-14, "Cannot get User Hash Value!")
	}
	k3 := fmt.Sprintf("{\"Properties\":{\"SandboxId\":\"RETAIL\",\"UserTokens\":[\"%s\"]},\"RelyingParty\":\"rp://api.minecraftservices.com/\",\"TokenType\":\"JWT\"}", w2)
	u3 := NewUrlMethod(XstsLive)
	t3, err := u3.Post(k3, true)
	if err != nil {
		return nil, NewMMCLLError(-15, err.Error())
	}
	var j3 map[string]any
	if err = json.Unmarshal([]byte(t3), &j3); err != nil {
		return nil, NewMMCLLError(-16, err.Error())
	}
	w3 := Safe(j3, "", "Token")
	if w3 == "" {
		ww3 := int64(Safe(j3, -1.0, "XErr").(float64))
		if ww3 == 2148916233 {
			return nil, NewMMCLLError(-17, "Your Account has not Xbox account, maybe you need to create an xbox account and buy Minecraft, then try to login again!")
		} else if ww3 == 2148316235 {
			return nil, NewMMCLLError(-18, "Your Account is banned, cause country or area is banned, so you cannot to login, please try to mount a VPN and try again!")
		} else if ww3 == 2148316236 {
			return nil, NewMMCLLError(-19, "Your Account is a child account, please login with family.microsoft.com and authorization your child account!")
		} else if ww3 == 2148316237 {
			return nil, NewMMCLLError(-20, "Your Account has risk, please login your account and pass the MSA authorization procedure!")
		} else if ww3 == 2148316238 {
			return nil, NewMMCLLError(-21, "Your Account adult verification is failed, please login your account and pass the adult verify!")
		} else if ww3 == 2148316239 {
			return nil, NewMMCLLError(-22, "Your Account Service Term is not pass, please login your account and pass the Service Term!")
		} else {
			return nil, NewMMCLLError(-1001, "Unknown Error, please feedback to author by this window! The error code is: "+strconv.FormatInt(ww3, 10))
		}
	}
	uhsXsts := Safe(j3, "", "DisplayClaims", "xui", 0, "uhs")
	if uhsXsts != uhsXbox {
		return nil, NewMMCLLError(-23, "The User Hash Xbox is not equal to the User Hash Xsts, Please try to login again!")
	}
	k4 := fmt.Sprintf("{\"identityToken\":\"XBL3.0 x=%s;%s\"}", uhsXsts, w3)
	u4 := NewUrlMethod(McLive)
	t4, err := u4.Post(k4, true)
	if err != nil {
		return nil, NewMMCLLError(-24, err.Error())
	}
	var j4 map[string]any
	if err = json.Unmarshal([]byte(t4), &j4); err != nil {
		return nil, NewMMCLLError(-25, err.Error())
	}
	w4 := Safe(j4, "", "access_token").(string)
	if w4 == "" {
		return nil, NewMMCLLError(-26, "Cannot get Access Token Value!")
	}
	u5 := NewUrlMethod(Verify)
	t5, err := u5.Get(w4)
	if err != nil {
		return nil, err
	}
	var j5 map[string]any
	if err = json.Unmarshal([]byte(t5), &j5); err != nil {
		return nil, err
	}
	name := Safe(j5, "", "name").(string)
	uuid := Safe(j5, "", "id").(string)
	if name == "" || uuid == "" {
		return nil, NewMMCLLError(-29, "Cannot get User name or User UUID, Maybe you are not buy Minecraft, Please try again when you buy legal Minecraft!")
	}
	result := &AccountResult{
		name:         name,
		uuid:         uuid,
		accessToken:  w4,
		refreshToken: "",
		clientToken:  "",
		base:         "",
	}
	return result, nil
}
func (account *AccountLogin) LoginMicrosoft(deviceCode string) (*AccountResult, error) {
	const MsLive = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token"
	k1 := fmt.Sprintf("grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id=%s&device_code=%s", account.key, deviceCode)
	u1 := NewUrlMethod(MsLive)
	g1, err := u1.Post(k1, false)
	if err != nil {
		return nil, NewMMCLLError(-4, err.Error())
	}
	var resJSON map[string]any
	if err = json.Unmarshal([]byte(g1), &resJSON); err != nil {
		return nil, NewMMCLLError(-5, err.Error())
	}
	if res := Safe(resJSON, "", "access_token").(string); res != "" {
		refreshToken := Safe(resJSON, "", "refresh_token").(string)
		if refreshToken != "" {
			a, err := account.microsoft(res)
			if err != nil {
				return nil, err
			} else {
				a.SetRefreshToken(refreshToken)
				return a, nil
			}
		} else {
			return nil, NewMMCLLError(-10, "Cannot get Refresh token!")
		}
	} else {
		res := int(Safe(resJSON, -1.0, "error_codes", 0).(float64))
		if res == 70016 {
			return nil, NewMMCLLError(-6, "Login Device Code is Invalid!")
		} else if res == 70020 {
			return nil, NewMMCLLError(-7, "Login Time out!")
		} else {
			return nil, NewMMCLLError(-8, "Unknown Error")
		}
	}
}
func (account *AccountLogin) RefreshMicrosoft() {

}
