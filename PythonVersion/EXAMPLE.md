## 这里是使用MMCLL启动游戏的一个小示例：

```python
from PyLib import *
import subprocess


def run(command: list):
    subprocess.run(command)


if __name__ == '__main__':
    try:
        account = LaunchAccount.new_offline("Steve", "1234567890abcdef1234567890abcdef")
        option = LaunchOption(account, "D:\\Java\\bin\\java.exe", "D:\\.minecraft", "D:\\.minecraft\\versions\\1.21", "D:\\.minecraft\\versions\\1.21")
        launch = launch_game(option, run)
        match launch:
            case 0:
                print("运行成功！")
            case _:
                print("运行失败，失败代码：", launch)
    except Exception as e:
        print("发生了报错！爆错内容：", e)
```