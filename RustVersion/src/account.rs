//! 登录账号的时需要用到的部分函数。需要初始化！
use crate::constants::USER_AGENT;
use crate::{MMCLLError, MMCLLResult};

pub struct UrlMethod {
    url: String,
}
impl UrlMethod {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    /// 以下三个为普通的网络请求，会阻塞主线程。
    /// 这里是post
    pub fn post(&self, key: &str, that: bool) -> Option<String> {
        let http = reqwest::blocking::Client::new();
        if that {
            let head = "application/x-www-form-urlencoded;charset=utf-8";
            let res = http
                .post(self.url.as_str())
                .timeout(std::time::Duration::from_secs(100))
                .header(reqwest::header::USER_AGENT, USER_AGENT)
                .header(reqwest::header::CONTENT_TYPE, head)
                .body(key.to_string())
                .send()
                .ok()?
                .text()
                .ok()?;
            Some(res.clone())
        } else {
            let head = "application/json";
            let res = http
                .post(self.url.as_str())
                .timeout(std::time::Duration::from_secs(100))
                .header(reqwest::header::USER_AGENT, USER_AGENT)
                .header(reqwest::header::CONTENT_TYPE, head)
                .header(reqwest::header::ACCEPT, head)
                .body(key.to_string())
                .send()
                .ok()?
                .text()
                .ok()?;
            Some(res.clone())
        }
    }

    /// 这里是get，但是有一个验证key
    pub fn get(&self, key: &str) -> Option<String> {
        let http = reqwest::blocking::Client::new();
        let res = http
            .get(self.url.as_str())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", key))
            .send()
            .ok()?
            .text()
            .ok()?;
        Some(res.clone())
    }

    /// 这里是获取默认文本或二进制文件的。
    /// 返回值为Vec<u8>
    pub fn get_default(&self) -> Option<Vec<u8>> {
        let http = reqwest::blocking::Client::new();
        let res = http
            .get(self.url.as_str())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .ok()?
            .bytes()
            .ok()?;
        Some(res.to_vec())
    }

    /// 以下为异步的网络请求。
    pub async fn post_async(&self, key: &str, that: bool) -> Option<String> {
        let http = reqwest::Client::new();
        if that {
            let res = http
                .post(self.url.as_str())
                .timeout(std::time::Duration::from_secs(100))
                .header(reqwest::header::USER_AGENT, USER_AGENT)
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .body(key.to_string())
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()?;
            Some(res.clone())
        } else {
            let res = http
                .post(self.url.as_str())
                .timeout(std::time::Duration::from_secs(100))
                .header(reqwest::header::USER_AGENT, USER_AGENT)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .header(reqwest::header::ACCEPT, "application/json")
                .body(key.to_string())
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()?;
            Some(res.clone())
        }
    }
    pub async fn get_async(&self, key: &str) -> Option<String> {
        let http = reqwest::Client::new();
        let res = http
            .get(self.url.as_str())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", key))
            .send()
            .await
            .ok()?
            .text()
            .await
            .ok()?;
        Some(res.clone())
    }
    pub async fn get_default_async(&self) -> Option<Vec<u8>> {
        let http = reqwest::Client::new();
        let res = http
            .get(self.url.as_str())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send()
            .await
            .ok()?
            .bytes()
            .await
            .ok()?;
        Some(res.to_vec())
    }
}
#[derive(Clone)]
pub struct AccountResult {
    name: String,
    uuid: String,
    access_token: String,
    refresh_token: String,
    client_token: String,
    base: String,
}
impl AccountResult {
    fn new() -> Self {
        Self {
            name: String::new(),
            uuid: String::new(),
            access_token: String::new(),
            refresh_token: String::new(),
            client_token: String::new(),
            base: String::new(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }
    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }
    pub fn get_refresh_token(&self) -> String {
        self.refresh_token.clone()
    }
    pub fn get_client_token(&self) -> String {
        self.client_token.clone()
    }
    pub fn get_base(&self) -> String {
        self.base.clone()
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_uuid(&mut self, uuid: &str) {
        self.uuid = uuid.to_string();
    }
    fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }
    fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = refresh_token.to_string();
    }
    fn set_client_token(&mut self, client_token: &str) {
        self.client_token = client_token.to_string();
    }
    fn set_base(&mut self, base: &str) {
        self.base = base.to_string();
    }
}
pub struct AccountLogin {
    key: String,
}
impl AccountLogin {
    /// 微软登录与外置登录均使用异步编写手法，各位可以使用tokio或futures运行。
    /// 该new函数需要传入一个client_id值，该值请自行查阅wiki.vg或者本仓库文档进行查阅。
    pub fn new_ms(client_id: &str) -> Self {
        Self {
            key: client_id.to_string(),
        }
    }

