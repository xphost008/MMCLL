/**
 * 欢迎来到Tank Launcher Module! 
 * 本模块使用MIT协议进行开源！各位可以随意使用本模块的函数。
 * 本模块暂未发布至crates.io，因为我不想发！！
 */
/**
 * 部分常量值，在程序的任意位置均可直接调用。
 */
pub mod some_const {
    //版本号
    pub const TLM_VERSION: &str = "0.0.1-Alpha";
    pub const OK: i32 = 0;  //完成
    pub const ERR_UNKNOWN_ERROR: i32 = 1;  //未知错误
    pub const ERR_LAUNCH_ACCOUNT_USERNAME: i32 = -1;  //账号名称格式错误
    pub const ERR_LAUNCH_ACCOUNT_USERUUID: i32 = -2;  //账号UUID格式错误
    pub const ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN: i32 = -3;  //账号AccessToken错误
    pub const ERR_LAUNCH_ACCOUNT_NO_LEGAL: i32 = -4;  //账号未购买正版
    pub const ERR_LAUNCH_ACCOUNT_THIRDPARTY_ACCESS_TOKEN_OR_URL: i32 = -5;  //账号第三方的AccessToken或者URL错误。
    pub const ERR_LAUNCH_ACCOUNT_THIRDPARTY_BASE: i32 = -6;  //账号base64编码错误
    pub const ERR_LAUNCH_JAVA_PATH: i32 = -7;  //Java路径错误（文件未找到）
    pub const ERR_LAUNCH_ROOT_PATH: i32 = -8;  //游戏根路径错误（文件夹未找到）
    pub const ERR_LAUNCH_VERSION_PATH: i32 = -9;  //游戏版本路径错误（文件夹未找到）
    pub const ERR_LAUNCH_GAME_PATH: i32 = -10;  //游戏实际路径错误（文件夹未找到）
    pub const ERR_LAUNCH_WIDTH: i32 = -11;  //窗口宽度错误（小于854或大于屏幕宽度）
    pub const ERR_LAUNCH_HEIGHT: i32 = -12;  //窗口高度错误（小于480或大于屏幕高度）
    pub const ERR_LAUNCH_MIN_MEMORY: i32 = -13;  //最小内存错误（小于256或大于1024）
    pub const ERR_LAUNCH_MAX_MEMORY: i32 = -13;  //最大内存错误（小于1024或大于系统内存）
    pub const ERR_LAUNCH_CUSTOM_INFO: i32 = -14;  //自定义信息错误（未填写，必须要个默认值！）
}
/**
 * 部分全局变量值。在需要的时候可以使用unsafe包裹住该变量以便使用，赋值和引用均可。但是你需要为你赋过的值负责！。
 */
pub mod some_var {
    pub static mut DOWNLOAD_SOURCE: i32 = 1;  //下载源：目前仅支持两个数字，1：官方、2：BMCLAPI
    pub static mut MC_ROOT_JSON: String = String::new();  //mc的元数据（可以自己赋值也可以由类库帮忙赋值！）仅能赋值元数据值，如果赋上了别的值，后果自负！
    pub static mut AUTHLIB_PATH: String = String::new();  //设置第三方登录的模块jar文件。在使用第三方登录的时候一定要设置该参数！
    // pub static mut AUTHLIB_URL: String = String::new();
}
pub mod account_mod {
    pub struct UrlMethod {
        url: String,
    }
    impl UrlMethod {
        pub fn new(url: &str) -> Self {
            Self {
                url: url.to_string()
            }
        }
        pub fn post(&self, key: &str, that: bool) -> Option<String>{
            let http = reqwest::blocking::Client::new();
            return if that {
                let head = "application/x-www-form-urlencoded;charset=utf-8";
                let res = http
                    .post(self.url.as_str())
                    .timeout(std::time::Duration::from_secs(100))
                    .header(reqwest::header::CONTENT_TYPE, head)
                    .body(key.to_string())
                    .send();
                if let Err(_) = res { return None }
                let res = res.unwrap().text();
                if let Err(_) = res { return None }
                Some(res.unwrap().clone())
            } else {
                let head = "application/json";
                let res = http
                    .post(self.url.as_str())
                    .timeout(std::time::Duration::from_secs(100))
                    .header(reqwest::header::CONTENT_TYPE, head)
                    .header(reqwest::header::ACCEPT, head)
                    .body(key.to_string())
                    .send();
                if let Err(_) = res { return None }
                let res = res.unwrap().text();
                if let Err(_) = res { return None }
                Some(res.unwrap().clone())
            }
        }
        pub fn get(&self, key: &str) -> Option<String>{
            let http = reqwest::blocking::Client::new();
            let res = http
                .get(self.url.as_str())
                .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", key))
                .send();
            if let Err(_) = res { return None }
            let res = res.unwrap().text();
            if let Err(_) = res { return None }
            return Some(res.unwrap().clone());
        }
        pub fn get_default(&self) -> Option<String>{
            let res = reqwest::blocking::get(self.url.as_str());
            if let Err(_) = res { return None }
            let res = res.unwrap().text();
            if let Err(_) = res { return None }
            return Some(res.unwrap().clone());
        }
        pub fn get_response(&self) -> Option<reqwest::blocking::Response> {
            let http = reqwest::blocking::Client::new();
            let res = http.get(self.url.as_str()).send();
            if let Err(_) = res { return None; }
            Some(res.unwrap())
        }
    }
    // /**
    //  * 账号返回类，每次登录账号都会返回一个该类。
    //  */
    // struct AccountResult{
    //     user_name: String,
    //     user_uuid: String,
    //     user_access_token: String,
    //     user_refresh_token: String,
    //     user_client_token: String,
    // }
    // impl AccountResult{
    //     fn new() -> Self{
    //         Self{
    //             user_name: String::new(),
    //             user_uuid: String::new(),
    //             user_access_token: String::new(),
    //             user_refresh_token: String::new(),
    //             user_client_token: String::new(),
    //         }
    //     }
    //     fn set_user_name(&mut self, user_name: String){
    //         self.user_name = user_name.clone();
    //     }
    //     fn set_user_uuid(&mut self, user_uuid: String){
    //         self.user_uuid = user_uuid.clone();
    //     }
    //     fn set_user_access_token(&mut self, user_access_token: String){
    //         self.user_access_token = user_access_token.clone();
    //     }
    //     fn set_user_refresh_token(&mut self, user_refresh_token: String){
    //         self.user_refresh_token = user_refresh_token.clone();
    //     }
    //     fn set_user_client_token(&mut self, user_client_token: String){
    //         self.user_client_token = user_client_token.clone();
    //     }
    //     pub fn get_user_name(&self) -> String{
    //         return self.user_name.clone();
    //     }
    //     pub fn get_user_uuid(&self) -> String{
    //         return self.user_uuid.clone();
    //     }
    //     pub fn get_user_access_token(&self) -> String{
    //         return self.user_access_token.clone();
    //     }
    //     pub fn get_user_refresh_token(&self) -> String{
    //         return self.user_refresh_token.clone();
    //     }
    //     pub fn get_user_client_token(&self) -> String{
    //         return self.user_client_token.clone();
    //     }
    // }
    // /**
    //  * 该impl才是账号登录类，你可以进入这里随时进行账号登录！
    //  */
    // pub struct AccountLogin{}
    // impl AccountLogin{

    // }
}
/**
 * 许多在启动时可能需要用到的静态函数。（无需初始化，仅需直接调用。）
 */
