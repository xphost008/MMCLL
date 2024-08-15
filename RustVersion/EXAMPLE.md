# 这里是使用MMCLL启动游戏的一个小示例：

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