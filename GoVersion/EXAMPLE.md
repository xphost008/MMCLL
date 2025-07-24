## 这里是使用MMCLL启动游戏的一个小示例：

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

但是请切记，上述代码有一个小问题，那就是在启动1.21-forge的时候，出现的启动参数命令过长从而启动失败的bug。

各位可以将启动参数加上双引号，然后中间加个空格存成字符串保存到外部文件，之后再运行！

## 这里是使用MMCLL登录微软账号的一个小示例：

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