pub mod main_mod {
    /**
     * 从一个path获取外部文件。
     */
    pub fn get_file(path: &str) -> Option<String> {
        let p = std::path::Path::new(path);
        let f = std::fs::read_to_string(p);
        return if let Ok(e) = f { Some(e) } else { None }
    }
    /**
     * 将内容写出到文件
     */
    pub fn set_file(path: &str, content: String) -> bool {
        let p = std::path::Path::new(path);
        if p.is_dir() { return false; }
        let parent = p.parent();
        if let None = parent { return false };
        let parent = parent.unwrap();
        if !parent.exists() || parent.exists() && parent.is_file() {
            let q = std::fs::create_dir_all(parent);
            if let Err(_) = q { return false; }
        }
        let f = std::fs::File::create(p);
        if let Err(_) = f { return false; }
        let mut f = f.unwrap();
        use std::io::Write;
        let r = f.write_all(content.as_bytes());
        if let Err(_) = r { return false; }
        true
    }
    /**
     * 删除文件
     */
    pub fn delete_file(path: &str) -> bool {
        let p = std::path::Path::new(path);
        if !p.exists() || p.exists() && p.is_dir() { return false; }
        let res = std::fs::remove_file(p);
        return if let Err(_) = res { false } else { true }
    }
    /**
     * 获取某一个文件的SHA1值
     */
    pub fn get_sha1(path: &str) -> Option<String> {
        let file = std::fs::File::open(path);
        if let Err(e) = file {
            println!("{:?}", e);
            return None;
        }
        let mut file = file.unwrap();
        use crypto::digest::Digest;
        use std::io::Read;
        let mut sha1 = crypto::sha1::Sha1::new();
        let mut buffer = [0; 1024];
        loop {
            let n = file.read(&mut buffer);
            if let Err(_) = n { return None; }
            let n = n.unwrap();
            if n == 0 { break; }
            sha1.input(&buffer[..n]);
        }
        let hash = sha1.result_str();
        Some(hash)
    }
    /**
     * 该函数目前仅适用于在离线登录时根据用户名生成一个唯一的UUID。
     */
    pub fn generate_bukkit_uuid(name: &str) -> String{
        use crypto::digest::Digest;
        let mut md5 = crypto::md5::Md5::new();
        md5.input_str(format!("OfflinePlayer:{}", name).as_str());
        let res_str = md5.result_str();
        let res_hex = hex::decode(res_str.as_str());
        if let Err(_) = res_hex { return String::new(); }
        let mut res_hex = res_hex.unwrap();
        res_hex[6] = (res_hex[6] & 0x0f) | 0x30;
        res_hex[8] = (res_hex[8] & 0x3f) | 0x80;
        return hex::encode(res_hex);
    }
    /**
     * 该函数目前仅适用于在初始化第三方登录时对该皮肤站元数据进行base64编码。
     */
    pub fn generate_thirdparty_metadata_base64(url: &str) -> String {
        use base64::Engine;
        let um = super::account_mod::UrlMethod::new(url);
        let metadata = um.get_default();
        if let None = metadata { return String::new(); }
        let base = base64::engine::general_purpose::STANDARD.encode(metadata.unwrap().replace("\\/", "/"));
        return base;
    }
    /**
     * 截取文件名
     */
    pub fn extract_file_name(file: &str) -> String {
        let rf = file.rfind("\\");
        if let None = rf { return String::new(); }
        let rf = rf.unwrap();
        let versub = file.get((rf + 1)..file.len());
        if let None = versub { return String::new(); }
        let versub = versub.unwrap();
        return versub.to_string();
    }
    /**
     * 获取exe的位数（32位或64位）
     */
    pub fn get_file_bit(file: String) -> Option<bool> {
        let path = std::path::Path::new(file.as_str());
        if !path.exists() || path.exists() && path.is_dir() { return None; }
        let data = pelite::FileMap::open(path);
        if let Err(_) = data { return None; }
        let data = data.unwrap();
        let file = pelite::PeFile::from_bytes(&data);
        if let Err(_) = file { return None; }
        let file = file.unwrap();
        match file {
            pelite::Wrap::T64(_) => { return Some(true); }
            pelite::Wrap::T32(_) => { return Some(false); }
        }
    }
    /**
     * 获取exe文件的版本号
     */
    pub fn get_file_version(file: String) -> Option<String> {
        let path = std::path::Path::new(file.as_str());
        if !path.exists() || path.exists() && path.is_dir() { return None; }
        let data = pelite::FileMap::open(path);
        if let Err(_) = data { return None; }
        let data = data.unwrap();
        let file = pelite::PeFile::from_bytes(&data);
        if let Err(_) = file { return None; }
        let file = file.unwrap();
        let file = file.resources();
        if let Err(_) = file { return None; }
        let fixed_version = file.unwrap().version_info();
        if let Err(_) = fixed_version { return None; }
        let fixed_version = fixed_version.unwrap().fixed();
        if let None = fixed_version { return None; }
        let fixed_version = fixed_version.unwrap();
        return Some(format!("{}.{}.{}.{}", 
                        fixed_version.dwFileVersion.Major.to_string(),
                        fixed_version.dwFileVersion.Minor.to_string(),
                        fixed_version.dwFileVersion.Build.to_string(),
                        fixed_version.dwFileVersion.Patch.to_string()
                    ));
    }
}
/**
 * 专注于启动游戏的模块，所有启动游戏的函数都可以在这里面找到！
 */