    /// 外置登录需要填入一个服务器地址
    /// 该服务器地址需要精确到api/yggdrasil/以便直接进行post、get。
    /// 记住，服务器地址末尾必须不能有/符号，否则一定会出错！
    /// 示例填入：https://littleskin.cn/api/yggdrasil
    pub fn new_tp(server: &str) -> Self {
        Self {
            key: server.to_string(),
        }
    }

    /// 这里是获取用户代码的，返回两个值（用户代码、设备代码），各位需要自行使用浏览器打开【https://microsoft.com/link 】进行登录。
    /// 如果获取到了网址，但是在解析JSON时失败了，会直接panic掉！因此你应该需要一个catch_unwind来包围住这个函数。
    /// 剩余的json里的4个值，分别是：
    /// verification_uri: https://www.microsoft.com/link
    /// expires_in: 900
    /// interval: 5
    /// message: 中文信息
    /// 各位可以自行赋值，因此无需返回。
    pub async fn get_user_code(&self) -> MMCLLResult<(String, String)> {
        const URL: &str =
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode?mkt=zh-CN";
        let k1 = format!(
            "client_id={}&scope=XboxLive.signin%20offline_access",
            self.key.clone()
        );
        let client = UrlMethod::new(URL);
        let res = client
            .post_async(k1.as_str(), true)
            .await
            .ok_or(MMCLLError::LoginCannotGetUserCode)?;
        let login = serde_json::from_str::<serde_json::Value>(res.as_str())
            .map_err(|_| MMCLLError::LoginCannotGetUserCode)?;
        let user_code = login["user_code"]
            .as_str()
            .ok_or(MMCLLError::LoginCannotGetUserCode)?;
        let device_code = login["device_code"]
            .as_str()
            .ok_or(MMCLLError::LoginCannotGetUserCode)?;
        Ok((user_code.to_string(), device_code.to_string()))
    }

    /// 登录的时候是刷新还是请求，这里是私有函数。
    async fn microsoft(
        &self,
        access_token: &str,
        refresh_token: &str,
    ) -> MMCLLResult<AccountResult> {
        const XBOX_LIVE: &str = "https://user.auth.xboxlive.com/user/authenticate";
        const XSTS_LIVE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
        const MC_LIVE: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";
        const VERIFY: &str = "https://api.minecraftservices.com/minecraft/profile";
        let mut result_account = AccountResult::new();
        let k2 = format!("{}{}{}", "{\"Properties\":{\"AuthMethod\":\"RPS\",\"SiteName\":\"user.auth.xboxlive.com\",\"RpsTicket\":\"d=", access_token, "\"},\"RelyingParty\":\"http://auth.xboxlive.com\",\"TokenType\":\"JWT\"}");
        let t2 = UrlMethod::new(XBOX_LIVE)
            .post_async(k2.as_str(), false)
            .await
            .ok_or(MMCLLError::LoginXboxLiveInvalid)?;
        let j2 = serde_json::from_str::<serde_json::Value>(t2.as_str()).map_err(|_| 21)?;
        let w2 = j2["Token"].as_str().ok_or(22)?;
        let uhs_xbox = j2["DisplayClaims"]["xui"][0]["uhs"].as_str().ok_or(23)?;
        let k3 = format!(
            "{}{}{}",
            "{\"Properties\":{\"SandboxId\":\"RETAIL\",\"UserTokens\":[\"",
            w2,
            "\"]},\"RelyingParty\":\"rp://api.minecraftservices.com/\",\"TokenType\":\"JWT\"}"
        );
        let t3 = UrlMethod::new(XSTS_LIVE)
            .post_async(k3.as_str(), false)
            .await
            .ok_or(MMCLLError::LoginXstsLiveInvalid)?;
        let j3 = serde_json::from_str::<serde_json::Value>(t3.as_str()).map_err(|_| 24)?;
        let w3 = j3["Token"].as_str();
        if w3.is_none() {
            let ww3 = j3["XErr"].as_i64().ok_or(25)?;
            return if ww3 == 2148916233 {
                Err(MMCLLError::LoginXstsNoXbox)
            } else if ww3 == 2148916235 {
                Err(MMCLLError::LoginXstsIllegal)
            } else if ww3 == 2148916236 || ww3 == 2148916237 {
                Err(MMCLLError::LoginXstsNoAdult)
            } else if ww3 == 2148916238 {
                Err(MMCLLError::LoginXstsUnder18)
            } else {
                Err(26.into())
            };
        }
        let w3 = w3.ok_or(27)?;
        let uhs_xsts = j3["DisplayClaims"]["xui"][0]["uhs"].as_str().ok_or(28)?;
        if uhs_xbox != uhs_xsts {
            return Err(MMCLLError::LoginXboxXstsUsercode);
        }
        let k4 = format!(
            "{}{}{}{}{}",
            "{\"identityToken\":\"XBL3.0 x=", uhs_xsts, ";", w3, "\"}"
        );
        let t4 = UrlMethod::new(MC_LIVE)
            .post_async(k4.as_str(), false)
            .await
            .ok_or(MMCLLError::LoginXboxLiveInvalid)?;
        let j4 = serde_json::from_str::<serde_json::Value>(t4.as_str()).map_err(|_| 29)?;
        let w4 = j4["access_token"].as_str().ok_or(30)?;
        let t5 = UrlMethod::new(VERIFY)
            .get_async(w4)
            .await
            .ok_or(MMCLLError::LoginMcInvalid)?;
        let j5 = serde_json::from_str::<serde_json::Value>(t5.as_str()).map_err(|_| 31)?;
        let name = j5["name"].as_str().ok_or(MMCLLError::LoginNoMinecraft)?;
        let uuid = j5["id"].as_str().ok_or(MMCLLError::LoginNoMinecraft)?;
        result_account.set_name(name);
        result_account.set_uuid(uuid);
        result_account.set_access_token(w4);
        result_account.set_refresh_token(refresh_token);
        Ok(result_account)
    }

