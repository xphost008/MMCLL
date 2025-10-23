package mmcll

import (
	"golang.org/x/sys/windows"
)

func GetWindowsVersion() bool {
	info := windows.RtlGetVersion()
	if info == nil {
		return false
	}
	if info.MajorVersion >= 10 {
		return true
	}
	return false
}

func GetMcOs() string {
	return "windows"
}

func GetSeparator() string {
	return ";"
}
