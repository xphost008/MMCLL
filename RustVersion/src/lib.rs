//!
//! 欢迎来到Tank Launcher Module!
//! 本模块使用MIT协议进行开源！各位可以随意使用本模块的函数。
//! 本模块暂未发布至crates.io，因为我不想发！！
//! 本模块中，很多需要用到网络请求的部分，未使用async包围。因为作者的TLM不是GUI程序。各位想加就加上async/await把！
pub mod account;
pub mod download;
pub mod err;
pub mod launcher;

pub use err::{MMCLLError, MMCLLResult};

/// 部分常量值，在程序的任意位置均可直接调用。
pub mod constants {
    pub const LAUNCHER_NANE: &str = "MMCLL"; //在使用此库时，请自觉将此值改成你的【<启动器名称>】。在使用默认方式启动时，会自动将【${launcher_name}】替换成该值。
    pub const LAUNCHER_VERSION: &str = "0.0.1-Alpha-12"; //在使用此库时，请自觉将此值改成你的【<启动器版本>】【可以加上Alpha、Beta、Pre三个值，因为在启动替换（${launcher_version}）时用到这个值。不过各位可以自行去函数put_arguments进行修改以适配该值。】
    pub const USER_AGENT: &str = "MMCLL/0.0.1.12"; //在使用此库时，请自觉将此值改成你的【<启动器名称>/<启动器版本>】。
}

/// 部分全局变量值。在需要的时候可以使用with_borrow包裹住该变量以便使用，赋值和引用均可。但是你需要为你赋过的值负责！。
pub mod some_var {
    use std::{cell::RefCell, sync::atomic::AtomicBool};
    thread_local! {
        /// 是否从官方下载源下载
        static IS_DOWNLOAD_SOURCE_OFFICIAL: AtomicBool = const { AtomicBool::new(false) };
        /// mc的元数据（可以自己赋值也可以由类库帮忙赋值！）仅能赋值元数据值，如果赋上了别的值，后果自负！
        pub static MC_ROOT_JSON: RefCell<serde_json::Value> = const { RefCell::new(serde_json::Value::Null) };
        /// 设置第三方登录的模块jar文件。在使用第三方登录的时候一定要设置该参数！
        pub static AUTHLIB_PATH: RefCell<String> = const { RefCell::new(String::new()) };
        // 最大线程，但是在Rust里指的是最大并发量（必须要提前赋值，否则将按照默认64处理。）
        pub static BIGGEST_THREAD: RefCell<i32> = const { RefCell::new(64) };
        #[allow(unused)]
        pub static AUTHLIB_URL: RefCell<String> = const { RefCell::new(String::new()) };
    }

    /// 获取是否从官方下载源下载
    pub fn is_download_from_official() -> bool {
        let mut res = false;
        IS_DOWNLOAD_SOURCE_OFFICIAL.with(|f| {
            res = f.load(std::sync::atomic::Ordering::SeqCst);
        });
        res
    }
}

/// 许多在启动时可能需要用到的静态函数。（无需初始化，仅需直接调用。）
/// 从一个path获取外部文件。
/// 此处使用了encoding转码，以防止有某些大聪明玩家使用GBK方式写文件
pub fn get_file(path: &str) -> Option<String> {
    use std::io::Read;
    let p = std::path::Path::new(path);
    let mut ss = std::fs::File::open(p).ok()?;
    let mut buf = Vec::new();
    ss.read_to_end(&mut buf).ok()?;
    if let Ok(e) = String::from_utf8(buf.clone()) {
        Some(e)
    } else {
        let (cow, _, h) = encoding_rs::GBK.decode(&buf);
        if h {
            None
        } else {
            Some(cow.to_string())
        }
    }
}

/// 将Vec<u8>写出到文件
pub fn set_file_vecu8(path: &str, content: &[u8]) -> bool {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        return false;
    }
    let parent = match p.parent() {
        Some(p) => p,
        None => return false,
    };
    if !parent.exists() || parent.exists() && parent.is_file() {
        let q = std::fs::create_dir_all(parent);
        if q.is_err() {
            return false;
        }
    }
    let mut f = match std::fs::File::create(p) {
        Ok(f) => f,
        Err(_) => return false,
    };
    use std::io::Write;
    f.write_all(content).is_ok()
}

/// 将内容写出到文件
pub fn set_file(path: &str, content: String) -> bool {
    set_file_vecu8(path, content.as_bytes())
}

/// 删除文件
pub fn delete_file(path: &str) -> bool {
    let p = std::path::Path::new(path);
    if !p.exists() || p.exists() && p.is_dir() {
        return false;
    }
    std::fs::remove_file(p).is_ok()
}