    /// 公开函数，用于登录微软账号。需要提供一个device_code。
    pub async fn login_microsoft(&self, device_code: String) -> MMCLLResult<AccountResult> {
        const MS_LIVE: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
        let k1 = format!(
            "grant_type=urn:ietf:params:oauth:grant-type:device_code&client_id={}&device_code={}",
            self.key, device_code
        );
        let client = UrlMethod::new(MS_LIVE);
        let g1 = client.post_async(k1.as_str(), true).await.ok_or(2)?;
        let j1 = serde_json::from_str::<serde_json::Value>(g1.as_str()).map_err(|_| 3)?;
        let w1 = j1["access_token"].as_str();
        if let Some(e) = w1 {
            let r = j1["refresh_token"].as_str().ok_or(4)?;
            let a = self.microsoft(e, r).await?;
            Ok(a)
        } else {
            let e1 = j1["error_code"][0].as_i64().ok_or(5)?;
            if e1 == 70016 {
                Err(MMCLLError::LoginDeviceCodeInvalid)
            } else if e1 == 70020 {
                Err(MMCLLError::LoginTimeout)
            } else {
                Err(10.into())
            }
        }
    }

    /// 刷新微软账号，需要提供一个refresh_token。
    pub async fn refresh_microsoft(&self, refresh_token: String) -> MMCLLResult<AccountResult> {
        const MS_LIVE: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
        let k1 = format!(
            "grant_type=refresh_token&client_id={}&refresh_token={}",
            self.key, refresh_token
        );
        let client = UrlMethod::new(MS_LIVE);
        let g1 = client.post_async(k1.as_str(), true).await.ok_or(11)?;
        let j1 = serde_json::from_str::<serde_json::Value>(g1.as_str()).map_err(|_| 12)?;
        let w1 = j1["access_token"].as_str();
        if let Some(e) = w1 {
            let r = j1["refresh_token"].as_str().ok_or(13)?;
            let a = self.microsoft(e, r).await?;
            Ok(a)
        } else {
            let e1 = j1["error_code"][0].as_i64().ok_or(14)?;
            if e1 == 70011 {
                Err(MMCLLError::LoginRefreshTokenExpire)
            } else {
                Err(15.into())
            }
        }
    }

