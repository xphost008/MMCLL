## 这里是使用MMCLL启动游戏的一个小示例：

```rust
use rust_lib::launcher_mod::*;
pub fn start_launch(){
    let catch = std::panic::catch_unwind(|| {
    	let option = LauncherOption::new(
    		AccountLogin::new_offline_default("SSSteve"),
    		"D:\\Java\\bin\\java.exe",
    		"D:\\.minecraft",
    		"D:\\.minecraft\\versions\\1.21",
            "D:\\.minecraft\\versions\\1.21"
    	);
        let launch = launch_game(option, |command: Vec<&str>| {
            println!("参数拼接成功！正在为您启动游戏！");
            let cmd = std::process::Command::new("cmd")
                .arg("/c")
                .args(command)
                .spawn();
            if let Ok(mut o) = cmd {
                if let Ok(e) = o.wait() {
                    if let Some(f) = e.code() {
                        println!("程序已退出，退出代码为：{}", f);
                    }
                }
            }
        });
        if let Err(e) = launch {
            match e {
                -1 => {
                    println!("账号名称格式错误！")
                }
                -2 => {
                    println!("账号UUID格式错误")
                }
                -3 => {
                    println!("账号AccessToken错误")
                }
                -4 => {
                    println!("账号未购买正版")
                }
                -5 => {
                    println!("账号第三方的AccessToken或者URL错误。")
                }
                -6 => {
                    println!("账号base64编码错误")
                }
                -7 => {
                    println!("Java路径错误（文件未找到）")
                }
                -8 => {
                    println!("游戏根路径错误（文件夹未找到）")
                }
                -9 => {
                    println!("游戏版本路径错误（文件夹未找到）")
                }
                -10 => {
                    println!("游戏实际路径错误（文件夹未找到）")
                }
                -11 => {
                    println!("窗口宽度错误（小于854或大于屏幕宽度）")
                }
                -12 => {
                    println!("窗口高度错误（小于480或大于屏幕高度）")
                }
                -13 => {
                    println!("最大或最小内存错误（最大内存小于1024或大于系统内存 或 最小内存小于256或大于1024）")
                }
                -14 => {
                    println!("自定义信息错误（未填写，必须要个默认值！）")
                }
                _ => {
                    println!("出现了未知错误！请立刻联系TLM开发者！")
                }
            }
        }
    });
    if let Err(e) = catch {
        let cp = e.downcast_ref::<&str>();
        println!("启动时参数检测无误，但是拼接参数时出现了错误！错误提示：\n{}", cp.unwrap());
    }
}
```

但是请切记，上述代码有一个小问题，那就是在启动1.21-forge的时候，出现的启动参数命令过长从而启动失败的bug。

各位可以将启动参数加上双引号，然后中间加个空格存成字符串保存到外部文件，之后再运行！

## 这里是使用MMCLL登录账号的一个小示例：

```rust
pub fn login_microsoft(){
    //使用tokio执行异步程序，但是阻塞了主线程。
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let login = AccountLogin::new("<你的微软client_id>");
        //此时应该会返回Err(-1)错误，未能获取用户代码，请自行解决。
        let (user_code, device_code) = login.get_user_code().await.unwrap();
        println!("请复制你的用户代码，并将其粘贴到浏览器上：{}", user_code);
        let mut cb: clipboard::ClipboardContext = clipboard::ClipboardProvider::new().unwrap();
        cb.set_contents(user_code.to_owned()).unwrap();
        std::process::Command::new("explorer.exe").arg("https://www.microsoft.com/link").spawn().expect("Some Error appear!");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let s = login.login_microsoft(device_code.clone()).await;
            match s {
                Ok(e) => {
                    //这里仅打印输出name、uuid、access_token、refresh_token，如果想应用请自行弄。
                    println!("{}\n{}\n{}\n{}",e.get_name(), e.get_uuid(), e.get_access_token(), e.get_refresh_token());
                    break;
                },
                Err(e) => {
                    match e {
                        // -1错误是在获取用户代码时出现的错误，这里暂时不用管。
                        // -2错误是暂未完成登录，重新开始一次循环。因此不用捕获。
                        -3 => {
                            println!("登录超时（15分钟未完成登录），请重试！");
                            break;
                        },  
                        // -4错误是刷新账号时出现的错误，这里不用捕获。
                        -5 => {
                            println!("在进行xbox登录时出现了错误，可能是没挂vβn的原因。");
                            break;
                        },
                        -6 => {
                            println!("在进行xsts登录时出现了错误，可能是没挂vβn的原因。");
                            break;
                        },
                        -7 => {
                            println!("在进行xsts登录时，由于该账户没有xbox账号，你可能需要自己注册一个。");
                            break;
                        },
                        -8 => {
                            println!("在进行xsts登录时，由于该国家/禁止被禁止，无法登录。");
                            break;
                        },
                        -9 => {
                            println!("该账号需要成人验证（韩国）。");
                            break;
                        },
                        -10 => {
                            println!("该账号设置未满18周岁，需要成人将该账户添加到家庭组中。");
                            break;
                        },
                        -11 => {
                            println!("你请求的xbox usercode与xsts usercode二者不一致，请重新尝试！");
                            break;
                        },
                        -12 => {
                            println!("在进行mc登录时出现了错误，可能是没挂vβn的原因。");
                            break;
                        },
                        -13 => {
                            println!("该账号暂未购买mc，请重新尝试！");
                            break;
                        }
                        _ => {
                            println!("出现了未知错误，请立即反馈给作者！错误代码：{}", e);
                            break;
                        }
                    }
                }
            }
        }
    });
}
```