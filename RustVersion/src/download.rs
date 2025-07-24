use crate::some_var::is_download_from_official;
use crate::{MMCLLError, MMCLLResult};

use serde_json::json;

/// 获取MC版本（可以使用该值赋值给MC_ROOT_JSON）
pub async fn get_mc_versions() -> MMCLLResult<serde_json::Value> {
    let v = if is_download_from_official() {
        "https://piston-meta.mojang.com/mc/game/version_manifest.json"
    } else {
        "https://bmclapi2.bangbang93.com/mc/game/version_manifest.json"
    };
    let md = String::from_utf8(
        crate::account::UrlMethod::new(v)
            .get_default_async()
            .await
            .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
    )
    .map_err(|_| 1)?;
    let sj = serde_json::from_str::<serde_json::Value>(md.as_str()).map_err(|_| 2)?;
    Ok(sj.clone())
}

/// 获取Forge版本的JSON（无论BMCLAPI还是Official，最终都会转成一种标准TLM格式：）
/// 具体格式请见：README.md
pub async fn get_forge_versions(mcversion: &str) -> MMCLLResult<serde_json::Value> {
    if !is_download_from_official() {
        let md = String::from_utf8(
            crate::account::UrlMethod::new(
                format!(
                    "https://bmclapi2.bangbang93.com/forge/minecraft/{}",
                    mcversion
                )
                .as_str(),
            )
            .get_default_async()
            .await
            .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
        )
        .map_err(|_| 3)?;
        let sj = serde_json::from_str::<serde_json::Value>(md.as_str())
            .map_err(|_| MMCLLError::DownloadForgeVersionNotFound)?;
        let sj = sj
            .as_array()
            .ok_or(MMCLLError::DownloadForgeVersionNotFound)?;
        if sj.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        let mut version = Vec::with_capacity(sj.len());
        for a_file in sj.iter() {
            let mcv = a_file["mcversion"].as_str().ok_or(4)?;
            let v = a_file["version"].as_str().ok_or(5)?;
            let bch = a_file["branch"].as_str().unwrap_or("");
            let raw = [mcv, v, bch]
                .iter()
                .filter(|e| !e.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("-");
            let need_insert_jar = {
                if let Some(files) = a_file["files"].as_array() {
                    files.iter().any(|j| {
                        let cg = j["category"]
                            .as_str()
                            .map(|e| e == "installer")
                            .unwrap_or(false);
                        let fm = j["format"].as_str().map(|e| e == "jar").unwrap_or(false);
                        cg && fm
                    })
                } else {
                    false
                }
            };
            let installer: serde_json::Value = if need_insert_jar {
                let ins = if bch.is_empty() {
                    format!(
                        "https://bmclapi2.bangbang93.com/forge/download?mcversion={}&version={}&category=installer&format=jar",
                        mcv, v
                    )
                } else {
                    format!(
                        "https://bmclapi2.bangbang93.com/forge/download?mcversion={}&version={}&branch={}&category=installer&format=jar",
                        mcv, v, bch
                    )
                };
                serde_json::Value::String(ins)
            } else {
                serde_json::Value::Null
            };
            version.push(json! {{
                "mcversion": mcv,
                "version": v,
                "rawversion": raw,
                "installer": installer
            }});
        }
        let res_data = json!({
            "forge": [version]
        });
        Ok(res_data)
    } else {
        let md = String::from_utf8(
            crate::account::UrlMethod::new(
                "https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml",
            )
            .get_default_async()
            .await
            .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
        )
        .map_err(|_| 1)?;
        let mut sj = quick_xml::Reader::from_str(md.as_str());
        sj.config_mut().trim_text(true);
        let mut res = serde_json::from_str::<serde_json::Value>("{\"forge\":[]}").unwrap();
        let mut obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let obj = obj.as_object_mut().unwrap();
        let rv = res.get_mut("forge").unwrap().as_array_mut().unwrap();
        let mut versioning = false;
        let mut versions = false;
        let mut version = false;
        let mut buf: Vec<u8> = Vec::new();
        loop {
            match sj.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Start(e)) => match e.name().as_ref() {
                    b"versioning" => versioning = true,
                    b"versions" if versioning => versions = true,
                    b"version" if versions => version = true,
                    _ => (),
                },
                Ok(quick_xml::events::Event::End(e)) => match e.name().as_ref() {
                    b"versioning" => versioning = false,
                    b"versions" => versions = false,
                    b"version" => version = false,
                    _ => (),
                },
                Ok(quick_xml::events::Event::Text(e)) => {
                    if versioning && versions && version {
                        let text = e.unescape().map_err(|_| 10)?.into_owned();
                        let sp = text.split('-').collect::<Vec<&str>>();
                        if (sp.len() == 2 || sp.len() == 3) && sp[0].eq(mcversion) {
                            obj.insert(
                                String::from("mcversion"),
                                serde_json::Value::String(sp[0].to_string()),
                            );
                            obj.insert(
                                String::from("version"),
                                serde_json::Value::String(sp[1].to_string()),
                            );
                            obj.insert(
                                String::from("rawversion"),
                                serde_json::Value::String(text.clone()),
                            );
                            let ins = format!("https://maven.minecraftforge.net/net/minecraftforge/forge/{}/forge-{}-installer.jar", text, text);
                            obj.insert(
                                String::from("installer"),
                                serde_json::Value::String(ins.clone()),
                            );
                            rv.push(serde_json::Value::Object(obj.clone()));
                            obj.clear();
                        }
                    }
                }
                Ok(quick_xml::events::Event::Eof) => break,
                _ => (),
            }
        }
        if rv.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        Ok(res.clone())
    }
}