    /// 外置登录，输入账号和密码，如果有client_token，则也填写，否则填String::new()即可。
    pub async fn login_thirdparty(
        &self,
        username: String,
        password: String,
        client_token: String,
    ) -> MMCLLResult<Vec<AccountResult>> {
        use base64::Engine;
        let base = UrlMethod::new(self.key.as_str());
        let base = base
            .get_default_async()
            .await
            .ok_or(MMCLLError::LoginCannotGetMetadata)?;
        let base = String::from_utf8(base).map_err(|_| MMCLLError::LoginCannotGetMetadata)?;
        if base.is_empty() {
            return Err(MMCLLError::LoginCannotGetMetadata);
        }
        let base = base64::engine::general_purpose::STANDARD.encode(base.replace("\\/", "/"));
        let res = format!("{}/authserver/authenticate", self.key.clone());
        let k1: String;
        if client_token.is_empty() {
            k1 = format!(
                "{}{}{}{}{}",
                "{\"username\":\"",
                username,
                "\",\"password\":\"",
                password,
                "\",\"requestUser\":false,\"agent\":{\"name\":\"Minecraft\",\"version\":1}}"
            );
        } else {
            k1 = format!(
                "{}{}{}{}{}{}{}",
                "{\"username\":\"",
                username,
                "\",\"password\":\"",
                password,
                "\",\"client_token\":\"",
                client_token,
                "\",\"requestUser\":false,\"agent\":{\"name\":\"Minecraft\",\"version\":1}}"
            );
        }
        let w1 = UrlMethod::new(res.as_str());
        let t1 = w1
            .post_async(k1.as_str(), false)
            .await
            .ok_or(MMCLLError::LoginUsernameOrPassword)?;
        let j1 = serde_json::from_str::<serde_json::Value>(t1.as_str()).map_err(|_| 46)?;
        let a1 = j1["accessToken"].as_str();
        if a1.is_none() {
            let err = j1["errorMessage"].as_str().ok_or(47)?;
            return if err.contains("invalid")
                && err.contains("username")
                && err.contains("password")
            {
                Err(MMCLLError::LoginUsernameOrPassword)
            } else {
                Err(48.into())
            };
        }
        let a1 = a1.ok_or(49)?;
        let r1 = j1
            .get("availableProfiles")
            .ok_or(50)?
            .as_array()
            .ok_or(51)?;
        let mut v: Vec<AccountResult> = Vec::new();
        for i in r1.iter() {
            let mut ar = AccountResult::new();
            let name = i["name"].as_str().ok_or(52)?;
            let id = i["id"].as_str().ok_or(53)?;
            ar.set_name(name);
            ar.set_uuid(id);
            ar.set_access_token(a1);
            ar.set_client_token(client_token.as_str());
            ar.set_base(base.as_str());
            v.push(ar);
        }
        Ok(v)
    }

    /// 刷新外置登录，填入access_token，如果有client_token则填，否则填String::new()即可。
    pub async fn refresh_thirdparty(
        &self,
        access_token: String,
        client_token: String,
    ) -> MMCLLResult<AccountResult> {
        let res = format!("{}/authserver/refresh", self.key);
        let k1: String;
        if client_token.is_empty() {
            k1 = format!(
                "{}{}{}",
                "{\"accessToken\":\"",
                access_token.clone(),
                "\",\"requestUser\":false}"
            );
        } else {
            k1 = format!(
                "{}{}{}{}{}",
                "{\"accessToken\":\"",
                access_token.clone(),
                "\",\"client_token\":\"",
                client_token,
                "\",\"requestUser\":false}"
            );
        }
        let t1 = UrlMethod::new(res.as_str());
        let t1 = t1
            .post_async(k1.as_str(), false)
            .await
            .ok_or(MMCLLError::LoginAccessTokenExpire)?;
        let j1 = serde_json::from_str::<serde_json::Value>(t1.as_str()).map_err(|_| 54)?;
        let ac = j1["accessToken"].as_str();
        if ac.is_none() {
            let err = j1["errorMessage"].as_str().ok_or(55)?;
            return if err.contains("invalid") && err.contains("token") {
                Err(MMCLLError::LoginAccessTokenExpire)
            } else {
                Err(56.into())
            };
        }
        let ac = ac.ok_or(57)?;
        let mut res_acc = AccountResult::new();
        res_acc.set_access_token(ac);
        res_acc.set_client_token(client_token.as_str());
        Ok(res_acc)
    }
}