package mmcll

import (
	"crypto/md5"
	"crypto/sha1"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"reflect"
)

// ArrayIndexOf 查询切片中的元素索引
func ArrayIndexOf[T comparable](array []T, item T) int {
	for i, v := range array {
		if v == item {
			return i
		}
	}
	return -1
}

// If 仿三元运算符
func If(condition bool, trueVal, falseVal any) any {
	if condition {
		return trueVal
	}
	return falseVal
}

// SafeGet 稍微定义一个SafeGet，用于安全的获取切片索引（
// 可以像用 map[string]any 那样使用这个函数（
func SafeGet[T any](slice []T, index int) (T, bool) {
	if index >= 0 && index < len(slice) {
		return slice[index], true
	}
	var zero T // 类型零值
	return zero, false
}

// Safe 通过一个 cond JSON 对象，安全的获取到里面的值。如果获取不到或者类型不匹配，则返回defaultValue类型的值。
// cond: 传入的JSON对象
// param defaultValue 默认值
// keys 需要遍历的key，可以深度获取
func Safe(cond any, defaultValue any, keys ...any) any {
	var current = cond
out:
	for _, key := range keys {
		switch current.(type) {
		case map[string]any:
			if k, ok := key.(string); ok {
				if v, ok := current.(map[string]any)[k]; ok {
					current = v
				} else {
					return defaultValue
				}
			}
		case []any:
			if k, ok := key.(int); ok {
				if v, ok := SafeGet(current.([]any), k); ok {
					current = v
				} else {
					return defaultValue
				}
			}
		default:
			break out
		}
	}
	if reflect.TypeOf(defaultValue) != reflect.TypeOf(current) {
		return defaultValue
	}
	return current
}

// GetFile 获取文件内容
func GetFile(path string) (string, error) {
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

func GetBFile(path string) ([]byte, error) {
	return os.ReadFile(path)
}

// GetSha1 获取文件sha1
func GetSha1(path string) (string, error) {
	open, err := os.Open(path)
	if err != nil {
		return "", err
	}
	defer open.Close()
	hash := sha1.New()
	if _, err = io.Copy(hash, open); err != nil {
		return "", err
	}
	return fmt.Sprintf("%x", hash.Sum(nil)), nil
}

// GenerateBukkitUUID 生成一个 Username 通过 Bukkit 生成的标准 UUID
func GenerateBukkitUUID(username string) string {
	username = "OfflinePlayer:" + username
	data := []byte(username)
	hash := md5.Sum(data)
	hash[6] = (hash[6] & 0x0f) | 0x30
	hash[8] = (hash[8] & 0x3f) | 0x80
	return fmt.Sprintf("%x", hash)
}

// SetFile 设置文件
func SetFile(path, content string) error {
	// 确保目录存在
	dir := filepath.Dir(path)
	if dir != "" {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0755)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := file.WriteString(content); err != nil {
		return err
	}
	return nil
}

func SetBFile(path string, content []byte) error {
	dir := filepath.Dir(path)
	if dir != "" {
		if err := os.MkdirAll(dir, 0755); err != nil {
			return err
		}
	}
	file, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		return err
	}
	defer file.Close()
	if _, err := file.Write(content); err != nil {
		return err
	}
	return nil
}
