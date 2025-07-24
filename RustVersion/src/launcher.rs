//! 专注于启动游戏的模块，所有启动游戏的函数都可以在这里面找到！
use crate::constants::{LAUNCHER_NANE, LAUNCHER_VERSION};
use crate::some_var::is_download_from_official;
use crate::{MMCLLError, MMCLLResult};

/// 此方法用于将json libraries里的name值转换为path。
pub fn convert_name_to_path(name: String) -> Option<String> {
    let mut name = name.clone();
    let suffix: String;
    if name.contains("@") {
        let rf = name.rfind("@")?;
        suffix = name[(rf + 1)..name.len()].to_string();
        name = name[0..rf].to_string();
    } else {
        suffix = String::from("jar")
    }
    let spl: Vec<&str> = name.split(":").collect();
    if spl.len() == 4 {
        Some(format!(
            "{}\\{}\\{}\\{}-{}-{}.{}",
            spl[0].replace(".", "\\"),
            spl[1],
            spl[2],
            spl[1],
            spl[2],
            spl[3],
            suffix
        ))
    } else if spl.len() == 3 {
        Some(format!(
            "{}\\{}\\{}\\{}-{}.{}",
            spl[0].replace(".", "\\"),
            spl[1],
            spl[2],
            spl[1],
            spl[2],
            suffix
        ))
    } else {
        None
    }
}
/// 根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
/// 会按照clientVersion、patches->game|version、metadata->versions->releaseTime、id值进行找。
/// 如果连最终的id值也不符合，则返回必定返回None！
/// 但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
/// 所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。当然也可以用catch_unwind来捕捉panic也就对了！
pub fn get_mc_vanilla_version(json: String) -> Option<String> {
    let root = serde_json::from_str::<serde_json::Value>(json.as_str()).ok()?;
    if let Some(e) = root["clientVersion"].as_str() {
        if !e.is_empty() {
            return Some(e.to_string());
        }
    }
    if let Some(e) = root["patches"].as_array() {
        for i in e.iter() {
            let id = i["id"].as_str();
            if id.is_none() {
                continue;
            }
            if id?.eq("game") {
                let mcid = i["version"].as_str();
                if let Some(f) = mcid {
                    if !f.is_empty() {
                        return Some(f.to_string());
                    }
                }
            }
        }
    }
    if let Some(w) = root["releaseTime"].as_str() {
        let v = if is_download_from_official() {
            "https://piston-meta.mojang.com/mc/game/version_manifest.json"
        } else {
            "https://bmclapi2.bangbang93.com/mc/game/version_manifest.json"
        };
        if super::some_var::MC_ROOT_JSON
            .with_borrow(|e| e.clone())
            .is_null()
        {
            let url = crate::account::UrlMethod::new(v);
            if let Some(e) = url.get_default() {
                let e = String::from_utf8(e);
                if let Ok(f) = e {
                    let s = serde_json::from_str::<serde_json::Value>(f.as_str()).ok()?;
                    super::some_var::MC_ROOT_JSON.set(s.clone());
                }
            }
        }
        if !super::some_var::MC_ROOT_JSON
            .with_borrow(|e| e.clone())
            .is_null()
        {
            let mrj = super::some_var::MC_ROOT_JSON.with_borrow(|e| e.clone());
            if let Some(g) = mrj["version"].as_array() {
                for h in g.iter() {
                    if let Some(j) = h["releaseTime"].as_str() {
                        if j.eq(w) {
                            if let Some(d) = h["id"].as_str() {
                                return Some(d.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    if let Some(e) = root["id"].as_str() {
        if !e.is_empty() {
            return Some(e.to_string());
        }
    }
    None
}

/// 解压任意文件到路径。
/// 该函数并不会返回进度值，各位可以自行查看该函数并实现自己的回显进度的zip解压。
/// 该函数目前仅会返回bool值，如果解压成功则返回true，反之如果里面出现任何错误，则直接返回false。
pub fn unzip(zipfile: String, extfile: String) -> bool {
    let zip_path = std::path::Path::new(zipfile.as_str());
    let ext_path = std::path::Path::new(extfile.as_str());
    if !zip_path.exists() || (zip_path.exists() && !zip_path.is_file()) {
        return false;
    }
    if !zip_path.exists() || (zip_path.exists() && zip_path.is_file()) {
        let cd = std::fs::create_dir_all(ext_path);
        if cd.is_err() {
            return false;
        }
    }
    let zip_file = std::fs::File::open(zip_path);
    if zip_file.is_err() {
        return false;
    }
    let zip_file = zip_file.unwrap();
    let zip_ext = zip::ZipArchive::new(zip_file);
    if zip_ext.is_err() {
        return false;
    }
    let mut zip_ext = zip_ext.unwrap();
    for i in 0..zip_ext.len() {
        let f = zip_ext.by_index(i);
        if f.is_err() {
            continue;
        }
        let mut f = f.unwrap();
        if f.is_dir() {
            let ext_dir = ext_path.join(std::path::Path::new(&f.name().replace("\\", "")));
            let cd = std::fs::create_dir_all(ext_dir);
            if cd.is_err() {
                continue;
            }
        } else {
            let file_path = ext_path.join(std::path::Path::new(f.name()));
            let ext_file = if !file_path.exists() {
                std::fs::File::create(file_path)
            } else {
                std::fs::File::open(file_path)
            };
            if ext_file.is_err() {
                continue;
            }
            let mut ext_file = ext_file.unwrap();
            let res = std::io::copy(&mut f, &mut ext_file);
            if res.is_err() {
                continue;
            }
        }
    }
    true
}

/// 删除文件夹中的所有文件，但是保留后缀为suffix的值。该函数用于解压natives时需要删掉除了dll以外的所有文件。
pub fn delete_file_keep(dir_path: String, suffix: &str) -> bool {
    if dir_path.is_empty() {
        return false;
    }
    if suffix.is_empty() || suffix.eq(".") {
        return false;
    }
    if !dir_path.contains("\\") {
        return false;
    }
    let suffix = &suffix[1..suffix.len()];
    let dir = walkdir::WalkDir::new(dir_path.as_str());
    for i in dir.into_iter().filter_map(|e| e.ok()) {
        let path = i.path();
        if path.exists() {
            if path.is_dir() {
                continue;
            }
            let path_ext = path.extension();
            if path_ext.is_none() {
                continue;
            }
            let path_ext = path_ext.unwrap();
            if !path_ext.eq(suffix) {
                let cd = std::fs::remove_file(path);
                if cd.is_err() {
                    continue;
                }
            }
        }
    }
    true
}

/// 任何类都可以用的数字转换！
/// 可以将字符串中的数字提取出来，或者是字符串中的非数字【字符】提取出来！
pub fn extract_number(ext: String, isnum: bool) -> String {
    ext.chars()
        .filter(|&c| {
            if isnum {
                c.is_numeric()
            } else {
                c.is_ascii_alphabetic()
            }
        })
        .collect::<String>()
}

/// 根据找到的json中的inheritsFrom或者jar值，准确的找到另一个有关该原版的版本文件夹。
pub fn get_mc_inherits_from(version_path: String, ioj: &str) -> Option<String> {
    let path = std::path::Path::new(version_path.as_str());
    if path.exists() && path.is_dir() {
        let real_path = get_mc_real_path(version_path.clone(), ".json")?;
        let real_file = crate::get_file(real_path.as_str())?;
        let root = serde_json::from_str::<serde_json::Value>(real_file.as_str()).ok()?;
        if let Some(e) = root[ioj].as_str() {
            if e.is_empty() {
                return Some(version_path.clone());
            }
            let parent_path = path.parent()?;
            let dir = walkdir::WalkDir::new(parent_path).min_depth(1).max_depth(1);
            for i in dir.into_iter().filter_map(|e| e.ok()) {
                let pa = i.path();
                if pa.is_file() {
                    continue;
                }
                let ps = pa.display().to_string();
                let version_json = get_mc_real_path(ps.clone(), ".json");
                if version_json.is_none() {
                    continue;
                }
                let json_content = crate::get_file(version_json?.as_str());
                if json_content.is_none() {
                    continue;
                }
                let vanilla_version = get_mc_vanilla_version(json_content?);
                if vanilla_version.is_none() {
                    continue;
                }
                if vanilla_version?.eq(e) {
                    return Some(ps.clone());
                }
            }
        } else {
            return Some(version_path.clone());
        }
    }
    None
}

/// 从inheritsFrom键中找到的json当作原版json，并拼接上inheritsFrom键所在的json，将其合并成一个json！
pub fn replace_mc_inherits_from(mut raw_json: String, mut ins_json: String) -> Option<String> {
    fn return_some(k: &mut serde_json::Map<String, serde_json::Value>) -> Option<String> {
        serde_json::to_string(&k).ok()
    }
    if raw_json.is_empty() || ins_json.is_empty() {
        return None;
    }
    raw_json = raw_json.replace("\\", "");
    ins_json = ins_json.replace("\\", "");
    if raw_json.eq(ins_json.as_str()) {
        return Some(raw_json);
    }
    let rt_raw = serde_json::from_str::<serde_json::Value>(raw_json.as_str()).ok()?;
    let rt_raw = rt_raw.as_object()?;
    let mut rt_ins = serde_json::from_str::<serde_json::Value>(ins_json.as_str()).ok()?;
    let rt_ins = rt_ins.as_object_mut()?;
    let mc = rt_raw["mainClass"].as_str()?;
    rt_ins.remove("mainClass");
    rt_ins.insert(
        "mainClass".to_string(),
        serde_json::Value::String(mc.to_string()),
    );
    let id = rt_raw["id"].as_str()?;
    rt_ins.remove("id");
    rt_ins.insert("id".to_string(), serde_json::Value::String(id.to_string()));
    let raw_lib = rt_raw.get("libraries");
    if let Some(d) = raw_lib {
        if let Some(e) = d.as_array() {
            for i in e.iter() {
                if let Some(f) = i.as_object() {
                    if let Some(h) = rt_ins.get_mut("libraries") {
                        if let Some(g) = h.as_array_mut() {
                            g.push(serde_json::Value::Object(f.clone()));
                        }
                    }
                }
            }
        }
    }
    if let Some(r1) = rt_raw.get("arguments") {
        if let Some(r2) = r1.get("jvm") {
            if let Some(e) = r2.as_array() {
                for i in e.iter() {
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
                for i in e.iter() {
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
            rt_ins.insert(
                "minecraftArguments".to_string(),
                serde_json::Value::String(e.to_string()),
            );
        }
    }
    return_some(rt_ins)
}

/// 从一个文件夹中根据suffix获取一个准确的文件。
/// 其中当suffix为“.json”的时候逻辑可能会略有不同，请不要在意！
/// suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
/// 目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回None
pub fn get_mc_real_path(version_path: String, suffix: &str) -> Option<String> {
    let path = std::path::Path::new(version_path.as_str());
    if path.exists() && path.is_dir() {
        let dir = walkdir::WalkDir::new(path).min_depth(1).max_depth(1);
        for i in dir.into_iter().filter_map(|e| e.ok()) {
            let pa = i.path();
            if pa.is_dir() {
                continue;
            }
            let ps = pa.display().to_string();
            if ps.contains(suffix) {
                return if suffix.eq(".json") {
                    let file_content = crate::get_file(ps.as_str());
                    if file_content.is_none() {
                        continue;
                    }
                    let root = serde_json::from_str::<serde_json::Value>(file_content?.as_str());
                    if root.is_err() {
                        continue;
                    }
                    let root = root.unwrap();
                    let libr = root["libraries"].is_array();
                    if !libr {
                        continue;
                    }
                    let mics = root["mainClass"].is_string();
                    if !mics {
                        continue;
                    }
                    let idid = root["id"].is_string();
                    if !idid {
                        continue;
                    }
                    Some(ps)
                } else {
                    Some(ps)
                };
            } else if !suffix.contains(".") {
                let sha = crate::get_sha1(ps.as_str());
                if sha.is_none() {
                    continue;
                }
                if sha?.eq(suffix) {
                    return Some(ps);
                }
            }
        }
    }
    None
}

/// 判断参数，以及拼接参数！适用于在整合参数时。
pub fn judge_arguments(args_json: String, key: &str) -> Option<Vec<String>> {
    let root = serde_json::from_str::<serde_json::Value>(args_json.as_str()).ok()?;
    let argu = root["arguments"][key].as_array()?;
    let mut res: Vec<String> = Vec::new();
    for i in argu.iter() {
        let i_str = serde_json::to_string(i);
        if i_str.is_err() {
            continue;
        }
        let i_str = i_str.unwrap();
        if i_str.contains("rules") {
            continue;
        }
        let i_str = i.as_str();
        if i_str.is_none() {
            continue;
        }
        let i_str = i_str?.replace(" ", "");
        res.push(i_str.clone());
    }
    Some(res.clone())
}

/// 单纯只是一个判断版本json里的libraries中，有rules的类库，是否allow在windows上。
/// 需要填入一个serde_json的对象Map值！而且该对象必须已经从rules中取了出来！
pub fn judge_mc_rules(root: &serde_json::Value) -> bool {
    let rules = root["rules"].as_array();
    if rules.is_none() {
        return true;
    }
    let rules = rules.unwrap();
    for i in rules.iter() {
        let action = i["action"].as_str();
        if action.is_none() {
            continue;
        }
        let action = action.unwrap();
        if action.eq("allow") {
            let os_name = i["os"]["name"].as_str();
            if os_name.is_none() {
                continue;
            }
            let os_name = os_name.unwrap();
            if !os_name.eq("windows") {
                return false;
            }
        } else if action.eq("disallow") {
            let os_name = i["os"]["name"].as_str();
            if os_name.is_none() {
                continue;
            }
            let os_name = os_name.unwrap();
            if os_name.eq("windows") {
                return false;
            }
        }
    }
    true
}

/// 获取MC类库（GetCPLibraries）
pub fn get_mc_libs(raw_json: String, root_path: &str, version_path: &str) -> Option<String> {
    let mut res = String::new();
    let mut raw_list: Vec<String> = Vec::new();
    let mut no_list: Vec<String> = Vec::new();
    let mut no_low: Vec<String> = Vec::new();
    let mut temp_list: Vec<String> = Vec::new();
    let mut no_opt: Vec<String> = Vec::new();
    let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str()).ok()?;
    let json_lib = root["libraries"].as_array()?;
    for i in json_lib.iter() {
        let name = i["name"].as_str();
        if name.is_none() {
            continue;
        }
        let expect_rule = judge_mc_rules(&i.clone());
        let mut expect_downloads = true;
        if let Some(e) = i.get("downloads") {
            if let Some(f) = e.get("classifiers") {
                if f.as_object().is_some() {
                    expect_downloads = false;
                    if let Some(g) = e.get("artifact") {
                        if g.as_object().is_some() {
                            expect_downloads = true;
                        }
                    }
                }
            }
        }
        if expect_rule && expect_downloads {
            raw_list.push(name?.to_string())
        }
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
        if toint.is_err() {
            continue;
        }
        let toint = toint.unwrap();
        if !temp_list.contains(&nonum) {
            temp_list.push(nonum);
            no_low.push(i);
        } else {
            let temp_1 = temp_list.iter().position(|x| x == &nonum);
            if temp_1.is_none() {
                continue;
            }
            let temp_2 = no_low.get(temp_1?);
            if temp_2.is_none() {
                continue;
            }
            let temp_2 = extract_number(temp_2?.to_string(), true);
            let temp_3 = temp_2.parse::<u64>();
            if temp_3.is_err() {
                continue;
            }
            let temp_3 = temp_3.unwrap();
            if temp_3 < toint {
                no_low.remove(temp_1?);
                no_low.insert(temp_1?, i);
            }
        }
    }
    //dim you! optifine!
    let mut temp: Vec<String> = Vec::new();
    for i in no_low.into_iter() {
        if i.contains("optifine") {
            temp.push(i.clone());
        } else {
            no_opt.push(i.clone());
        }
    }
    if !temp.is_empty() {
        no_opt.extend(temp.clone());
    }
    //end you! optifine!
    for i in no_opt.into_iter() {
        if let Some(e) = convert_name_to_path(i) {
            res.push_str(
                format!(
                    "{}\\libraries\\{}{}",
                    root_path, e, "${classpath_separator}"
                )
                .as_str(),
            );
        }
    }
    let mut inh = get_mc_inherits_from(version_path.to_string(), "jar")?;
    if inh.eq(version_path) {
        let inhj = get_mc_inherits_from(version_path.to_string(), "inheritsFrom")?;
        inh = inhj.clone();
    }
    let sha = root
        .get("downloads")?
        .get("client")?
        .get("sha1")?
        .as_str()?;
    let tmp = get_mc_real_path(inh, sha);
    if let Some(e) = tmp {
        res.push_str(e.as_str());
    } else {
        res = res[0..res.rfind("$")?].to_string();
    }
    Some(res)
}

/// 解压natives。填入原json和根路径和版本路径。解压成功返回true，否则返回false。
pub fn unzip_native(raw_json: String, root_path: &str, version_path: &str) -> bool {
    let mut raw_list: Vec<String> = Vec::new();
    let mut no_list: Vec<String> = Vec::new();
    let mut no_low: Vec<String> = Vec::new();
    let mut temp_list: Vec<String> = Vec::new();
    let root = serde_json::from_str::<serde_json::Value>(raw_json.as_str());
    if root.is_err() {
        return false;
    }
    let root = root.unwrap();
    let json_lib = root["libraries"].as_array();
    if json_lib.is_none() {
        return false;
    }
    let json_lib = json_lib.unwrap();
    for i in json_lib.iter() {
        let expect_rule = judge_mc_rules(&i.clone());
        let lib_name = i["name"].as_str();
        if lib_name.is_none() {
            continue;
        }
        let lib_name = lib_name.unwrap();
        let lib_arch = i["natives"]["windows"].as_str();
        if lib_arch.is_none() {
            continue;
        }
        let lib_arch = lib_arch.unwrap();
        if expect_rule {
            raw_list.push(format!("{}:{}", lib_name, lib_arch))
        }
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
        if toint.is_err() {
            continue;
        }
        let toint = toint.unwrap();
        if !temp_list.contains(&nonum) {
            temp_list.push(nonum);
            no_low.push(i);
        } else {
            let temp_1 = temp_list.iter().position(|x| x == &nonum);
            if temp_1.is_none() {
                continue;
            }
            let temp_1 = temp_1.unwrap();
            let temp_2 = no_low.get(temp_1);
            if temp_2.is_none() {
                continue;
            }
            let temp_2 = extract_number(temp_2.unwrap().to_string(), true);
            let temp_3 = temp_2.parse::<u64>();
            if temp_3.is_err() {
                continue;
            }
            let temp_3 = temp_3.unwrap();
            if temp_3 < toint {
                no_low.remove(temp_1);
                no_low.insert(temp_1, i);
            }
        }
    }
    let dir = format!(
        "{}\\{}-{}-natives",
        version_path,
        crate::extract_file_name(version_path),
        LAUNCHER_NANE
    );
    let ver_file = std::path::Path::new(dir.as_str());
    if !ver_file.exists() || (ver_file.exists() && ver_file.is_file()) {
        let cf = std::fs::create_dir_all(ver_file);
        if cf.is_err() {
            return false;
        }
    } else {
        return true;
    }
    if no_low.is_empty() {
        true
    } else {
        for c in no_low.into_iter() {
            let cvn = convert_name_to_path(c);
            if cvn.is_none() {
                continue;
            }
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

/// 自定义启动设置类，各位可以调用！
/// 其中，你需要保证以下两点最重要：
/// root_path里面包含【assets、libraries】两个文件夹
/// version_path里面包含【版本.json、版本.jar】两个文件
/// 后期解压Native是默认解压到version_path路径下的！
#[derive(Clone)]
pub struct LaunchOption {
    /// 账号类
    account: LaunchAccount,
    /// Java路径
    java_path: String,
    /// MC根路径
    root_path: String,
    /// MC版本路径
    version_path: String,
    /// MC游戏文件夹
    game_path: String,
    /// 游戏窗口高度
    window_height: usize,
    /// 游戏窗口宽度
    window_width: usize,
    /// 游戏最小内存
    min_memory: usize,
    /// 游戏最大内存
    max_memory: usize,
    /// 游戏自定义信息
    custom_info: String,
    /// 游戏额外JVM参数
    additional_jvm: String,
    /// 游戏额外game参数
    additional_game: String,
}
impl LaunchOption {
    pub fn new(
        account: LaunchAccount,
        java_path: &str,
        root_path: &str,
        version_path: &str,
        game_path: &str,
    ) -> Self {
        Self {
            account,
            java_path: java_path.to_string(),
            root_path: root_path.to_string(),
            version_path: version_path.to_string(),
            game_path: game_path.to_string(),
            window_height: 480,
            window_width: 854,
            min_memory: 256,
            max_memory: 4096,
            custom_info: format!("{}-{}", LAUNCHER_NANE, LAUNCHER_VERSION),
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
struct LaunchGame {
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
/// 启动游戏的私有实现类，如果想要调用的话，请直接使用下方的launch_game函数。
/// 如果你想自己实现启动逻辑，可以看下面启动游戏的逻辑，然后调用相对应的函数。因为除了该私有实现以外，别的函数都是pub的！
impl LaunchGame {
    fn new<F>(option: LaunchOption, callback: F) -> Self
    where
        F: Fn(Vec<&str>) + 'static,
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
    /// 启动游戏的私有函数，此处为检查是否有错。
    fn check_error(&self) -> MMCLLResult<()> {
        let event_loop = winit::event_loop::EventLoop::new();
        let monitor = event_loop.available_monitors().next();
        if monitor.is_none() {
            return Err(MMCLLError::UnknownError(864));
        }
        let monitor = monitor.unwrap();
        let window_size = monitor.size();
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let mem = (sys.total_memory() as f64 / 1024.0 / 1024.0).ceil() as i32;
        if self.account.get_online() == 0 {
            let regu = regex::Regex::new("^[0-9a-f]{32}$").unwrap();
            if !regu.is_match(self.account.get_uuid()) {
                return Err(MMCLLError::LaunchAccountUserUUID);
            }
            let regn = regex::Regex::new("^[a-zA-Z0-9]{3,16}$").unwrap();
            if !regn.is_match(self.account.get_name()) {
                return Err(MMCLLError::LaunchAccountUsername);
            }
        } else if self.account.get_online() == 1 {
            let um = crate::account::UrlMethod::new(
                "https://api.minecraftservices.com/minecraft/profile",
            );
            let ih = um.get(self.account.get_access_token());
            if ih.is_none() {
                return Err(MMCLLError::LaunchAccountAccessToken);
            }
            let json =
                serde_json::from_str::<serde_json::Value>(ih.unwrap().replace("\\/", "/").as_str());
            if json.is_err() {
                return Err(MMCLLError::UnknownError(891));
            }
            let json = json.unwrap();
            let name = json["name"].as_str();
            if name.is_none() {
                return Err(MMCLLError::LaunchAccountNoLegal);
            }
            let name = name.unwrap();
            let uuid = json["id"].as_str();
            if uuid.is_none() {
                return Err(MMCLLError::LaunchAccountNoLegal);
            }
            let uuid = uuid.unwrap();
            if name != self.account.get_name() && uuid != self.account.get_name() {
                return Err(MMCLLError::LaunchAccountAccessToken);
            }
        } else if self.account.get_online() == 2 {
            if self.account.get_base().is_empty()
                || !regex::Regex::new(
                    r"^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$",
                )
                .unwrap()
                .is_match(self.account.get_base())
            {
                return Err(MMCLLError::LaunchAccountThirdpartyBase);
            }
            let t = format!("{}/authserver/validate", self.account.get_url());
            let pl = format!(
                "{}{}{}",
                "{\"accesstoken\":\"",
                self.account.get_access_token(),
                "\"}"
            );
            let po = crate::account::UrlMethod::new(t.as_str());
            let pl = po.post(pl.as_str(), true);
            let ap = super::some_var::AUTHLIB_PATH.with_borrow(|e| e.clone());
            let ap = std::path::Path::new(ap.as_str());
            if !ap.exists() || ap.is_dir() {
                return Err(MMCLLError::LaunchAccountAuthlib);
            }
            if pl.is_none() {
                return Err(MMCLLError::LaunchAccountThirdpartyAccessTokenOrURL);
            }
        }
        let jpath = std::path::Path::new(self.java_path.as_str());
        if !jpath.exists() || !jpath.is_file() {
            return Err(MMCLLError::LaunchJavaPath);
        }
        let rpath = std::path::Path::new(self.root_path.as_str());
        if !rpath.exists() || !rpath.is_dir() {
            return Err(MMCLLError::LaunchRootPath);
        }
        let vpath = std::path::Path::new(self.version_path.as_str());
        if !vpath.exists() || !vpath.is_dir() {
            return Err(MMCLLError::LaunchVersionPath);
        }
        let gpath = std::path::Path::new(self.game_path.as_str());
        if !gpath.exists() || !gpath.is_dir() {
            return Err(MMCLLError::LaunchGamePath);
        }
        if self.window_width < 854 || self.window_width > (window_size.width as usize) {
            return Err(MMCLLError::LaunchWidth);
        }
        if self.window_height < 480 || self.window_height > (window_size.height as usize) {
            return Err(MMCLLError::LaunchHeight);
        }
        if self.min_memory > 1024 || self.min_memory < 256 {
            return Err(MMCLLError::LaunchMinMemory);
        }
        if self.max_memory < 1024 || self.max_memory > (mem as usize) {
            return Err(MMCLLError::LaunchMaxMemory);
        }
        if self.custom_info.is_empty() {
            return Err(MMCLLError::LaunchCustomInfo);
        }
        Ok(())
    }
    /// 拼接全局参数
    fn put_arguments(
        &self,
        real_json: String,
        def_jvm: String,
        defn_jvm: String,
    ) -> MMCLLResult<Vec<String>> {
        let root = serde_json::from_str::<serde_json::Value>(real_json.as_str())
            .map_err(|_| MMCLLError::GameRawJsonStructure)?;
        let mcid = root["id"]
            .as_str()
            .ok_or(MMCLLError::GameRawJsonStructure)?;
        let main_class = root["mainClass"]
            .as_str()
            .ok_or(MMCLLError::GameRawJsonStructure)?;
        let asset_index = root["assetIndex"]["id"]
            .as_str()
            .ok_or(MMCLLError::GameRawJsonStructure)?;
        let mut result: Vec<String> = Vec::new();
        let def_jvm: Vec<String> = def_jvm
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| String::from(*e))
            .collect();
        let defn_jvm: Vec<String> = defn_jvm
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| String::from(*e))
            .collect();
        result.extend(def_jvm.clone());
        result.extend(defn_jvm.clone());
        if !self.additional_jvm.is_empty() {
            let add_jvm: Vec<String> = self
                .additional_jvm
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|e| String::from(*e))
                .collect();
            result.extend(add_jvm.clone());
        }
        let judge_argu = judge_arguments(real_json.clone(), "jvm");
        if let Some(e) = judge_argu {
            result.extend(e.clone());
        } else {
            result.push(String::from("-Djava.library.path=${natives_directory}"));
            result.push(String::from("-cp"));
            result.push(String::from("${classpath}"));
        }
        if !self.account.get_base().is_empty() {
            let ap = super::some_var::AUTHLIB_PATH.with_borrow(|e| e.clone());
            if ap.is_empty() {
                panic!("You're not assign the AUTHLIB_PATH in some_var mod, please retry!")
            }
            let path = std::path::Path::new(ap.as_str());
            if path.exists() && path.is_file() {
                result.push(format!("-javaagent:{}={}", ap, self.account.get_url()));
                result.push("-Dauthlibinjector.side=client".to_string());
                result.push(format!(
                    "-Dauthlibinjector.yggdrasil.prefetched={}",
                    self.account.get_base()
                ));
            } else {
                panic!("You're AUTHLIB_PATH file is not exist, please retry!")
            }
        }
        result.push(format!("-Xmn{}m", self.min_memory));
        result.push(format!("-Xmx{}m", self.max_memory));
        result.push(main_class.to_string());
        let mcag = root.get("minecraftArguments");
        if let Some(judge_game) = mcag {
            let judge_game = judge_game.as_str();
            if let Some(judge_game) = judge_game {
                if !judge_game.is_empty() {
                    let judge_game: Vec<String> = judge_game
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .iter()
                        .map(|e| String::from(*e))
                        .collect();
                    result.extend(judge_game);
                }
            }
            if let Some(judge_game) = judge_arguments(real_json.clone(), "game") {
                result.extend(judge_game);
            }
        } else {
            result.extend(
                judge_arguments(real_json.clone(), "game")
                    .ok_or(MMCLLError::GameRawJsonStructure)?,
            );
        }
        if !self.additional_game.contains("--fullScreen") {
            result.push("--width".to_string());
            result.push(self.window_width.to_string());
            result.push("--height".to_string());
            result.push(self.window_height.to_string());
        }
        if !self.additional_game.is_empty() {
            let add_game: Vec<String> = self
                .additional_game
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|e| String::from(*e))
                .collect();
            result.extend(add_game.clone());
        }
        if result.contains(&"optifine.OptiFineForgeTweaker".to_string()) {
            let temp_1 = result
                .iter()
                .position(|x| x.eq("optifine.OptiFineForgeTweaker"))
                .unwrap();
            result.remove(temp_1 - 1);
            result.remove(temp_1 - 1);
            result.push("--tweakClass".to_string());
            result.push("optifine.OptiFineForgeTweaker".to_string());
        }
        if result.contains(&"optifine.OptiFineTweaker".to_string()) {
            let temp_1 = result
                .iter()
                .position(|x| x.eq("optifine.OptiFineTweaker"))
                .unwrap();
            result.remove(temp_1 - 1);
            result.remove(temp_1 - 1);
            result.push("--tweakClass".to_string());
            result.push("optifine.OptiFineTweaker".to_string());
        }
        let libs = get_mc_libs(
            real_json.clone(),
            self.root_path.as_str(),
            self.version_path.as_str(),
        )
        .ok_or(MMCLLError::GameRawJsonStructure)?;
        for i in result.iter_mut() {
            *i = i
                .replace(
                    "${natives_directory}",
                    format!(
                        "{}\\{}-{}-natives",
                        self.version_path,
                        crate::extract_file_name(self.version_path.as_str()),
                        LAUNCHER_NANE
                    )
                    .as_str(),
                )
                .replace("${launcher_name}", LAUNCHER_NANE)
                .replace(
                    "${launcher_version}",
                    LAUNCHER_VERSION
                        .replace(".", "")
                        .replace("-", "")
                        .replace("Alpha", "")
                        .replace("Beta", "")
                        .replace("Pre", "")
                        .as_str(),
                )
                .replace("${classpath}", libs.as_str())
                .replace("${version_name}", mcid)
                .replace(
                    "${library_directory}",
                    format!("{}\\libraries", self.root_path).as_str(),
                )
                .replace("${auth_player_name}", self.account.get_name())
                .replace("${game_directory}", self.game_path.to_string().as_str())
                .replace(
                    "${assets_root}",
                    format!("{}\\assets", self.root_path).as_str(),
                )
                .replace("${assets_index_name}", asset_index)
                .replace("${auth_uuid}", self.account.get_uuid())
                .replace("${uuid}", self.account.get_uuid())
                .replace("${auth_access_token}", self.account.get_access_token())
                .replace("${user_type}", self.account.get_atype())
                .replace("${version_type}", self.custom_info.to_string().as_str())
                .replace("${auth_session}", self.account.get_uuid())
                .replace(
                    "${game_assets}",
                    format!("{}\\assets\\virtual\\legacy", self.root_path).as_str(),
                )
                .replace("${user_properties}", "{}")
                .replace("${classpath_separator}", ";"); //MacOS 是 冒号【:】
        }
        Ok(result)
    }
    /// 如果没有错误，则会调用该函数。如果启动过程中出现不可预知的错误，则会直接panic掉！
    fn game_launch(&self) -> MMCLLResult<()> {
        let def_jvm: String = String::from("-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Dfml.ignoreInvalidMinecraftCertificates=true -Dfml.ignorePatchDiscrepancies=true -Dlog4j2.formatMsgNoLookups=true");
        let defn_jvm: String = String::from("-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump");
        let version_json_path = get_mc_real_path(self.version_path.clone(), ".json");
        if version_json_path.is_none() {
            return Err(MMCLLError::GameEligibleJsonNotFound);
        }
        let version_json_path = version_json_path.unwrap();
        let final_json: String;
        let raw_json = crate::get_file(version_json_path.as_str());
        if raw_json.is_none() {
            return Err(MMCLLError::GameEligibleJsonNotFound);
        }
        let raw_json = raw_json.unwrap();
        let inherits_json = get_mc_inherits_from(self.version_path.clone(), "inheritsFrom");
        if inherits_json.is_none() {
            return Err(MMCLLError::GameInheritsFromVersionLose);
        }
        let inherits_json = inherits_json.unwrap();
        if !inherits_json.eq(self.version_path.as_str()) {
            let file = get_mc_real_path(inherits_json, ".json");
            if file.is_none() {
                return Err(MMCLLError::GameInheritsFromVersionLose);
            }
            let file = file.unwrap();
            let f_json = crate::get_file(file.as_str());
            if f_json.is_none() {
                return Err(MMCLLError::GameInheritsFromVersionLose);
            }
            final_json = f_json.unwrap();
        } else {
            final_json = raw_json.clone();
        }
        if !unzip_native(
            final_json.clone(),
            self.root_path.as_str(),
            self.version_path.as_str(),
        ) {
            return Err(MMCLLError::GameCannotUnzipNative);
        }
        let real_json = replace_mc_inherits_from(raw_json, final_json);
        if real_json.is_none() {
            return Err(MMCLLError::GameInheritsJsonStructure);
        }
        let real_json = real_json.unwrap();
        let mut param = self.put_arguments(real_json.clone(), def_jvm.clone(), defn_jvm.clone())?;
        param.splice(0..0, [self.java_path.clone()]);
        let command = param.iter().map(AsRef::as_ref).collect();
        (self.callback)(command);
        Ok(())
    }
}

/// 提供了Account登录的启动类模块，该类不是用来登录账号的，只是用来启动游戏时才用到的！
///
/// 离线模式调用示例：LaunchAccount::new_offline("Steve", "1234567890abcdef1234567890abcdef");
/// 或：LaunchAccount::new_offline_default("Steve");  // UUID会自动按照bukkit方式生成。
/// 微软登录调用示例：LaunchAccount::new_microsoft("Steve", "1234567890abcdef1234567890abcdef", "<你的access token密钥>")
/// 第三方外置登录调用示例：LaunchAccount::new_thirdparty(
///                      "Steve",
///                      "1234567890abcdef1234567890abcdef",
///                      "<你的access token密钥>",
///                      "<你的皮肤站元数据base64编码>",
///                      "https://littleskin.cn/api/yggdrasil"")  # 皮肤站元数据必须得是精确到api/yggdrasil的！
/// 或：LaunchAccount::new_thirdparty(
///                      "Steve",
///                      "1234567890abcdef1234567890abcdef",
///                      "<你的access token密钥>",
///                      "https://littleskin.cn/api/yggdrasil"")  # 此时皮肤站元数据base64编码会自动从api密钥获取。
#[derive(Clone)]
pub struct LaunchAccount {
    /// 玩家登录名称
    name: String,
    /// 玩家登录UUID
    ///
    /// 该UUID必须符合32位16进制字符，否则会导致启动游戏失败！
    uuid: String,
    /// 登录密钥
    ///
    /// 该密钥仅在使用微软、第三方登录时才会用到
    access_token: String,
    /// 登录类型
    ///
    /// 该类型无需自己填写，仅用于标记登录类型
    atype: String,
    /// 第三方登录元数据base64编码方案
    ///
    /// 该参数仅在使用第三方登录时才会用到
    base: String,
    /// 第三方登录网址
    url: String,
    /// 登录类型标记
    ///
    /// 该参数无需自己填写，仅用于标记登录类型
    online: i32,
}
impl LaunchAccount {
    fn new(
        name: String,
        uuid: String,
        access_token: String,
        atype: String,
        base: String,
        url: String,
        online: i32,
    ) -> Self {
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
    /// 新建一个离线登录。如果你身处除中国以外的地方，请不要使用该新建函数。
    pub fn new_offline(name: &str, uuid: &str) -> Self {
        LaunchAccount::new(
            name.to_string(),
            uuid.to_string(),
            uuid.to_string(),
            String::from("Legacy"),
            String::new(),
            String::new(),
            0,
        )
    }
    #[allow(dead_code, deprecated)]
    #[deprecated(
        since = "0.0.8",
        note = "Please use main_mod generate_bukkit_uuid function."
    )]
    /// 新建一个默认的玩家，仅需输入玩家名称，使用bukkit方式生成一个UUID。
    pub fn new_offline_default(name: &str) -> Self {
        let uuid = crate::generate_bukkit_uuid(name);
        LaunchAccount::new(
            name.to_string(),
            uuid.clone(),
            uuid.clone(),
            String::from("Legacy"),
            String::new(),
            String::new(),
            0,
        )
    }
    /// 新建了一个微软登录。该登录方式适用于全世界。
    pub fn new_microsoft(name: &str, uuid: &str, access_token: &str) -> Self {
        LaunchAccount::new(
            name.to_string(),
            uuid.to_string(),
            access_token.to_string(),
            String::from("msa"),
            String::new(),
            String::new(),
            1,
        )
    }
    /// 新建了一个第三方登录。除非你信任该模块地址，否则你不能使用该新建函数。
    pub fn new_thirdparty(
        name: &str,
        uuid: &str,
        access_token: &str,
        base: &str,
        url: &str,
    ) -> Self {
        LaunchAccount::new(
            name.to_string(),
            uuid.to_string(),
            access_token.to_string(),
            String::from("msa"),
            base.to_string(),
            url.to_string(),
            2,
        )
    }
    #[allow(dead_code, deprecated)]
    #[deprecated(
        since = "0.0.8",
        note = "Please login thirdparty in account_mod, and auto get base64 code by sync."
    )]
    /// 新建了一个第三方登录。并且无需填入元数据，仅需多填入一个第三方登录网址。
    pub fn new_thirdparty_default(name: &str, uuid: &str, access_token: &str, url: &str) -> Self {
        LaunchAccount::new(
            name.to_string(),
            uuid.to_string(),
            access_token.to_string(),
            String::from("msa"),
            crate::generate_thirdparty_metadata_base64(url),
            url.to_string(),
            2,
        )
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
/// 该函数为启动游戏的函数，接受一个LaunchOption函数和一个闭包。
/// 其中，闭包用于获取启动参数。
pub fn launch_game<F>(option: LaunchOption, callback: F) -> MMCLLResult<()>
where
    F: Fn(Vec<&str>) + 'static,
{
    let res = LaunchGame::new(option, callback);
    res.check_error()?;
    res.game_launch()
}