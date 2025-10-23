package mmcll

import "fmt"

const (
	LauncherName      = "MMCLL"          // LauncherName 启动器名称（请自行修改）
	LauncherVersion   = "0.0.1-Alpha-12" // LauncherVersion 启动器版本
	LauncherUserAgent = "MMCLL/0.0.1.12" // LauncherUserAgent 启动器UserAgent，用于在网络请求时的设置
)

// ErrorMMCLL 定义报错类型
type ErrorMMCLL struct {
	code int32  // code 报错代码
	msg  string // msg 报错信息
}

func (e ErrorMMCLL) Error() string {
	return fmt.Sprintf("Err Code: %d, Err Message: %s", e.code, e.msg)
}
func NewMMCLLError(code int32, msg string) ErrorMMCLL {
	return ErrorMMCLL{code, msg}
}
func (e ErrorMMCLL) Code() int32 {
	return e.code
}
func (e ErrorMMCLL) Msg() string {
	return e.msg
}