/// 获取fabric版本的TLM实现版JSON
pub async fn get_fabric_version(mcversion: &str) -> MMCLLResult<serde_json::Value> {
    let meta = format!(
        "{}/{}",
        if is_download_from_official() {
            "https://meta.fabricmc.net/v2/versions/loader"
        } else {
            "https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader"
        },
        mcversion
    );
    let url = crate::account::UrlMethod::new(meta.as_str());
    let text = String::from_utf8(
        url.get_default_async()
            .await
            .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
    )
    .map_err(|_| 12)?;
    let ser = serde_json::from_str::<serde_json::Value>(text.as_str())
        .map_err(|_| MMCLLError::DownloadFabricVersionNotFound)?;
    let mut res = serde_json::from_str::<serde_json::Value>("{\"fabric\":[]}").unwrap();
    let mut obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    let obj = obj.as_object_mut().unwrap();
    let rv = res.get_mut("fabric").unwrap().as_array_mut().unwrap();
    if !ser.is_array() {
        Err(MMCLLError::DownloadQuiltVersionNotFound)
    } else {
        let ser = ser.as_array().unwrap();
        for i in ser.iter() {
            let c = i["loader"]["version"].as_str();
            if c.is_none() {
                continue;
            }
            let c = c.unwrap();
            obj.insert(
                String::from("rawversion"),
                serde_json::Value::String(String::from(c)),
            );
            obj.insert(
                String::from("mcversion"),
                serde_json::Value::String(String::from(mcversion)),
            );
            obj.insert(
                String::from("version"),
                serde_json::Value::String(String::from(c)),
            );
            obj.insert(
                String::from("profile"),
                serde_json::Value::String(format!(
                    "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
                    mcversion, c
                )),
            );
            rv.push(serde_json::Value::Object(obj.clone()));
            obj.clear();
        }
        if rv.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        Ok(res.clone())
    }
}

/// 获取quilt版本的TLM实现版JSON
pub async fn get_quilt_version(mcversion: &str) -> MMCLLResult<serde_json::Value> {
    let meta = format!(
        "{}/{}",
        if is_download_from_official() {
            "https://meta.quiltmc.org/v3/versions/loader"
        } else {
            "https://bmclapi2.bangbang93.com/quilt-meta/v3/versions/loader"
        },
        mcversion
    );
    let url = crate::account::UrlMethod::new(meta.as_str());
    let text = String::from_utf8(
        url.get_default_async()
            .await
            .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
    )
    .map_err(|_| 13)?;
    let ser = serde_json::from_str::<serde_json::Value>(text.as_str())
        .map_err(|_| MMCLLError::DownloadQuiltVersionNotFound)?;
    let mut res = serde_json::from_str::<serde_json::Value>("{\"quilt\":[]}").unwrap();
    let mut obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    let obj = obj.as_object_mut().unwrap();
    let rv = res.get_mut("quilt").unwrap().as_array_mut().unwrap();
    if ser.is_object() {
        Err(MMCLLError::DownloadQuiltVersionNotFound)
    } else {
        let ser = ser.as_array().unwrap();
        for i in ser.iter() {
            let c = i["loader"]["version"].as_str();
            if c.is_none() {
                continue;
            }
            let c = c.unwrap();
            obj.insert(
                String::from("rawversion"),
                serde_json::Value::String(String::from(c)),
            );
            obj.insert(
                String::from("mcversion"),
                serde_json::Value::String(String::from(mcversion)),
            );
            obj.insert(
                String::from("version"),
                serde_json::Value::String(String::from(c)),
            );
            obj.insert(
                String::from("profile"),
                serde_json::Value::String(format!(
                    "https://meta.quiltmc.org/v3/versions/loader/{}/{}/profile/json",
                    mcversion, c
                )),
            );
            rv.push(serde_json::Value::Object(obj.clone()));
            obj.clear();
        }
        if rv.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        Ok(res.clone())
    }
}