pub mod launcher_mod {
    /**
     * 此方法用于将json libraries里的name值转换为path。
     */
    pub fn convert_name_to_path(name: String) -> Option<String> {
        let mut name = name.clone();
        let suffix: String;
        if name.contains("@") {
            let rf = name.rfind("@");
            if let None = rf {
                return None;
            }
            let rf = rf.unwrap();
            suffix = name[(rf + 1)..name.len()].to_string();
            name = name[0..rf].to_string();
        }else{
            suffix = String::from("jar")
        }
        let spl: Vec<&str> = name.split(":").collect();
        return if spl.len() == 4 {
            Some(format!("{}\\{}\\{}\\{}-{}-{}.{}", spl[0].replace(".", "\\"), spl[1], spl[2], spl[1], spl[2], spl[3], suffix))
        } else if spl.len() == 3 {
            Some(format!("{}\\{}\\{}\\{}-{}.{}", spl[0].replace(".", "\\"), spl[1], spl[2], spl[1], spl[2], suffix))
        } else {
            None
        }
    }
    /**
     * 根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
     * 会按照clientVersion、patches->game|version、metadata->versions->releaseTime、id值进行找。
     * 如果连最终的id值也不符合，则返回必定返回None！
     * 但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
     * 所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。当然也可以用catch_unwind来捕捉panic也就对了！
     */
    pub fn get_mc_vanilla_version(json: String) -> Option<String> {
        let root = serde_json::from_str::<serde_json::Value>(json.as_str());
        if let Err(_) = root { return None; }
        let binding = root.unwrap();
        let root = binding.as_object();
        if let None = root { return None; }
        let root = root.unwrap();
        let cid = root.get("clientVersion");
        if let None = cid { return None; }
        if let Some(e) = cid.unwrap().as_str() {
            if !e.is_empty() {
                return Some(e.to_string());
            }
        }
        let patch = root.get("patches");
        if let None = patch { return None; }
        if let Some(e) = patch.unwrap().as_array() {
            for i in e.into_iter() {
                let pat = i.as_object();
                if let None = pat { continue; }
                let pat = pat.unwrap();
                let id = pat.get("id");
                if let None = id { continue; }
                let id = id.unwrap().as_str();
                if let None = id { continue; }
                let id = id.unwrap();
                if id.eq("game") {
                    let mcid = pat.get("version");
                    if let None = mcid { continue; }
                    if let Some(f) = mcid.unwrap().as_str() {
                        if !f.is_empty(){
                            return Some(f.to_string());
                        }
                    }
                }
            }
        }
        if let Some(w) = root.get("releaseTime") {
            if let Some(real_time) = w.as_str() {
                unsafe {
                    let v = match super::some_var::DOWNLOAD_SOURCE {
                        2 => { "https://bmclapi2.bangbang93.com/mc/game/version_manifest.json" }
                        _ => { "https://piston-meta.mojang.com/mc/game/version_manifest.json" }
                    };
                    if super::some_var::MC_ROOT_JSON.is_empty() {
                        let url = super::account_mod::UrlMethod::new(v);
                        if let Some(e) = url.get_default() {
                            super::some_var::MC_ROOT_JSON = e.clone();
                        }
                    }
                    if !super::some_var::MC_ROOT_JSON.is_empty(){
                        if let Ok(e) = serde_json::from_str::<serde_json::Value>(super::some_var::MC_ROOT_JSON.as_str()) {
                            if let Some(f) = e.as_object() {
                                if let Some(v) = f.get("versions") {
                                    if let Some(g) = v.as_array() {
                                        for h in g.into_iter() {
                                            if let Some(i) = h.as_object() {
                                                if let Some(r) = i.get("releaseTime") {
                                                    if let Some(j) = r.as_str() {
                                                        if j.eq(real_time) {
                                                            if let Some(d) = i.get("id") {
                                                                if let Some(k) = d.as_str() {
                                                                    return Some(k.to_string());
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(f) = root.get("id") {
            if let Some(e) = f.as_str() {
                if !e.is_empty() { return Some(e.to_string()); }
            }
        }
        None
    }
    /**
     * 解压任意文件到路径。
     * 该函数并不会返回进度值，各位可以自行查看该函数并实现自己的回显进度的zip解压。
     * 该函数目前仅会返回bool值，如果解压成功则返回true，反之如果里面出现任何错误，则直接返回false。
     */
    pub fn unzip(zipfile: String, extfile: String) -> bool {
        let zip_path = std::path::Path::new(zipfile.as_str());
        let ext_path = std::path::Path::new(extfile.as_str());
        if !zip_path.exists() || (zip_path.exists() && !zip_path.is_file()) { return false; }
        if !zip_path.exists() || (zip_path.exists() && zip_path.is_file()) {
            let cd = std::fs::create_dir_all(ext_path);
            if let Err(_) = cd { return false; }
        }
        let zip_file = std::fs::File::open(zip_path);
        if let Err(_) = zip_file { return false; }
        let zip_file = zip_file.unwrap();
        let zip_ext = zip::ZipArchive::new(zip_file);
        if let Err(_) = zip_ext { return false; }
        let mut zip_ext = zip_ext.unwrap();
        for i in 0..zip_ext.len() {
            let f = zip_ext.by_index(i);
            if let Err(_) = f { continue; }
            let mut f = f.unwrap();
            if f.is_dir() {
                let ext_dir = ext_path.join(std::path::Path::new(&f.name().replace("\\", "")));
                let cd = std::fs::create_dir_all(ext_dir);
                if let Err(_) = cd { continue; }
            } else {
                let file_path = ext_path.join(std::path::Path::new(f.name()));
                let ext_file = if !file_path.exists() {
                    std::fs::File::create(file_path)
                } else {
                    std::fs::File::open(file_path)
                };
                if let Err(_) = ext_file { continue; }
                let mut ext_file = ext_file.unwrap();
                let res = std::io::copy(&mut f, &mut ext_file);
                if let Err(_) = res { continue; }
            }
        }
        true
    }
    /**
     * 删除文件夹中的所有文件，但是保留后缀为suffix的值。该函数用于解压natives时需要删掉除了dll以外的所有文件。
     */
    pub fn delete_file_keep(dir_path: String, suffix: &str) -> bool {
        if dir_path.is_empty() { return false; }
        if suffix.is_empty() || suffix.eq(".") { return false; }
        if let None = dir_path.find("\\"){ return false; }
        let suffix = &suffix[1..suffix.len()];
        let dir = walkdir::WalkDir::new(dir_path.as_str());
        for i in dir.into_iter().filter_map(|e| e.ok()) {
            let path = i.path();
            if path.exists() {
                if path.is_dir() { continue; }
                let path_ext = path.extension();
                if let None = path_ext { continue; }
                let path_ext = path_ext.unwrap();
                if !path_ext.eq(suffix) {
                    let cd = std::fs::remove_file(path);
                    if let Err(_) = cd { continue; }
                }
            }
        } 
        true
    }
    /**
     * 任何类都可以用的数字转换！
     * 可以将字符串中的数字提取出来，或者是字符串中的非数字【字符】提取出来！
     */
    pub fn extract_number(ext: String, isnum: bool) -> String {
        let mut temp = String::new();
        if ext.len() == 0 { return String::new(); }
        for i in ext.chars() {
            if isnum {
                if i.is_numeric() { temp.push(i); }
            }else{
                if !i.is_numeric() { temp.push(i); }
            }
        }
        return temp.clone();
    }
    /**
     * 根据找到的json中的inheritsFrom或者jar值，准确的找到另一个有关该原版的版本文件夹。
     */
    pub fn get_mc_inherits_from(version_path: String, ioj: &str) -> Option<String> {
        let path = std::path::Path::new(version_path.as_str());
        if path.exists() && path.is_dir() {
            let real_path = get_mc_real_path(version_path.clone(), ".json");
            if let None = real_path { return None; }
            let real_path = real_path.unwrap();
            let real_file = super::main_mod::get_file(real_path.as_str());
            if let None = real_file { return None; }
            let real_file = real_file.unwrap();
            let root = serde_json::from_str::<serde_json::Value>(real_file.as_str());
            if let Err(_) = root { return None; }
            let root = root.unwrap();
            let root = root.as_object();
            if let None = root { return None; }
            let root = root.unwrap();
            if let Some(f) = root.get(ioj) {
                if let Some(e) = f.as_str() {
                    if e.is_empty() { return Some(version_path.clone()) }
                    let parent_path = path.parent();
                    if let None = parent_path { return None; }
                    let parent_path = parent_path.unwrap();
                    let dir = walkdir::WalkDir::new(parent_path).min_depth(1).max_depth(1);
                    for i in dir.into_iter().filter_map(|e| e.ok()) {
                        let pa = i.path();
                        if pa.is_file() { continue; }
                        let ps = pa.display().to_string();
                        let version_json = get_mc_real_path(ps.clone(), ".json");
                        if let None = version_json { continue; }
                        let version_json = version_json.unwrap();
                        let json_content = super::main_mod::get_file(version_json.as_str());
                        if let None = json_content { continue; }
                        let json_content = json_content.unwrap();
                        let vanilla_version = get_mc_vanilla_version(json_content);
                        if let None = vanilla_version { continue; }
                        let vanilla_version = vanilla_version.unwrap();
                        if vanilla_version.eq(e) { return Some(ps.clone()); }
                    }
                }else{ return Some(version_path.clone()); }
            }else{ return Some(version_path.clone()); }
        }
        None
    }
    /**
     * 从inheritsFrom键中找到的json当作原版json，并拼接上inheritsFrom键所在的json，将其合并成一个json！
     */
    pub fn replace_mc_inherits_from(mut raw_json: String, mut ins_json: String) -> Option<String> {
        fn return_some(k: &mut serde_json::Map<String, serde_json::Value>) -> Option<String> {
            let s = serde_json::to_string(&k);
            if let Err(_) = s { return None; }
            let s = s.unwrap();
            return Some(s);
        }
        if raw_json.is_empty() || ins_json.is_empty() { return None; }
        raw_json = raw_json.replace("\\", "");
        ins_json = ins_json.replace("\\", "");
        if raw_json.eq(ins_json.as_str()) { return Some(raw_json); }
        let rt_raw = serde_json::from_str::<serde_json::Value>(raw_json.as_str());
        if let Err(_) = rt_raw { return None; }
        let rt_raw = rt_raw.unwrap();
        let rt_raw = rt_raw.as_object();
        if let None = rt_raw { return None; }
        let rt_raw = rt_raw.unwrap();
        let rt_ins = serde_json::from_str::<serde_json::Value>(ins_json.as_str());
        if let Err(_) = rt_ins { return None; }
        let mut rt_ins = rt_ins.unwrap();
        let rt_ins = rt_ins.as_object_mut();
        if let None = rt_ins { return None; }
        let rt_ins = rt_ins.unwrap();
        let mc = rt_raw.get("mainClass");
        if let None = mc { return None; }
        let mc = mc.unwrap().as_str();
        if let None = mc { return None; }
        let mc = mc.unwrap();
        rt_ins.remove("mainClass");
        rt_ins.insert("mainClass".to_string(), serde_json::Value::String(mc.to_string()));
        let id = rt_raw.get("id");
        if let None = id { return None; }
        let id = id.unwrap().as_str();
        if let None = id { return None; }
        let id = id.unwrap();
        rt_ins.remove("id");
        rt_ins.insert("id".to_string(), serde_json::Value::String(id.to_string()));
        let raw_lib = rt_raw.get("libraries");
        if let None = raw_lib { return None; }
        if let Some(e) = raw_lib.unwrap().as_array() {
            for i in e.into_iter() {
                if let Some(f) = i.as_object() {
                    if let Some(h) = rt_ins.get_mut("libraries") {
                        if let Some(g) = h.as_array_mut() {
                            g.push(serde_json::Value::Object(f.clone()));
                        }
                    }
                }
            }
        }
        if let Some(r1) = rt_raw.get("arguments") {
            if let Some(r2) = r1.get("jvm") {
                if let Some(e) = r2.as_array() {
                    for i in e.into_iter() {
                        if let Some(i1) = rt_ins.get_mut("arguments") {
                            if let Some(i2) = i1.get_mut("jvm") {
                                if let Some(f) = i2.as_array_mut() {
                                    f.push(i.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(r1) = rt_raw.get("arguments") {
            if let Some(r2) = r1.get("game") {
                if let Some(e) = r2.as_array() {
                    for i in e.into_iter() {
                        if let Some(i1) = rt_ins.get_mut("arguments") {
                            if let Some(i2) = i1.get_mut("game") {
                                if let Some(f) = i2.as_array_mut() {
                                    f.push(i.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(m) = rt_raw.get("minecraftArguments") {
            if let Some(e) = m.as_str() {
                rt_ins.remove("minecraftArguments");
                rt_ins.insert("minecraftArguments".to_string(), serde_json::Value::String(e.to_string()));
            }
        }
        return return_some(rt_ins); 
    }
    /**
     * 从一个文件夹中根据suffix获取一个准确的文件。
     * 其中当suffix为“.json”的时候逻辑可能会略有不同，请不要在意！
     * suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
     * 目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回None
     */
    pub fn get_mc_real_path(version_path: String, suffix: &str) -> Option<String> {
        let path = std::path::Path::new(version_path.as_str());
        if path.exists() && path.is_dir() {
            let dir = walkdir::WalkDir::new(version_path.clone()).min_depth(1).max_depth(1);
            for i in dir.into_iter().filter_map(|e| e.ok()) {
                let pa = i.path();
                if pa.is_dir() { continue; }
                let ps = pa.display().to_string();
                if ps.contains(suffix) {
                    return if suffix.eq(".json") {
                        let file_content = super::main_mod::get_file(ps.as_str());
                        if let None = file_content { continue; }
                        let file_content = file_content.unwrap();
                        let root = serde_json::from_str::<serde_json::Value>(file_content.as_str());
                        if let Err(_) = root { continue; }
                        let root = root.unwrap();
                        let root = root.as_object();
                        if let None = root { continue; }
                        let root = root.unwrap();
                        let libr = root.get("libraries");
                        if let None = libr { continue; }
                        let libr = libr.unwrap().is_array();
                        if !libr { continue; }
                        let mics = root.get("mainClass");
                        if let None = mics { continue; }
                        let mics = mics.unwrap().as_str();
                        if let None = mics { continue; }
                        let idid = root.get("id");
                        if let None = idid { continue; }
                        let idid = idid.unwrap().as_str();
                        if let None = idid { continue; }
                        Some(ps)
                    } else {
                        Some(ps)
                    };
                }else if !suffix.contains(".") {
                    let sha = super::main_mod::get_sha1(ps.as_str());
                    if let None = sha { continue; }
                    let sha = sha.unwrap();
                    if sha.eq(suffix) {
                        return Some(ps)
                    }
                }
            }
        }
        return None;
    }
    /**
     * 判断参数，以及拼接参数！适用于在整合参数时。
     */
    pub fn judge_arguments(args_json: String, key: &str) -> Option<Vec<String>>{
        let root = serde_json::from_str::<serde_json::Value>(args_json.as_str());
        if let Err(_) = root { return None; }
        let root = root.unwrap();
        let argu = root.get("arguments");
        if let None = argu { return None; }
        let argu = argu.unwrap().get(key);
        if let None = argu { return None; }
        let argu = argu.unwrap().as_array();
        if let None = argu { return None };
        let argu = argu.unwrap();
        let mut res: Vec<String> = Vec::new();
        for i in argu.into_iter() {
            let i_str = serde_json::to_string(i);
            if let Err(_) = i_str { continue; }
            let i_str = i_str.unwrap();
            if i_str.contains("rules") { continue; }
            let i_str = i.as_str();
            if let None = i_str { continue; }
            let i_str = i_str.unwrap().replace(" ", "");
            res.push(i_str.clone());
        }
        Some(res.clone())
    }

    /**
     * 单纯只是一个判断版本json里的libraries中，有rules的类库，是否allow在windows上。
     * 需要填入一个serde_json的对象Map值！而且该对象必须已经从rules中取了出来！
     */
    pub fn judge_mc_rules(root: &serde_json::Map<String, serde_json::Value>) -> bool {
        let rules = root.get("rules");
        if let None = rules { return true; }
        let rules = rules.unwrap().as_array();
        if let None = rules { return true; }
        let rules = rules.unwrap();
        for i in rules.into_iter() {
            let rule_1 = i.as_object();
            if let None = rule_1 { continue; }
            let rule_1 = rule_1.unwrap();
            let action = rule_1.get("action");
            if let None = action { continue; }
            let action = action.unwrap().as_str();
            if let None = action { continue; }
            let action = action.unwrap();
            if action.eq("allow") {
                let rule_2 = rule_1.get("os");
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap().get("name");
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap().as_str();
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap();
                if !rule_2.eq("windows") { return false; }
            }else if action.eq("disallow") {
                let rule_2 = rule_1.get("os");
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap().get("name");
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap().as_str();
                if let None = rule_2 { continue; }
                let rule_2 = rule_2.unwrap();
                if rule_2.eq("windows") { return false; }
            }
        }
        return true;
    }
    /**
     * 获取MC类库（GetCPLibraries）
     */
    pub fn get_mc_libs(raw_json: String, root_path: &str, version_path: &str) -> Option<String> {
        let mut res = String::new();
        let mut raw_list: Vec<String> = Vec::new();
        let mut no_list: Vec<String> = Vec::new();
        let mut no_low: Vec<String> = Vec::new();
        let mut temp_list: Vec<String> = Vec::new();
        let mut no_opt: Vec<String> = Vec::new();
        let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str());
        if let Err(_) = root { return None; }
        let root = root.unwrap();
        let root = root.as_object();
        if let None = root{ return None; }
        let root = root.unwrap();
        let json_lib = root.get("libraries");
        if let None = json_lib { return None; }
        let json_lib = json_lib.unwrap().as_array();
        if let None = json_lib { return None; } 
        let json_lib = json_lib.unwrap();
        for i in json_lib.into_iter() {
            let lib_root = i.as_object();
            if let None = lib_root { continue; }
            let lib_root = lib_root.unwrap();
            let name = lib_root.get("name");
            if let None = name { continue; }
            let name = name.unwrap().as_str();
            if let None = name { continue; }
            let name = name.unwrap();
            let expect_rule = judge_mc_rules(lib_root);
            let mut expect_native = true;
            let mut expect_downloads = true;
            if let Some(e) = lib_root.get("natives") {
                if let Some(_) = e.as_object() {
                    expect_native = false;
                }
            }
            if let Some(e) = lib_root.get("downloads") {
                if let Some(f) = e.get("classifiers") {
                    if let Some(_) = f.as_object() {
                        expect_downloads = false;
                        if let Some(g) = e.get("artifact") {
                            if let Some(_) = g.as_object() {
                                expect_downloads = true;
                            }
                        }
                    }
                }
            }
            if expect_rule && expect_native && expect_downloads { raw_list.push(name.to_string()) }
        }
        for i in raw_list.into_iter() {
            if !no_list.contains(&i) {
                no_list.push(i);
            }
        }
        for i in no_list.into_iter() {
            let nocom = i
                                .replace(".", "")
                                .replace(":", "")
                                .replace("-", "")
                                .replace("/", "")
                                .replace("@jar", "")
                                .replace("@zip", "")
                                .replace("@", "");
            let nonum = extract_number(nocom.clone(), false);
            let noword = extract_number(nocom.clone(), true);
            let toint = noword.parse::<u64>();
            if let Err(_) = toint { continue; }
            let toint = toint.unwrap();
            if !temp_list.contains(&nonum) {
                temp_list.push(nonum);
                no_low.push(i);
            } else {
                let temp_1 = temp_list.iter().position(|x| x == &nonum);
                if let None = temp_1 { continue; }
                let temp_1 = temp_1.unwrap();
                let temp_2 = no_low.get(temp_1);
                if let None = temp_2 { continue; }
                let temp_2 = extract_number(temp_2.unwrap().to_string(), true);
                let temp_3 = temp_2.parse::<u64>();
                if let Err(_) = temp_3 { continue; }
                let temp_3 = temp_3.unwrap();
                if temp_3 < toint {
                    no_low.remove(temp_1);
                    no_low.insert(temp_1, i);
                }
            }
        }
        //fuck you！ optifine!
        let mut temp = String::new();
        for i in no_low.into_iter(){
            if i.contains("optifine") {
                temp = i.clone();
                continue;
            }
            no_opt.push(i.clone());
        }
        if !temp.is_empty() {
            no_opt.push(temp.clone());
        }
        //end you! optifine!
        for i in no_opt.into_iter() {
            if let Some(e) = convert_name_to_path(i) {
                res.push_str(format!("{}\\libraries\\{}{}", root_path, e, "${classpath_separator}").as_str());
            }
        }
        let inh = get_mc_inherits_from(version_path.to_string(), "jar");
        if let None = inh { return None; }
        let mut inh = inh.unwrap();
        if inh.eq(version_path) {
            let inhj = get_mc_inherits_from(version_path.to_string(), "inheritsFrom");
            if let None = inhj { return None; }
            inh = inhj.unwrap().clone();
        }
        let sha = root.get("downloads");
        if let None = sha { return None; }
        let sha = sha.unwrap().get("client");
        if let None = sha { return None; }
        let sha = sha.unwrap().get("sha1");
        if let None = sha { return None; }
        let sha = sha.unwrap().as_str();
        if let None = sha { return None; }
        let sha = sha.unwrap();
        let tmp = get_mc_real_path(inh, sha);
        if let Some(e) = tmp {
            res.push_str(e.as_str());
        } else {
            res = res[0..res.rfind("$").unwrap()].to_string();
        }
        Some(res)
    }
    /**
     * 解压natives。填入原json和根路径和版本路径。解压成功返回true，否则返回false。
     */
    pub fn unzip_native(raw_json: String, root_path: &str, version_path: &str) -> bool {
        let mut raw_list: Vec<String> = Vec::new();
        let mut no_list: Vec<String> = Vec::new();
        let mut no_low: Vec<String> = Vec::new();
        let mut temp_list: Vec<String> = Vec::new();
        let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str());
        if let Err(_) = root { return false; }
        let root = root.unwrap();
        let root = root.as_object();
        if let None = root{ return false; }
        let root = root.unwrap();
        let json_lib = root.get("libraries");
        if let None = json_lib { return false; }
        let json_lib = json_lib.unwrap().as_array();
        if let None = json_lib { return false; } 
        let json_lib = json_lib.unwrap();
        for i in json_lib.into_iter() {
            let lib_root = i.as_object();
            if let None = lib_root { continue; }
            let lib_root = lib_root.unwrap();
            let expect_rule = judge_mc_rules(lib_root);
            let lib_name = lib_root.get("name");
            if let None = lib_name { continue; }
            let lib_name = lib_name.unwrap().as_str();
            if let None = lib_name { continue; }
            let lib_name = lib_name.unwrap();
            let lib_arch = lib_root.get("natives");
            if let None = lib_arch { continue; }
            let lib_arch = lib_arch.unwrap().get("windows");
            if let None = lib_arch { continue; }
            let lib_arch = lib_arch.unwrap().as_str();
            if let None = lib_arch { continue; }
            let lib_arch = lib_arch.unwrap();
            if expect_rule { raw_list.push(format!("{}:{}", lib_name, lib_arch)) }
        }
        for i in raw_list.into_iter() {
            if !no_list.contains(&i) {
                no_list.push(i);
            }
        }
        for i in no_list.into_iter() {
            let nocom = i
                                .replace(".", "")
                                .replace(":", "")
                                .replace("-", "")
                                .replace("/", "")
                                .replace("@jar", "")
                                .replace("@zip", "")
                                .replace("@", "");
            let nonum = extract_number(nocom.clone(), false);
            let noword = extract_number(nocom.clone(), true);
            let toint = noword.parse::<u64>();
            if let Err(_) = toint { continue; }
            let toint = toint.unwrap();
            if !temp_list.contains(&nonum) {
                temp_list.push(nonum);
                no_low.push(i);
            } else {
                let temp_1 = temp_list.iter().position(|x| x == &nonum);
                if let None = temp_1 { continue; }
                let temp_1 = temp_1.unwrap();
                let temp_2 = no_low.get(temp_1);
                if let None = temp_2 { continue; }
                let temp_2 = extract_number(temp_2.unwrap().to_string(), true);
                let temp_3 = temp_2.parse::<u64>();
                if let Err(_) = temp_3 { continue; }
                let temp_3 = temp_3.unwrap();
                if temp_3 < toint {
                    no_low.remove(temp_1);
                    no_low.insert(temp_1, i);
                }
            }
        }
        let dir = format!("{}\\{}-TLM-natives", version_path, super::main_mod::extract_file_name(version_path));
        let ver_file = std::path::Path::new(dir.as_str());
        if !ver_file.exists() || (ver_file.exists() && ver_file.is_file()) {
            let cf = std::fs::create_dir_all(ver_file);
            if let Err(_) = cf { return false; }
        }else{ return true; }
        return if no_low.len() == 0 { true } else {
            for c in no_low.into_iter() {
                let cvn = convert_name_to_path(c);
                if let None = cvn { continue; }
                let cvn = cvn.unwrap();
                let rpath = format!("{}\\libraries\\{}", root_path, cvn);
                let uzp = unzip(rpath.clone(), dir.clone());
                if !uzp {
                    continue;
                }
            }
            delete_file_keep(dir, ".dll")
        }
    }
    /**
     * 自定义启动设置类，各位可以调用！
     * 其中，你需要保证以下两点最重要：
     * root_path里面包含【assets、libraries】两个文件夹
     * version_path里面包含【版本.json、版本.jar】两个文件
     * 后期解压Native是默认解压到version_path路径下的！
     * @param account: 账号类，参见LaunchAccount。
     * @param java_path: Java路径
     * @param root_path: MC根路径（用于查询assets、libraries）
     * @param version_path: MC版本路径（用于查询MC元数据JSON和本体jar）
     * @param game_path: MC游戏文件夹（直接用于存储游戏目录）
     * @param window_height: 游戏窗口高度
     * @param window_width: 游戏窗口宽度
     * @param max_memory: 游戏最大内存
     * @param custom_info: 游戏自定义信息（显示在游戏标题界面的右下角和游戏内f3的基本信息。）
     * @param additional_jvm: 游戏额外JVM参数
     * @param additional_game: 游戏额外game参数
     * @param pre_launch_script: 启动前执行脚本
     */
    pub struct LaunchOption{
        account: LaunchAccount,
        java_path: String,
        root_path: String,
        version_path: String,
        game_path: String,
        window_height: usize,
        window_width: usize,
        min_memory: usize,
        max_memory: usize,
        custom_info: String,
        additional_jvm: String,
        additional_game: String,
    }
    impl LaunchOption {
        pub fn new(account: LaunchAccount, java_path: &str, root_path: &str, version_path: &str, game_path: &str) -> Self {
            Self {
                account,
                java_path: java_path.to_string(),
                root_path: root_path.to_string(),
                version_path: version_path.to_string(),
                game_path: game_path.to_string(),
                window_height: 854,
                window_width: 480,
                min_memory: 256,
                max_memory: 4096,
                custom_info: "Tank Launcher Module!".to_string(),
                additional_jvm: String::new(),
                additional_game: String::new(),
            }
        }
        pub fn set_window_height(&mut self, window_height: usize) {
            self.window_height = window_height;
        }
        pub fn set_window_width(&mut self, window_width: usize) {
            self.window_width = window_width;
        }
        pub fn set_min_memory(&mut self, min_memory: usize) {
            self.min_memory = min_memory;
        }
        pub fn set_max_memory(&mut self, max_memory: usize) {
            self.max_memory = max_memory;
        }
        pub fn set_custom_info(&mut self, custom_info: &str) {
            self.custom_info = custom_info.to_string();
        }
        pub fn set_additional_jvm(&mut self, additional_jvm: &str) {
            self.additional_jvm = additional_jvm.to_string();
        }
        pub fn set_additional_game(&mut self, additional_game: &str) {
            self.additional_game = additional_game.to_string();
        }
        pub fn get_account(&self) -> LaunchAccount {
            self.account.clone()
        }
        pub fn get_java_path(&self) -> &str {
            self.java_path.as_str()
        }
        pub fn get_root_path(&self) -> &str {
            self.root_path.as_str()
        }
        pub fn get_version_path(&self) -> &str {
            self.version_path.as_str()
        }
        pub fn get_game_path(&self) -> &str {
            self.game_path.as_str()
        }
        pub fn get_window_height(&self) -> usize {
            self.window_height
        }
        pub fn get_window_width(&self) -> usize {
            self.window_width
        }
        pub fn get_min_memory(&self) -> usize {
            self.min_memory
        }
        pub fn get_max_memory(&self) -> usize {
            self.max_memory
        }
        pub fn get_custom_info(&self) -> &str {
            self.custom_info.as_str()
        }
        pub fn get_additional_jvm(&self) -> &str {
            self.additional_jvm.as_str()
        }
        pub fn get_additional_game(&self) -> &str {
            self.additional_game.as_str()
        }
    }
    pub struct LaunchGame {
        account: LaunchAccount,
        java_path: String,
        root_path: String,
        version_path: String,
        game_path: String,
        window_height: usize,
        window_width: usize,
        min_memory: usize,
        max_memory: usize,
        custom_info: String,
        additional_jvm: String,
        additional_game: String,
        callback: Box<dyn Fn(Vec<&str>)>,
    }
    /**
     * 启动游戏的私有实现类，如果想要调用的话，请直接使用下方的launch_game函数。
     * 如果你想自己实现启动逻辑，可以看下面启动游戏的逻辑，然后调用相对应的函数。因为除了该私有实现以外，别的函数都是pub的！
     */
    impl LaunchGame {
        fn new<F>(option: LaunchOption, callback: F) -> Self 
        where
            F: Fn(Vec<&str>) + 'static
        {
            Self {
                account: option.get_account(),
                java_path: option.get_java_path().to_string(),
                root_path: option.get_root_path().to_string(),
                version_path: option.get_version_path().to_string(),
                game_path: option.get_game_path().to_string(),
                window_height: option.get_window_height(),
                window_width: option.get_window_width(),
                min_memory: option.get_min_memory(),
                max_memory: option.get_max_memory(),
                custom_info: option.get_custom_info().to_string(),
                additional_jvm: option.get_additional_jvm().to_string(),
                additional_game: option.get_additional_game().to_string(),
                callback: Box::new(callback),
            }
        }
        /**
         * 启动游戏的私有函数，此处为检查是否有错。
         */
        fn check_error(&self) -> i32 {
            use super::some_const::*;
            let event_loop = winit::event_loop::EventLoop::new();
            let monitor = event_loop.available_monitors().next();
            if let None = monitor {
                return ERR_UNKNOWN_ERROR;
            }
            let monitor = monitor.unwrap();
            let window_size = monitor.size();
            let mut sys = sysinfo::System::new_all();
            sys.refresh_all();
            let mem = (sys.total_memory() as f64 / 1024.0 / 1024.0).ceil() as i32;
            if self.account.get_online() == 0 {
                let regu = regex::Regex::new("^[0-9a-f]{32}$");
                if let Err(_) = regu {
                    return ERR_UNKNOWN_ERROR;
                }
                let regu = regu.unwrap();
                if !regu.is_match(self.account.get_uuid()) {
                    return ERR_LAUNCH_ACCOUNT_USERUUID;
                }
                let regn = regex::Regex::new("^[a-zA-Z0-9]{3,16}$");
                if let Err(_) = regn {
                    return ERR_UNKNOWN_ERROR;
                }
                let regn = regn.unwrap();
                if !regn.is_match(self.account.get_name()) {
                    return ERR_LAUNCH_ACCOUNT_USERNAME;
                }
            } else if self.account.get_online() == 1 {
                let um = super::account_mod::UrlMethod::new("https://api.minecraftservices.com/minecraft/profile");
                let ih = um.get(self.account.get_access_token());
                if let None = ih {
                    return ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN;
                }
                let json = serde_json::from_str::<serde_json::Value>(ih.unwrap().replace("\\/", "/").as_str());
                if let Err(_) = json {
                    return ERR_UNKNOWN_ERROR;
                }
                let json = json.unwrap();
                let json = json.as_object();
                if let None = json {
                    return ERR_UNKNOWN_ERROR;
                }
                let json = json.unwrap();
                let name = json.get("name");
                if let None = name {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let name = name.unwrap().as_str();
                if let None = name {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let name = name.unwrap();
                let uuid = json.get("id");
                if let None = uuid {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let uuid = uuid.unwrap().as_str();
                if let None = uuid {
                    return ERR_LAUNCH_ACCOUNT_NO_LEGAL;
                }
                let uuid = uuid.unwrap();
                if name != self.account.get_name() && uuid != self.account.get_name() {
                    return ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN;
                }
            } else if self.account.get_online() == 2 {
                if self.account.get_base().is_empty() || !regex::Regex::new(r"^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$").unwrap().is_match(self.account.get_base()) {
                    return ERR_LAUNCH_ACCOUNT_THIRDPARTY_BASE;
                }
                let t = format!("{}/authserver/validate", self.account.get_url());
                let pl = format!("{}{}{}", "{\"accesstoken\":\"", self.account.get_access_token(), "\"}");
                let po = super::account_mod::UrlMethod::new(t.as_str());
                let pl = po.post(pl.as_str(), true);
                if let None = pl {
                    return ERR_LAUNCH_ACCOUNT_THIRDPARTY_ACCESS_TOKEN_OR_URL;
                }
            }
            let jpath = std::path::Path::new(self.java_path.as_str());
            if !jpath.exists() || !jpath.is_file() {
                return ERR_LAUNCH_JAVA_PATH;
            }
            let rpath = std::path::Path::new(self.root_path.as_str());
            if !rpath.exists() || !rpath.is_dir() {
                return ERR_LAUNCH_ROOT_PATH;
            }
            let vpath = std::path::Path::new(self.version_path.as_str());
            if !vpath.exists() || !vpath.is_dir() {
                return ERR_LAUNCH_VERSION_PATH;
            }
            let gpath = std::path::Path::new(self.game_path.as_str());
            if !gpath.exists() || !gpath.is_dir() {
                return ERR_LAUNCH_GAME_PATH;
            }
            if self.window_width < 854 || self.window_width > (window_size.width as usize) {
                return ERR_LAUNCH_WIDTH;
            }
            if self.window_height < 480 || self.window_height > (window_size.height as usize) {
                return ERR_LAUNCH_HEIGHT;
            }
            if self.min_memory > 1024 || self.min_memory < 256 {
                return ERR_LAUNCH_MIN_MEMORY;
            }
            if self.max_memory < 1024 || self.max_memory > (mem as usize) {
                return ERR_LAUNCH_MAX_MEMORY;
            }
            if self.custom_info == "" {
                return ERR_LAUNCH_CUSTOM_INFO;
            }
            return OK;
        }
        /**
         * 拼接1.12.2以上版本的全局参数
         */
        fn put_113(&self, real_json: String, def_jvm: String, defn_jvm: String) -> Option<Vec<String>>{
            let root = serde_json::from_str::<serde_json::Value>(real_json.as_str());
            if let Err(_) = root { return None; }
            let root = root.unwrap();
            let root = root.as_object();
            if let None = root { return None }
            let root = root.unwrap();
            let mcid = root.get("id");
            if let None = mcid { return None; }
            let mcid = mcid.unwrap().as_str();
            if let None = mcid { return None; }
            let mcid = format!("\"{}\"", mcid.unwrap().to_string());
            //定义一个新的Vec
            let mut result: Vec<String> = Vec::new();
            //下列拼接JVM参数
            let def_jvm: Vec<String> = def_jvm.split_whitespace().collect::<Vec<&str>>().iter().map(|e| String::from(*e)).collect();
            let defn_jvm: Vec<String> = defn_jvm.split_whitespace().collect::<Vec<&str>>().iter().map(|e| String::from(*e)).collect();
            result.extend(def_jvm.clone());
            result.extend(defn_jvm.clone());
            result.push("-Dos.name=Windows 10".to_string());
            result.push("-Dos.version=10.0".to_string());
            if !self.additional_jvm.is_empty() { 
                let add_jvm: Vec<String> = (self.additional_jvm.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                result.extend(add_jvm.clone());
            }
            let judge_argu = judge_arguments(real_json.clone(), "jvm");
            if let None = judge_argu { return None; }
            result.extend(judge_argu.unwrap());
            let libs = get_mc_libs(real_json.clone(), self.root_path.as_str(), self.version_path.as_str());
            if let None = libs { return None; }
            let libs = libs.unwrap();
            for i in result.iter_mut() {
                *i = i
                    .replace("${natives_directory}", 
                        format!("{}\\{}-TLM-natives", 
                            self.version_path,
                            super::main_mod::extract_file_name(self.version_path.as_str())).as_str())
                    .replace("${launcher_name}", "TLM")
                    .replace("${launcher_version}", 
                        super::some_const::TLM_VERSION
                                .replace(".", "")
                                .replace("-", "")
                                .replace("Alpha", "")
                                .replace("Beta", "")
                                .replace("Pre", "")
                                .as_str())
                    .replace("${classpath}", libs.as_str())
                    .replace("${version_name}", mcid.as_str())
                    .replace("${library_directory}", format!("{}\\libraries", self.root_path).as_str())
                    .replace("${classpath_separator}", ";");  //MacOS 是 冒号【:】
            }
            if !self.account.get_base().is_empty() {
                unsafe {
                    if super::some_var::AUTHLIB_PATH.is_empty(){
                        panic!("You're not assign the AUTHLIB_PATH in some_var mod, please retry!")
                    }
                    let path = std::path::Path::new(super::some_var::AUTHLIB_PATH.as_str());
                    if path.exists() && path.is_file() {
                        result.push(format!("-javaagent:{}={}",
                            super::some_var::AUTHLIB_PATH.as_str(),
                            self.account.get_url()
                        ));
                        result.push("-Dauthlibinjector.side=client".to_string());
                        result.push(format!("-Dauthlibinjector.yggdrasil.prefetched={}",
                            super::some_var::AUTHLIB_PATH.as_str()
                        ));
                    }else{
                        panic!("You're AUTHLIB_PATH file is not exist, please retry!")
                    }
                }
            }
            result.push(format!("-Xmn{}m", self.min_memory));
            result.push(format!("-Xmx{}m", self.max_memory));
            //下列拼接game参数
            let main_class = root.get("mainClass");
            if let None = main_class { return None; }
            let main_class = main_class.unwrap().as_str();
            if let None = main_class { return None; }
            result.push(main_class.unwrap().to_string());
            let judge_game = judge_arguments(real_json.clone(), "game");
            if let None = judge_game { return None; }
            result.extend(judge_game.unwrap());
            if !self.additional_game.contains("--fullScreen") {
                result.push("--width".to_string());
                result.push(self.window_width.to_string());
                result.push("--height".to_string());
                result.push(self.window_height.to_string());
            }
            let ma = root.get("minecraftArguments");
            if let Some(e) = ma {
                if let Some(f) = e.as_str() {
                    if !f.is_empty() {
                        let ma: Vec<String> = (f.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                        result.extend(ma);
                    }
                }
            }
            let asset_index = root.get("assetIndex");
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap().get("id");
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap().as_str();
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap();
            for i in result.iter_mut() {
                *i = i
                    .replace("${auth_player_name}", self.account.get_name())
                    .replace("${version_name}", mcid.as_str())
                    .replace("${game_directory}", format!("{}", self.game_path).as_str())
                    .replace("${assets_root}", format!("{}\\assets", self.root_path).as_str())
                    .replace("${assets_index_name}", asset_index)
                    .replace("${auth_uuid}", self.account.get_uuid())
                    .replace("${auth_access_token}", self.account.get_access_token())
                    .replace("${user_type}", self.account.get_atype())
                    .replace("${version_type}", format!("{}", self.custom_info).as_str());
            }
            if !self.additional_game.is_empty() {
                let add_game: Vec<String> = (self.additional_game.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                result.extend(add_game.clone());
            }
            if result.contains(&"optifine.OptiFineForgeTweaker".to_string()) {
                let temp_1 = result.iter().position(|x| x == &"optifine.OptiFineForgeTweaker".to_string()).unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineForgeTweaker".to_string());
            }
            if result.contains(&"optifine.OptiFineTweaker".to_string()) {
                let temp_1 = result.iter().position(|x| x == &"optifine.OptiFineTweaker".to_string()).unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineTweaker".to_string());
            }
            Some(result)
        }
        /**
         * 拼接1.12.2以下版本的全局参数
         */
        fn put_112(&self, real_json: String, def_jvm: String, defn_jvm: String) -> Option<Vec<String>> {
            let root = serde_json::from_str::<serde_json::Value>(real_json.as_str());
            if let Err(_) = root { return None; }
            let root = root.unwrap();
            let ma = root.get("minecraftArguments");
            if let None = ma { return None; }
            let ma = ma.unwrap().as_str();
            if let None = ma { return None; }
            let ma = ma.unwrap();
            if ma.is_empty() { return None; }
            let mcid = root.get("id");
            if let None = mcid { return None; }
            let mcid = mcid.unwrap().as_str();
            if let None = mcid { return None; }
            let mcid = mcid.unwrap();
            let asset_index = root.get("assetIndex");
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap().get("id");
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap().as_str();
            if let None = asset_index { return None; }
            let asset_index = asset_index.unwrap();
            let mut result: Vec<String> = Vec::new();
            let def_jvm: Vec<String> = def_jvm.split_whitespace().collect::<Vec<&str>>().iter().map(|e| String::from(*e)).collect();
            let defn_jvm: Vec<String> = defn_jvm.split_whitespace().collect::<Vec<&str>>().iter().map(|e| String::from(*e)).collect();
            result.extend(def_jvm.clone());
            result.extend(defn_jvm.clone());
            if !self.additional_jvm.is_empty() { 
                let add_jvm: Vec<String> = (self.additional_jvm.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                result.extend(add_jvm.clone());
            }
            let judge_argu = judge_arguments(real_json.clone(), "jvm");
            if let Some(e) = judge_argu {
                result.extend(e.clone());
            }
            if !self.additional_jvm.is_empty() { 
                let add_jvm: Vec<String> = (self.additional_jvm.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                result.extend(add_jvm.clone());
            }
            let libs = get_mc_libs(real_json.clone(), self.root_path.as_str(), self.version_path.as_str());
            if let None = libs { return None; }
            let libs = libs.unwrap();
            result.push(format!("-Djava.library.path={}\\{}-TLM-natives",
                self.version_path, 
                super::main_mod::extract_file_name(self.version_path.as_str())));
            result.push("-cp".to_string());
            result.push(libs);
            if !self.account.get_base().is_empty() {
                unsafe {
                    if super::some_var::AUTHLIB_PATH.is_empty(){
                        panic!("You're not assign the AUTHLIB_PATH in some_var mod, please retry!")
                    }
                    let path = std::path::Path::new(super::some_var::AUTHLIB_PATH.as_str());
                    if path.exists() && path.is_file() {
                        result.push(format!("-javaagent:{}={}",
                            super::some_var::AUTHLIB_PATH.as_str(),
                            self.account.get_url()
                        ));
                        result.push("-Dauthlibinjector.side=client".to_string());
                        result.push(format!("-Dauthlibinjector.yggdrasil.prefetched={}",
                            super::some_var::AUTHLIB_PATH.as_str()
                        ));
                    }else{
                        panic!("You're AUTHLIB_PATH file is not exist, please retry!")
                    }
                }
            }
            result.push(format!("-Xmn{}m", self.min_memory));
            result.push(format!("-Xmx{}m", self.max_memory));
            let main_class = root.get("mainClass");
            if let None = main_class { return None; }
            let main_class = main_class.unwrap().as_str();
            if let None = main_class { return None; }
            result.push(main_class.unwrap().to_string());
            let judge_game = judge_arguments(real_json.clone(), "game");
            if let Some(s) = judge_game {
                result.extend(s.clone());
            }
            let ma: Vec<String> = ma.split_whitespace().collect::<Vec<&str>>().iter().map(|e| String::from(*e)).collect();
            result.extend(ma);
            if !self.additional_game.contains("--fullScreen") {
                result.push("--width".to_string());
                result.push(self.window_width.to_string());
                result.push("--height".to_string());
                result.push(self.window_height.to_string());
            }
            if !self.additional_game.is_empty() {
                let add_game: Vec<String> = (self.additional_game.split_whitespace().collect::<Vec<&str>>()).iter().map(|e| String::from(*e)).collect();
                result.extend(add_game.clone());
            }
            for i in result.iter_mut() {
                *i = i
                    .replace("${auth_player_name}", self.account.get_name())
                    .replace("${auth_session}", self.account.get_uuid())
                    .replace("${game_directory}", format!("{}", self.game_path).as_str())
                    .replace("${game_assets}", format!("{}\\assets\\virtual\\legacy", self.root_path).as_str())
                    .replace("${assets_root}", format!("{}\\assets", self.root_path).as_str())
                    .replace("${version_name}", mcid)
                    .replace("${assets_index_name}", asset_index)
                    .replace("${auth_uuid}", self.account.get_uuid())
                    .replace("${auth_access_token}", self.account.get_access_token())
                    .replace("${user_properties}", "{}")
                    .replace("${user_type}", self.account.get_atype())
                    .replace("${version_type}", self.custom_info.as_str())
                    .replace("${classpath_separator}", ";");  //MacOS 是 冒号【:】
            }
            if result.contains(&"optifine.OptiFineForgeTweaker".to_string()) {
                let temp_1 = result.iter().position(|x| x == &"optifine.OptiFineForgeTweaker".to_string()).unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineForgeTweaker".to_string());
            }
            if result.contains(&"optifine.OptiFineTweaker".to_string()) {
                let temp_1 = result.iter().position(|x| x == &"optifine.OptiFineTweaker".to_string()).unwrap();
                result.remove(temp_1 - 1);
                result.remove(temp_1 - 1);
                result.push("--tweakClass".to_string());
                result.push("optifine.OptiFineTweaker".to_string());
            }
            Some(result)
        }
        /**
         * 如果没有错误，则会调用该函数。如果启动过程中出现不可预知的错误，则会直接panic掉！
         */
        fn game_launch(&self) {
            let def_jvm: String = String::from("-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Dfml.ignoreInvalidMinecraftCertificates=True -Dfml.ignorePatchDiscrepancies=True -Dlog4j2.formatMsgNoLookups=True");
            let defn_jvm: String = String::from("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump");
            let version_json_path = get_mc_real_path(self.version_path.clone(), ".json");
            if let None = version_json_path {
                panic!("Cannot find eligible json in the version dir, please retry!");
            }
            let version_json_path = version_json_path.unwrap();
            let final_json: String;
            let raw_json = super::main_mod::get_file(version_json_path.as_str());
            if let None = raw_json {
                panic!("Cannot read json in the version dir, please retry!")
            }
            let raw_json = raw_json.unwrap();
            let inherits_json = get_mc_inherits_from(self.version_path.clone(), "inheritsFrom");
            if let None = inherits_json {
                panic!("Cannot find vanilla key pass version inheritsFrom key, you have not download vanilla version, please retry!")
            }
            let inherits_json = inherits_json.unwrap();
            if !inherits_json.eq(self.version_path.as_str()) {
                let file = get_mc_real_path(inherits_json, ".json");
                if let None = file {
                    panic!("Cannot read already find inheritsFrom key to the vanilla json, please retry!");
                }
                let file = file.unwrap();
                let fjson = super::main_mod::get_file(file.as_str());
                if let None = fjson {
                    panic!("Cannot read already find inheritsFrom key to the vanilla json content, please retry!")
                }
                final_json = fjson.unwrap();
            }else{
                final_json = raw_json.clone();
            }
            if !unzip_native(final_json.clone(), self.root_path.as_str(), self.version_path.as_str()) {
                panic!("Cannot unzip natives, please retry!")
            }
            let real_json = replace_mc_inherits_from(raw_json, final_json);
            if let None = real_json {
                panic!("Cannot find id or mainClass in inheritsFrom json, please retry!")
            }
            let real_json = real_json.unwrap();
            let mut param = self.put_113(real_json.clone(), def_jvm.clone(), defn_jvm.clone());
            if let None = param {
                param = self.put_112(real_json.clone(), def_jvm.clone(), defn_jvm.clone());
                if let None = param {
                    panic!("Cannot join all param in your params, please retry!")
                }
            }
            let mut param = param.unwrap();
            param.splice(0..0, [self.java_path.clone()]);
            let command = param.iter().map(AsRef::as_ref).collect();
            (self.callback)(command);
            // return param.unwrap();
        }
    }
    /**
     * 提供了Account登录的启动类模块，该类不是用来登录账号的，只是用来启动游戏时才用到的！
     * @function new_offline: 为新建了一个离线登录。如果你身处除中国以外的地方，请不要使用该新建函数。
     * @function new_offline_default: 为新建一个默认的玩家，仅需输入玩家名称，使用bukkit方式生成一个UUID。
     * @function new_microsoft为新建了一个微软登录。该登录方式适用于全世界。
     * @function new_thirdparty为新建了一个第三方登录。除非你信任该模块地址，否则你不能使用该新建函数。
     * @function new_thirdparty_default为新建了一个第三方登录。并且无需填入元数据，仅需多填入一个第三方登录网址。
     * @param name: 玩家登录名称
     * @param uuid: 玩家登录UUID（需要符合32位16进制字符）
     * @param access_token: 登录密钥（仅在使用微软、第三方时才用到。）
     * @param atype: 登录类型，该参数无需自己填
     * @param url: 第三方登录网址，该参数填入你的第三方登录域名。
     * @param base: 第三方登录元数据base64编码方案，如果你想使用第三方快速启动，
     * @param online: 仅用于标记目前你使用的哪种方式登录，不作为默认参数提供。
     * 
     * 
     * 离线模式调用示例：LaunchAccount::new_offline("Steve", "1234567890abcdef1234567890abcdef");
     * 或：LaunchAccount::new_offline_default("Steve");  // UUID会自动按照bukkit方式生成。
     * 微软登录调用示例：LaunchAccount::new_microsoft("Steve", "1234567890abcdef1234567890abcdef", "<你的access token密钥>")
     * 第三方外置登录调用示例：LaunchAccount::new_thirdparty(
     *                      "Steve", 
     *                      "1234567890abcdef1234567890abcdef", 
     *                      "<你的access token密钥>", 
     *                      "<你的皮肤站元数据base64编码>", 
     *                      "https://littleskin.cn/api/yggdrasil"")  # 皮肤站元数据必须得是精确到api/yggdrasil的！
     * 或：LaunchAccount::new_thirdparty(
     *                      "Steve", 
     *                      "1234567890abcdef1234567890abcdef", 
     *                      "<你的access token密钥>", 
     *                      "https://littleskin.cn/api/yggdrasil"")  # 此时皮肤站元数据base64编码会自动从api密钥获取。
     */
    #[derive(Clone)]
    pub struct LaunchAccount{
        name: String,
        uuid: String,
        access_token: String,
        atype: String,
        base: String,
        url: String,
        online: i32,
    }
    impl LaunchAccount{
        fn new(name: String, uuid: String, access_token: String, atype: String, base: String, url: String, online: i32) -> Self {
            Self {
                name: name.clone(),
                uuid: uuid.clone(),
                access_token: access_token.clone(),
                atype: atype.clone(),
                base: base.clone(),
                url: url.clone(),
                online,
            }
        }
        pub fn new_offline(name: &str, uuid: &str) -> Self{
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                uuid.to_string(),
                String::from("Legacy"),
                String::new(),
                String::new(),
                0)
        }
        pub fn new_offline_default(name: &str) -> Self {
            let uuid = super::main_mod::generate_bukkit_uuid(name);
            LaunchAccount::new(
                name.to_string(),
                uuid.clone(),
                uuid.clone(),
                String::from("Legacy"),
                String::new(),
                String::new(),
                0)
        }
        pub fn new_microsoft(name: &str, uuid: &str, access_token: &str) -> Self{
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                String::new(),
                String::new(),
                1)
        }
        pub fn new_thirdparty(name: &str, uuid: &str, access_token: &str, base: &str, url: &str) -> Self{
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                base.to_string(),
                url.to_string(),
                2)
        }
        pub fn new_thirdparty_default(name: &str, uuid: &str, access_token: &str, url: &str) -> Self {
            LaunchAccount::new(
                name.to_string(),
                uuid.to_string(),
                access_token.to_string(),
                String::from("msa"),
                super::main_mod::generate_thirdparty_metadata_base64(url),
                url.to_string(),
                2)
        }
        pub fn get_name(&self) -> &str {
            self.name.as_str()
        }
        pub fn get_uuid(&self) -> &str {
            self.uuid.as_str()
        }
        pub fn get_access_token(&self) -> &str {
            self.access_token.as_str()
        }
        pub fn get_atype(&self) -> &str {
            self.atype.as_str()
        }
        pub fn get_base(&self) -> &str {
            self.base.as_str()
        }
        pub fn get_url(&self) -> &str {
            self.url.as_str()
        }
        fn get_online(&self) -> i32 {
            self.online
        }
    }
    /**
     * 该函数为启动游戏的函数，接受一个LaunchOption函数和一个闭包。
     * 其中，闭包用于获取启动参数。
     */
    pub fn launch_game<F>(option: LaunchOption, callback: F) -> Result<(), i32>
    where
        F: Fn(Vec<&str>) + 'static
    {
        let res = LaunchGame::new(option, callback);
        let code = res.check_error();
        return if code != 0 {
            Err(code)
        } else {
            res.game_launch();
            Ok(())
        }
    }
}