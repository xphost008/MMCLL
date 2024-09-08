# 这里是Rust版本的MMCLL！

## 使用须知：

1. 本包下存放了rust_lib.rs、Cargo.toml两个文件。其中Cargo.toml是依赖文件，各位可以复制里面的内容。
2. 如果没有必要，请不要修改rust_lib.rs的内容。如果你有疑问，请先向作者提出issue后再尝试修改里面。
3. 在你的启动器进行发布时，我希望各位能在软件的某一处地方对该库进行鸣谢！

## 项目依赖：

1. 该项目所使用的部分依赖，均可能需要对其作者进行鸣谢。你可以查看Cargo.toml进行查看里面所使用的类库。
2. 该项目所使用的部分依赖，可能不允许商业闭源，或者是需要使用同样的协议进行开源发布。请各位自行斟酌。

## [项目函数使用方法](./SPECIFIC.md)

## [项目使用示例](./EXAMPLE.md)

# 对于任何一个的下载源处理，这里都会转成一种标准TLM格式。目前仅需这些描述信息即可下载任意模组加载器，无需太多描述。

```json
{
    "forge": [
        {
            "rawversion": "1.21.1-52.0.10",  //原始版本，会显示原始版本符，如果该forge被标记，则也同样会输出被标记的名称。
            "mcversion": "1.21.1",  //MC版本
            "version": "52.0.10",  //精简过的版本，仅包含版本号。
            "installer": "https://maven.minecraftforge.net/net/minecraftforge/forge/1.21.1-52.0.10/forge-1.21.1-52.0.10-installer.jar"  //jar下载链接，可以直接安装。
        },
        ...
    ]
}
```
```json
{
    "fabric": [
        {
            "rawversion": "0.16.5",  //原始版本，会显示原始版本符
            "mcversion": "1.21.1",  //MC版本
            "version": "0.16.5",  //精简过的版本，仅包含版本号。
            "profile": "https://meta.fabricmc.net/v2/versions/loader/1.21.1/0.16.5/profile/json"  //元数据json，直接下载放入versions即可。
        },
        ...
    ]
}
```
```json
{
    "quilt": [
        {
            "rawversion": "0.26.4-beta.7",  //原始版本，会显示原始版本符
            "mcversion": "1.21.1",  //MC版本
            "version": "0.26.4-beta.7",  //精简过的版本，仅包含版本号。
            "profile": "https://meta.quiltmc.org/v3/versions/loader/1.21.1/0.26.4-beta.7/profile/json"  //元数据json，直接下载放入versions即可。
        }
    ]
}
```
```json
{
    "neoforge": [
        {
            "rawversion": "21.1.42",  //原始版本，会显示原始版本符，对于neoforge，这里会显示原本的版本号，可以直接用于拼接网址的。
            "mcversion": "1.21.1",  //MC版本
            "version": "21.1.42",  //精简过的版本，仅包含版本号。
            "installer": "https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.42/neoforge-21.1.42-installer.jar"  //元数据json，直接下载放入versions即可。
        }
    ]
}
```