/// 获取neoforge版本的TLM实现版JSON
pub async fn get_neoforge_version(mcversion: &str) -> MMCLLResult<serde_json::Value> {
    if mcversion == "1.20.1" {
        let meta = if is_download_from_official() {
            "https://bmclapi2.bangbang93.com/neoforge/meta/api/maven/details/releases/net/neoforged/forge"
        } else {
            "https://maven.neoforged.net/api/maven/details/releases/net/neoforged/forge"
        };
        let url = crate::account::UrlMethod::new(meta);
        let text = String::from_utf8(
            url.get_default_async()
                .await
                .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
        )
        .map_err(|_| 14)?;
        let ser = serde_json::from_str::<serde_json::Value>(text.as_str())
            .map_err(|_| MMCLLError::DownloadNeoforgeVersionNotFound)?;
        let mut res = serde_json::from_str::<serde_json::Value>("{\"neoforge\":[]}").unwrap();
        let mut obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let obj = obj.as_object_mut().unwrap();
        let rv = res.get_mut("neoforge").unwrap().as_array_mut().unwrap();
        let ser = ser["files"].as_array().ok_or(15)?;
        for i in ser.iter() {
            let name = i["name"].as_str().ok_or(16)?;
            let spn = name.split('-').collect::<Vec<&str>>();
            if spn.len() == 2 {
                let spl = spn[0].split(".").collect::<Vec<&str>>();
                if spl.len() != 3 {
                    continue;
                }
                let mv = spn[0].to_string();
                let v = spn[1].to_string();
                let n = name.to_string();
                obj.insert(
                    String::from("rawversion"),
                    serde_json::Value::String(n.clone()),
                );
                obj.insert(
                    String::from("mcversion"),
                    serde_json::Value::String(mv.clone()),
                );
                obj.insert(
                    String::from("version"),
                    serde_json::Value::String(v.clone()),
                );
                obj.insert(String::from("installer"), serde_json::Value::String((if is_download_from_official() {
                    format!("https://maven.neoforged.net/releases/net/neoforged/forge/{}/forge-{}-installer.jar", n.clone(), n.clone())
                } else {
                    format!("https://bmclapi2.bangbang93.com/neoforge/version/{}/download/installer.jar", n.clone())
                }).to_string()));
                rv.push(serde_json::Value::Object(obj.clone()));
                obj.clear();
            }
        }
        if rv.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        Ok(res.clone())
    } else {
        let meta = if is_download_from_official() {
            "https://bmclapi2.bangbang93.com/neoforge/meta/api/maven/details/releases/net/neoforged/neoforge"
        } else {
            "https://maven.neoforged.net/api/maven/details/releases/net/neoforged/neoforge"
        };
        let url = crate::account::UrlMethod::new(meta);
        let text = String::from_utf8(
            url.get_default_async()
                .await
                .ok_or(MMCLLError::DownloadCannotGetMetadata)?,
        )
        .map_err(|_| 14)?;
        let ser = serde_json::from_str::<serde_json::Value>(text.as_str())
            .map_err(|_| MMCLLError::DownloadNeoforgeVersionNotFound)?;
        let mut res = serde_json::from_str::<serde_json::Value>("{\"neoforge\":[]}").unwrap();
        let mut obj = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        let obj = obj.as_object_mut().unwrap();
        let rv = res.get_mut("neoforge").unwrap().as_array_mut().unwrap();
        let ser = ser["files"].as_array().ok_or(15)?;
        for i in ser.iter() {
            let name = i["name"].as_str().ok_or(16)?;
            let srn = name.split("-").collect::<Vec<&str>>();
            let sname = srn[0];
            let spm = mcversion.split(".").collect::<Vec<&str>>();
            let rpm = if spm.len() == 3 {
                format!("{}.{}", spm[1], spm[2])
            } else if spm.len() == 2 {
                format!("{}.0", spm[1])
            } else {
                continue;
            };
            let spn = sname.split(".").collect::<Vec<&str>>();
            let rpn = if spn.len() >= 2 {
                format!("{}.{}", spn[0], spn[1])
            } else {
                continue;
            };
            if rpm.eq(rpn.as_str()) {
                let n = name.to_string();
                let mv = mcversion.to_string();
                let v = sname.to_string();
                obj.insert(
                    String::from("rawversion"),
                    serde_json::Value::String(n.clone()),
                );
                obj.insert(
                    String::from("mcversion"),
                    serde_json::Value::String(mv.clone()),
                );
                obj.insert(
                    String::from("version"),
                    serde_json::Value::String(v.clone()),
                );
                obj.insert(String::from("installer"), serde_json::Value::String((if is_download_from_official() {
                    format!("https://maven.neoforged.net/releases/net/neoforged/forge/{}/forge-{}-installer.jar", n.clone(), n.clone())
                } else {
                    format!("https://bmclapi2.bangbang93.com/neoforge/version/{}/download/installer.jar", n.clone())
                }).to_string()));
                rv.push(serde_json::Value::Object(obj.clone()));
                obj.clear();
            }
        }
        if rv.is_empty() {
            return Err(MMCLLError::DownloadForgeVersionNotFound);
        }
        Ok(res.clone())
    }
}