/// 获取某一个文件的SHA1值
pub fn get_sha1(path: &str) -> Option<String> {
    let mut file = std::fs::File::open(path).ok()?;
    use crypto::digest::Digest;
    use std::io::Read;
    let mut sha1 = crypto::sha1::Sha1::new();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        sha1.input(&buffer[..n]);
    }
    let hash = sha1.result_str();
    Some(hash)
}

/// 将16进制字符串转换成Vec<u8>形式
/// 例如【aabbcc】转成【[170, 187, 204]】
pub fn hex_decode(raw: &str) -> Option<Vec<u8>> {
    let mut res: Vec<u8> = Vec::new();
    if raw.len() % 2 != 0 {
        return None;
    }
    let reg = regex::Regex::new("^(?:[a-f0-9]{2})+$").unwrap();
    if !reg.is_match(raw) {
        return None;
    }
    let chars = raw.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < chars.len() {
        let r = u32::from_str_radix(format!("{}{}", chars[i], chars[i + 1]).as_str(), 16).unwrap();
        res.push(r as u8);
        i += 2;
    }
    Some(res.clone())
}

/// 将16进制数组转成String形式
/// 例如【[170, 187, 204]】转成【aabbcc】
pub fn hex_encode(raw: Vec<u8>) -> String {
    let mut res = String::new();
    for i in raw.into_iter() {
        res.push_str(format!("{:x}", i).as_str());
    }
    res.clone()
}

/// 该函数目前仅适用于在离线登录时根据用户名生成一个唯一的UUID。
pub fn generate_bukkit_uuid(name: &str) -> String {
    use crypto::digest::Digest;
    let mut md5 = crypto::md5::Md5::new();
    md5.input_str(format!("OfflinePlayer:{}", name).as_str());
    let res_str = md5.result_str();
    let mut res_hex = hex_decode(res_str.as_str()).unwrap();
    res_hex[6] = (res_hex[6] & 0x0f) | 0x30;
    res_hex[8] = (res_hex[8] & 0x3f) | 0x80;
    hex_encode(res_hex)
}

/// 该函数目前仅适用于在初始化第三方登录时对该皮肤站元数据进行base64编码。
/// 该函数已废弃，如果想获取元数据base64编码，请自行使用account_mod下的登录一次，即可直接异步获取。
#[allow(dead_code, deprecated)]
#[deprecated(
    since = "0.0.8",
    note = "Please login thirdparty in account_mod, and auto get base64 code by sync."
)]
pub fn generate_thirdparty_metadata_base64(url: &str) -> String {
    use base64::Engine;
    let um = account::UrlMethod::new(url);
    let metadata = um.get_default();
    if metadata.is_none() {
        return String::new();
    }
    let metadata = String::from_utf8(metadata.unwrap());
    if metadata.is_err() {
        return String::new();
    }

    base64::engine::general_purpose::STANDARD.encode(metadata.unwrap().replace("\\/", "/"))
}

/// 截取文件名
pub fn extract_file_name(file: &str) -> String {
    let rf = file.rfind("\\");
    if rf.is_none() {
        return String::new();
    }
    let rf = rf.unwrap();
    let versub = file.get((rf + 1)..file.len());
    if versub.is_none() {
        return String::new();
    }
    let versub = versub.unwrap();
    versub.to_string()
}

/// 获取exe的位数（32位或64位）
pub fn get_file_bit(file: String) -> Option<bool> {
    let path = std::path::Path::new(file.as_str());
    if !path.exists() || path.exists() && path.is_dir() {
        return None;
    }
    let data = pelite::FileMap::open(path).ok()?;
    let file = pelite::PeFile::from_bytes(&data).ok()?;
    match file {
        pelite::Wrap::T64(_) => Some(true),
        pelite::Wrap::T32(_) => Some(false),
    }
}

/// 获取exe文件的版本号
pub fn get_file_version(file: String) -> Option<String> {
    let path = std::path::Path::new(file.as_str());
    if !path.exists() || path.exists() && path.is_dir() {
        return None;
    }
    let data = pelite::FileMap::open(path).ok()?;
    let file = pelite::PeFile::from_bytes(&data).ok()?;
    let file = file.resources().ok()?;
    let fixed_version = file.version_info().ok()?.fixed()?;
    Some(format!(
        "{}.{}.{}.{}",
        fixed_version.dwFileVersion.Major,
        fixed_version.dwFileVersion.Minor,
        fixed_version.dwFileVersion.Build,
        fixed_version.dwFileVersion.Patch
    ))
}

/// 通过正版用户名获取其UUID
pub fn name_to_uuid(name: &str) -> Option<String> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", name);
    let url = account::UrlMethod::new(url.as_str());
    let res = url.get_default()?;
    let res = String::from_utf8(res.clone()).ok()?;
    let ser = serde_json::from_str::<serde_json::Value>(res.as_str()).ok()?;
    let ser = ser.get("id")?.as_str()?;
    Some(ser.to_string())
}