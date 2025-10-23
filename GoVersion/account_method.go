package mmcll

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"strings"
	"time"
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
func (url *UrlMethod) baseRequest(reqMethod string, header map[string]string, cookie map[string]string, data []byte) ([]byte, error) {
	if reqMethod == "" {
		reqMethod = http.MethodGet
	}
	if header == nil {
		header = make(map[string]string)
		header["Content-Type"] = "application/x-www-form-urlencoded;charset=UTF-8"
	}
	if header["User-Agent"] == "" {
		header["User-Agent"] = LauncherUserAgent
	}
	req, err := http.NewRequest(reqMethod, url.url, bytes.NewBuffer(data))
	if err != nil {
		return nil, err
	}
	for k, v := range header {
		req.Header.Set(k, v)
	}
	for k, v := range cookie {
		req.AddCookie(&http.Cookie{
			Name:  k,
			Value: v,
		})
	}
	client := &http.Client{}
	client.Timeout = time.Second * 10
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	// if resp.StatusCode < 200 || resp.StatusCode > 299 {
	// 	return nil, fmt.Errorf("http status code: %d", resp.StatusCode)
	// }
	return io.ReadAll(resp.Body)
}

// Post 请求
func (url *UrlMethod) Post(body string, isJSON bool) (string, error) {
	header := make(map[string]string)
	if isJSON {
		header["Content-Type"] = "application/json;charset=UTF-8"
		header["Accept"] = "application/json"
	} else {
		header["Content-Type"] = "application/x-www-form-urlencoded;charset=UTF-8"
	}
	res, err := url.baseRequest(http.MethodPost, header, nil, []byte(body))
	return string(res), err
}

// Get 请求，附带一个验证key的请求
func (url *UrlMethod) Get(authorization string) (string, error) {
	res, err := url.baseRequest(http.MethodGet, map[string]string{"Authorization": "Bearer " + authorization}, nil, []byte(""))
	return string(res), err
}

// GetDefault 默认请求，不带Authorization的请求，返回[]byte二进制数据，如果需要字符串转换，请自行添加string(res)
func (url *UrlMethod) GetDefault() ([]byte, error) {
	res, err := url.baseRequest(http.MethodGet, nil, nil, []byte(""))
	return res, err
}

// Post 请求，但是附带一个 header 和 Cookie！
func (url *UrlMethod) PostByHeaderAndCookie(header map[string]string, cookie map[string]string, body string) (string, error) {
	res, err := url.baseRequest(http.MethodPost, header, cookie, []byte(body))
	return string(res), err
}

// Get 请求，但是附带一个 header！
func (url *UrlMethod) GetByHeaderAndCookie(header map[string]string, cookie map[string]string) ([]byte, error) {
	res, err := url.baseRequest(http.MethodGet, header, cookie, []byte(""))
	return res, err
}

// GetCurseForge 默认请求，但是附带上x-api-key，返回string
func (url *UrlMethod) GetCurseForge(XApiKey string) (string, error) {
	res, err := url.baseRequest(http.MethodGet, map[string]string{
		"Accept":       "application/json",
		"Content-Type": "application/json",
		"x-api-key":    XApiKey,
	}, nil, []byte(""))
	return string(res), err
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

// AccountLogin key值在正版登录时，请输入client_id，如果是外置登录，请输入请求根网址【需要精确到/api/yggdrasil】，如果是 OAuth 登陆的第三方登陆，这里需要填写 ClientID。
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
		switch ww3 {
		case 2148916233:
			return nil, NewMMCLLError(-17, "Your Account has not Xbox precon, maybe you need to create an xbox precon and buy Minecraft, then try to precon again!")
		case 2148316235:
			return nil, NewMMCLLError(-18, "Your Account is banned, cause country or area is banned, so you cannot to precon, please try to mount a VPN and try again!")
		case 2148316236:
			return nil, NewMMCLLError(-19, "Your Account is a child precon, please precon with family.microsoft.com and authorization your child precon!")
		case 2148316237:
			return nil, NewMMCLLError(-20, "Your Account has risk, please precon your precon and pass the MSA authorization procedure!")
		case 2148316238:
			return nil, NewMMCLLError(-21, "Your Account adult verification is failed, please precon your precon and pass the adult verify!")
		case 2148316239:
			return nil, NewMMCLLError(-22, "Your Account Service Term is not pass, please precon your precon and pass the Service Term!")
		case 2148316240:
			return nil, NewMMCLLError(-23, "Your Account is not verified, please precon your precon and pass the verification!")
		default:
			return nil, NewMMCLLError(-1001, "Unknown Error, please feedback to author by this window! The error code is: "+strconv.FormatInt(ww3, 10))
		}
	}
	uhsXsts := Safe(j3, "", "DisplayClaims", "xui", 0, "uhs")
	if uhsXsts != uhsXbox {
		return nil, NewMMCLLError(-23, "The User Hash Xbox is not equal to the User Hash Xsts, Please try to precon again!")
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

// LoginMicrosoft 请尝试轮询这个接口，5秒一轮回。在返回 -6 错误时可以忽略。。其余的请向用户展示你所遇到的问题~
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
		switch res {
		case 70016:
			return nil, NewMMCLLError(-6, "Login Device Code is Invalid!")
		case 70020:
			return nil, NewMMCLLError(-7, "Login Time out!")
		default:
			return nil, NewMMCLLError(-8, "Unknown Error")
		}
	}
}
func (account *AccountLogin) RefreshMicrosoft(refreshToken string) (*AccountResult, error) {
	return nil, NewMMCLLError(-10001, "Not Implemented")
}