/// 这个函数是专门用来下载文件的，但是是私有的。
async fn download_as_window(
    savepath: &str,
    download_url: &str,
    file_hash: &str,
) -> MMCLLResult<()> {
    let file_path = std::path::Path::new(savepath);
    if file_path.exists() {
        if file_hash.is_empty() {
            return Err(MMCLLError::DownloadFileExists);
        }
        let real_file_hash = crate::get_sha1(savepath);
        if real_file_hash.is_none() {
            return Err(MMCLLError::DownloadFileExists);
        }
        let real_file_hash = real_file_hash.unwrap();
        if real_file_hash.ne(file_hash) {
            if !crate::delete_file(savepath) {
                return Err(MMCLLError::DownloadFileExists);
            }
        } else {
            return Err(MMCLLError::DownloadFileExists);
        }
    } else {
        let parent = file_path.parent();
        if parent.is_none() {
            return Err(MMCLLError::DownloadCannotCreateDir);
        }
        let rt = std::fs::create_dir_all(file_path.parent().unwrap());
        if rt.is_err() {
            return Err(MMCLLError::DownloadCannotCreateDir);
        }
    }
    if download_url.contains("linux")
        || download_url.contains("macos")
        || download_url.contains("osx")
    {
        return Err(MMCLLError::DownloadNotSupportSystem);
    }
    let url = crate::account::UrlMethod::new(download_url);
    let url = url.get_default_async().await;
    if url.is_none() {
        return Err(MMCLLError::DownloadFileDownloadFailure);
    }
    let url = url.unwrap();
    if !file_path.exists() {
        super::set_file_vecu8(savepath, &url);
        Ok(())
    } else {
        Err(MMCLLError::DownloadFileExists)
    }
}

/// 一个有关下载任务的一个结构体，里面存了save_path、download_url、file_hash等字段
#[derive(Clone)]
struct DownloadTask {
    save_path: String,
    download_url: String,
    file_hash: String,
}

/// 一个有关下载任务的实现，里面仅实现了contains，用于判断值是否相等。如果有任何一个值相等相等，则返回true~
trait DownloadTaskImpl {
    fn contains(&self, has: &DownloadTask) -> bool;
}
impl DownloadTaskImpl for Vec<DownloadTask> {
    fn contains(&self, x: &DownloadTask) -> bool {
        for i in self.iter() {
            if i.download_url.eq(x.download_url.as_str())
                || i.file_hash.eq(x.file_hash.as_str())
                || i.save_path.eq(x.save_path.as_str())
            {
                return true;
            }
        }
        false
    }
}
#[derive(Debug, Clone)]
pub struct DownloadMethod {
    savepath: String,
}

/// 会自动匹配下载源，只需要提前将DOWNLOAD_SOURCE赋值完毕即可。
/// 1：官方源、2：BMCLAPI
/// 目前有且仅有上述两个源
/// 在下载自定义函数的时候，仅需使用tokio运行一次该函数即可。
/// 因为该库会自动调用最大线程进行切割文件并进行下载。
/// 当然，如果你想自己实现多线程下载的话，也可以调用UrlMethod的get_default_async自主实现多异步下载。、
/// 除了Rust语言有async异步以外，Python、Go等语言都有异步模型，因此均采用多异步而不是多线程进行下载。
/// 除了某些语言可能没有异步，例如Delphi/Object Pascal仅有的Task、Thread等。
impl DownloadMethod {
    /// 新建一个下载实现。savepath根据需求填入
    /// 参见以下各种new实例
    pub fn new(savepath: &str) -> Self {
        Self {
            savepath: savepath.to_string(),
        }
    }

    /// 该函数使用的是安装Minecraft的类库。并且实时显示回显在callback里。
    /// savepath需要填入类似于.minecraft文件夹的位置。
    /// raw_json需要填入json源文件~
    /// callback填一个闭包，用于回显下载进度。（闭包第一个值是下载路径，第二个值是下载进度，第三个值是下载是否失败。）
    pub async fn download_minecraft_libraries<T>(
        &self,
        raw_json: String,
        callback: T,
    ) -> MMCLLResult<()>
    where
        T: Fn(String, usize, MMCLLResult<()>) + Send + std::marker::Copy + 'static,
    {
        let libs = serde_json::from_str::<serde_json::Value>(raw_json.as_str())
            .map_err(|_| MMCLLError::DownloadArgumentsError)?;
        let libs = libs["libraries"]
            .as_array()
            .ok_or(MMCLLError::DownloadArgumentsError)?;
        let mut lib_vec: Vec<DownloadTask> = Vec::new();
        for m in 0..libs.len() {
            let i = libs[m].clone();
            if !crate::launcher::judge_mc_rules(&i.clone()) {
                continue;
            }
            let dn = i.get("downloads");
            if let Some(dn) = dn {
                let da = dn.get("artifact");
                if let Some(da) = da {
                    let sap = da["path"].as_str();
                    if let Some(sap) = sap {
                        let sha = da["sha1"].as_str();
                        if let Some(sha) = sha {
                            let url = da["url"].as_str();
                            if let Some(url) = url {
                                let sapth = self.savepath.clone()
                                    + "\\libraries\\"
                                    + sap.replace("/", "\\").as_str();
                                lib_vec.push(DownloadTask {
                                    save_path: sapth.clone(),
                                    download_url: if is_download_from_official() {
                                        url.to_string()
                                    } else {
                                        url.replace(
                                            "https://libraries.minecraft.net/",
                                            "https://bmclapi2.bangbang93.com/maven/",
                                        )
                                    },
                                    file_hash: sha.to_string(),
                                });
                            }
                        }
                    }
                }
                let dc = dn.get("classifiers");
                if let Some(dc) = dc {
                    let dnw = dc.get("natives-windows");
                    if let Some(dnw) = dnw {
                        let sap = dnw["path"].as_str();
                        if let Some(sap) = sap {
                            let sha = dnw["sha1"].as_str();
                            if let Some(sha) = sha {
                                let url = dnw["url"].as_str();
                                if let Some(url) = url {
                                    let sapth = self.savepath.clone()
                                        + "\\libraries\\"
                                        + sap.replace("/", "\\").as_str();
                                    lib_vec.push(DownloadTask {
                                        save_path: sapth.clone(),
                                        download_url: if is_download_from_official() {
                                            url.to_string()
                                        } else {
                                            url.replace(
                                                "https://libraries.minecraft.net/",
                                                "https://bmclapi2.bangbang93.com/maven/",
                                            )
                                        },
                                        file_hash: sha.to_string(),
                                    });
                                }
                            }
                        }
                    }
                    let dnwx = dc.get("natives-windows-x64");
                    if let Some(dnwx) = dnwx {
                        let sap = dnwx["path"].as_str();
                        if let Some(sap) = sap {
                            let sha = dnwx["sha1"].as_str();
                            if let Some(sha) = sha {
                                let url = dnwx["url"].as_str();
                                if let Some(url) = url {
                                    let sapth = self.savepath.clone()
                                        + "\\libraries\\"
                                        + sap.replace("/", "\\").as_str();
                                    lib_vec.push(DownloadTask {
                                        save_path: sapth.clone(),
                                        download_url: if is_download_from_official() {
                                            url.to_string()
                                        } else {
                                            url.replace(
                                                "https://libraries.minecraft.net/",
                                                "https://bmclapi2.bangbang93.com/maven/",
                                            )
                                        },
                                        file_hash: sha.to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
            let name = i["name"].as_str();
            if name.is_none() {
                continue;
            }
            let mut name = name.unwrap().to_string();
            if name.is_empty() {
                continue;
            }
            if name.contains("linux")
                || name.contains("macos")
                || name.contains("osx")
                || name.contains("x86")
            {
                continue;
            }
            let real_native = i["natives"]["windows"].as_str();
            if let Some(e) = real_native {
                name = format!("{}:{}", name, e);
            }
            let real_path = crate::launcher::convert_name_to_path(name.to_string());
            if real_path.is_none() {
                continue;
            }
            let real_path = real_path.unwrap();
            let real_save = format!("{}\\libraries\\{}", self.savepath, real_path);
            let real_url = if let Some(e) = i["url"].as_str() {
                if !e.ends_with("/") {
                    format!("{}/{}", e, real_path.replace("\\", "/"))
                } else {
                    format!("{}{}", e, real_path.replace("\\", "/"))
                }
            } else {
                format!(
                    "{}/{}",
                    if is_download_from_official() {
                        "https://libraries.minecraft.net"
                    } else {
                        "https://bmclapi2.bangbang93.com/maven"
                    },
                    real_path.replace("\\", "/")
                )
            };
            let sha1 = i["sha1"].as_str().unwrap_or_default();
            let t = DownloadTask {
                save_path: real_save.clone(),
                download_url: real_url.clone(),
                file_hash: sha1.to_string(),
            };
            if lib_vec.contains(&t) {
                continue;
            }
            lib_vec.push(t);
        }
        //以上为lib_vec赋值了，现在lib_vec里面存放了所有来自libraries的DownloadTask实例~
        let bgt = super::some_var::BIGGEST_THREAD.with_borrow(|e| *e);
        let bgt = if !(1..=256).contains(&bgt) {
            32_usize
        } else {
            bgt as usize
        };
        let bgt = if bgt < lib_vec.len() {
            bgt
        } else {
            lib_vec.len()
        };
        let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(bgt));
        for (index, lib) in lib_vec.into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let permit = semaphore.acquire_owned().await.unwrap();
            let task = tokio::spawn(async move {
                let f = download_as_window(
                    lib.save_path.as_str(),
                    lib.download_url.as_str(),
                    lib.file_hash.as_str(),
                )
                .await;
                if let Err(e) = f {
                    callback(lib.download_url, index, Err(e));
                } else {
                    callback(lib.download_url, index, Ok(()));
                }
                drop(permit);
            });
            tasks.push(task);
        }
        // 以下代码均为自己思考，并且均弃用了方法~
        // 直到我用了deepseek之后，我才能使用信号量来限制最大线程数量。
        // let mut tdl_count = 0;
        // let mut tvl_count = 0;
        // for i in 0..bgt {
        //     let lib_vec = lib_vec.clone();
        //     tasks.push(tokio::task::spawn( async move {
        //         while tdl_count < lib_vec.len() {
        //             let download_url = lib_vec[tdl_count].download_url.clone();
        //             let file_hash = lib_vec[tdl_count].file_hash.clone();
        //             let save_path = lib_vec[tdl_count].save_path.clone();
        //             let index = lib_vec[tdl_count].index.clone();
        //             tdl_count += 1;
        //             let f = download_as_window(save_path.as_str(), download_url.as_str(), file_hash.as_str()).await;
        //             tvl_count += 1;
        //             if let Err(e) = f {
        //                 callback(download_url, tvl_count, e);
        //             }else{
        //                 callback(download_url, tvl_count, OK);
        //             }
        //         }
        //         // Self::run_download(i, callback, lib_vec).await;
        //         // tdl_count += 1;
        //         // download_as_window(lib_vec[tvl_count].save_path.as_str(), lib_vec[tvl_count].download_url.as_str(), lib_vec[tvl_count].file_hash.as_str()).await;
        //         // tvl_count += 1;
        //         // callback(lib_vec[tvl_count].download_url.clone(), tvl_count, OK);
        //     }));
        // }
        // for (index, dt) in lib_vec.into_iter().enumerate() {
        //     while tasks.len() >= bgt {
        //         for i in 0..tasks.len() {
        //             if tasks[i].is_finished() {
        //                 tasks.remove(i);
        //                 break;
        //             }
        //         }
        //     }
        //     // if tasks.len() > bgt {
        //     //     if let Ok(e) = tasks.pop().unwrap().await {
        //     //         callback(e.0, e.1, e.2);
        //     //     }else{
        //     //         return Err(ERR_DOWNLOAD_DOWNLOAD_FAILURE);
        //     //     }
        //     // }
        //     let task = tokio::task::spawn(async move {
        //         let index = index.clone();
        //         let dt = dt.clone();
        //         let f = download_as_window(dt.save_path.as_str(), dt.download_url.as_str(), dt.file_hash.as_str()).await;
        //         if let Err(e) = f {
        //             callback(dt.save_path, index, e);
        //         }else{
        //             callback(dt.save_path, index, OK);
        //         }
        //     });
        //     tasks.push(task);
        // }
        for task in tasks.into_iter() {
            if let Err(_) = task.await {
                return Err(MMCLLError::DownloadDownloadFailure);
            }
        }
        Ok(())
    }

    /// 该函数使用的是安装Minecraft的资源文件。并且实时显示回显在callback里。
    /// savepath需要填入类似于.minecraft文件夹的位置。
    /// raw_json需要填入assets json源文件~（就是一堆hash的那个json）
    /// callback填一个闭包，用于回显下载进度。（闭包第一个值是下载路径，第二个值是下载进度，第三个值是下载是否失败。）
    pub async fn download_minecraft_assets<T>(
        &self,
        raw_json: String,
        callback: T,
    ) -> MMCLLResult<()>
    where
        T: Fn(String, usize, MMCLLResult<()>) + Send + std::marker::Copy + 'static,
    {
        let libs = serde_json::from_str::<serde_json::Value>(raw_json.as_str())
            .map_err(|_| MMCLLError::DownloadArgumentsError)?;
        let libs = libs["objects"]
            .as_object()
            .ok_or(MMCLLError::DownloadArgumentsError)?;
        let mut lib_vec: Vec<DownloadTask> = Vec::new();
        for i in libs.keys() {
            let dh = libs[i]["hash"].as_str();
            if let Some(dh) = dh {
                let sub = &dh[0..2];
                lib_vec.push(DownloadTask {
                    save_path: format!("{}\\assets\\objects\\{}\\{}", self.savepath, sub, dh),
                    download_url: format!(
                        "{}/{}/{}",
                        if is_download_from_official() {
                            "https://resources.download.minecraft.net"
                        } else {
                            "https://bmclapi2.bangbang93.com/assets"
                        },
                        sub,
                        dh
                    ),
                    file_hash: dh.to_string(),
                });
            }
        }
        let bgt = super::some_var::BIGGEST_THREAD.with_borrow(|e| *e);
        let bgt = if !(1..=256).contains(&bgt) {
            32_usize
        } else {
            bgt as usize
        };
        let bgt = if bgt < lib_vec.len() {
            bgt
        } else {
            lib_vec.len()
        };
        let mut tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(bgt));
        for (index, lib) in lib_vec.into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let permit = semaphore.acquire_owned().await.unwrap();
            let task = tokio::spawn(async move {
                let f = download_as_window(
                    lib.save_path.as_str(),
                    lib.download_url.as_str(),
                    lib.file_hash.as_str(),
                )
                .await;
                if let Err(e) = f {
                    callback(lib.download_url, index, Err(e));
                } else {
                    callback(lib.download_url, index, Ok(()));
                }
                drop(permit);
            });
            tasks.push(task);
        }
        for task in tasks.into_iter() {
            if let Err(_) = task.await {
                return Err(MMCLLError::DownloadDownloadFailure);
            }
        }
        Ok(())
    }
}