// GetUserCodeThirdOAuth 需要填入绝对的 URL 地址，例如【 https://open.littleskin.cn/oauth/device_code 】
// 在获取到 device_code 和 user_code 之后，你需要自行跳转到对应皮肤站的回调链接，例如【 https://open.littleskin.cn/oauth/link 】
func (account *AccountLogin) GetUserCodeThirdOAuth(server string) (string, string, error) {
	method := NewUrlMethod(server)
	k1 := fmt.Sprintf("client_id=%s&scope=User.Read%%20Player.Read%%20Yggdrasil.PlayerProfiles.Select%%20openid%%20offline_access", account.key)
	res, err := method.Post(k1, false)
	if err != nil {
		return "", "", NewMMCLLError(-101, err.Error())
	}
	var resJSON map[string]any
	if err = json.Unmarshal([]byte(res), &resJSON); err != nil {
		return "", "", NewMMCLLError(-102, err.Error())
	}
	userCode := Safe(resJSON, "", "user_code").(string)
	deviceCode := Safe(resJSON, "", "device_code").(string)
	if userCode == "" || deviceCode == "" {
		return "", "", NewMMCLLError(-103, "Cannot get User Code or Device Code to String!")
	}
	return userCode, deviceCode, nil
}

// LoginThirdPartyOAuth 这里的 server 需要填入绝对的 URL 地址，例如【 https://open.littleskin.cn/oauth/token 】
// 请尝试轮询这个接口，5秒一轮回。在返回 -106 错误时可以忽略。。其余的请向用户展示你所遇到的问题~
func (account *AccountLogin) LoginThirdPartyOAuth(server, deviceCode string) (*AccountResult, error) {
	k1 := fmt.Sprintf("grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id=%s&device_code=%s", account.key, deviceCode)
	u1 := NewUrlMethod(server)
	g1, err := u1.Post(k1, false)
	if err != nil {
		return nil, NewMMCLLError(-104, err.Error())
	}
	var resJSON map[string]any
	if err = json.Unmarshal([]byte(g1), &resJSON); err != nil {
		return nil, NewMMCLLError(-105, err.Error())
	}
	if res := Safe(resJSON, "", "access_token").(string); res != "" {
		padBase64 := func(raw string) string {
			raw = strings.TrimSpace(raw)
			padLength := (4 - len(raw)%4) % 4
			return raw + strings.Repeat("=", padLength)
		}
		idToken := Safe(resJSON, "", "id_token").(string)
		refreshToken := Safe(resJSON, "", "refresh_token").(string)
		if refreshToken == "" || idToken == "" {
			return nil, NewMMCLLError(-110, "Cannot get Refresh Token or ID Token!")
		}
		decode, err := base64.StdEncoding.DecodeString(padBase64(strings.Split(idToken, ".")[1]))
		if err != nil {
			return nil, NewMMCLLError(-111, err.Error())
		}
		var resJSON map[string]any
		if err = json.Unmarshal(decode, &resJSON); err != nil {
			return nil, NewMMCLLError(-112, err.Error())
		}
		profile := Safe(resJSON, map[string]any{}, "selectedProfile").(map[string]any)
		username := Safe(profile, "", "name").(string)
		userUUID := Safe(profile, "", "id").(string)
		if userUUID == "" || username == "" {
			return nil, NewMMCLLError(-113, "Cannot get User UUID or User Name!")
		}
		return &AccountResult{
			name:         username,
			uuid:         userUUID,
			accessToken:  res,
			refreshToken: refreshToken,
			clientToken:  "",
			base:         "",
		}, nil
	} else {
		res := Safe(resJSON, "", "error", 0).(string)
		switch res {
		case "authorization_pending":
			return nil, NewMMCLLError(-106, "Login Device Code is Invalid!")
		case "expire_token":
			return nil, NewMMCLLError(-107, "Login Time out!")
		default:
			return nil, NewMMCLLError(-108, "Unknown Error")
		}
	}
}
func (account *AccountLogin) RefreshThirdOAuth(server string, refreshToken string) (*AccountResult, error) {
	return nil, NewMMCLLError(-10002, "Not Implemented")
}
func (account *AccountLogin) LoginThirdParty(username, password string) ([]*AccountResult, error) {
	u1 := fmt.Sprintf("%s/authserver/authenticate", account.key)
	k1 := fmt.Sprintf(`{
	"username": "%s",
	"password": "%s",
	"requestUser": false,
	"agent": {
		"name": "Minecraft",
		"version": 1
	}
}`, username, password)
	w1 := NewUrlMethod(u1)
	t1, err := w1.Post(k1, true)
	if err != nil {
		return nil, NewMMCLLError(-120, err.Error())
	}
	var j1 map[string]any
	if err = json.Unmarshal([]byte(t1), &j1); err != nil {
		return nil, NewMMCLLError(-121, err.Error())
	}
	a1 := Safe(j1, "", "accessToken").(string)
	if a1 == "" {
		mar, _ := json.Marshal(j1)
		return nil, NewMMCLLError(-121, "Cannot get Access Token, The Error JSON Is: "+string(mar))
	}
	r1 := Safe(j1, []any{}, "availableProfiles").([]any)
	var res []*AccountResult
	for _, v := range r1 {
		name := Safe(v, "", "name").(string)
		uuid := Safe(v, "", "id").(string)
		if name == "" || uuid == "" {
			continue
		}
		res = append(res, &AccountResult{
			name:         name,
			uuid:         uuid,
			accessToken:  a1,
			refreshToken: "",
			clientToken:  "",
			base:         "",
		})
	}
	return res, nil
}
