# MMCLL Rust 函数使用示例：

## 警告：在该类库里的任意函数，任何含有path的，均使用 filepath.join 构造！

## some_const.go

```
全局常量类，里面存放了一大堆的标准常量。使用时无需unsafe，

里面基本上都有注释啦！
```
#### 切记！里面有三个值需要注意：

```
USER_AGENT = "MMCLL/<版本>"
	这个常量需要替换成你的【<启动器名称>/<启动器版本>】，并且实时更新。各位也可以把该常量转移到变量里使用。（默认是MMCLL/<MMCLL的版本>）
LAUNCHER_NAME = "MMCLL"
	这个常量需要替换成你的【<启动器名称>】。在使用默认方式启动时，会自动将【${launcher_name}】替换成该值。
LAUNCHER_NAME = "<版本>"
	请自觉将此值改成你的【<启动器版本>】，并且实时更新。因为在默认方式启动替换（${launcher_version}）时用到这个值。各位可以自行去put_arguments查看逻辑以修改。
```

## main_method.go

```
func GetFile(path string) (string, err)
	返回指定path参数的文件内容
	err 不为 nil 表示读取失败，多半是因为【权限不够】或者【路径乱填】
	反之表示读取成功

func SetFile(path, content string) error
	将来自content的字符串保存到执行path目录下。
	path后缀可以自行填写
	返回nil则表示填写成功，反之填写失败

func GetSha1(path string) (string, err)
	返回指定path的文件sha1值。
	err 不为 nil 表示读取失败，多半是因为【权限不够】或者【路径乱填】
	反之表示读取成功

func GenerateBukkitUUID(username string) string
	返回指定离线账户用户名的标准bukkit的UUID
	返回空则表示读取失败【小概率事件】
	一般直接使用即可！无需unwrap！

func Safe(cond any, defaultValue any, keys ...any) any
	安全的获取 cond 里面的值，在我的程序里很多地方都用到了，均可以作为参考~

func SafeGet[T any](slice []T, index int) (T, bool)
	安全的获取数组里的值，用于得到一个索引之后，怕索引超出范围而执行，如果超出范围了，则 bool 为 false！

func If(condition bool, trueVal, falseVal any) any
	模仿的三元运算符

func ArrayIndexOf[T comparable](array []T, item T) int
	查询切片中的元素，等同于 array.IndexOf
```

## launch_method.go

```
func ConvNameToPath(name string) string
	将名称转成路径
	例如【org.slf4j:slf4j-api:2.0.9】返回【org\slf4j\slf4j-api\2.0.9\slf4j-api-2.0.9.jar】
	转换不成功则返回空

func GetVanillaVersion(j string) (string, error)
	根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
	会按照clientVersion、patches->game|version、（计划 releaseTime）、（计划 NeoForge vanilla key）、id值进行找。
	如果连最终的id值也没有，则返回必定返回None！
	但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
	所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。

func Unzip(zipPath, extractPath string) error
	读取zipfile的文件，并解压到extfile路径下。
	如果zip路径下的zip本身就不能解压，则返回err。
	zipfile是一个文件路径、extfile是一个文件夹路径。
	解压成功返回true

pub fn delete_file_keep(dir_path: string, suffix: &str) -> bool
	删除文件夹中的所有文件。但是保留suffix后缀的文件
	如果suffix填入空则默认保留所有文件夹，如果希望删掉整个文件夹，则需要对suffix乱填一个参数！

func ExtractNumber(str string) string
	获取字符串中的所有数字或者字符
	永远只返回去除所有数字的字符串

func GetMCInheritsFrom(versionPath, suffix string) (string, error)

	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该路径的父路径一定是要为【存放了所有版本json的文件夹】。
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	根据mc版本json里的【suffix】键，准确找到对应的原版路径。
	如果suffix填入的是【inheritsFrom或jar】，则将会找到对应值所对应的原版路径。
	version_path应该填入你的版本文件夹路径，而非版本json或是别的路径。
	函数将会根据版本文件夹路径自动查询到版本json，随后判断里面是否包含【suffix】键，如果不包含，则默认返回Some(version_path)。
	如果包含，则会将版本文件夹路径往父文件夹退一格，然后遍历父文件夹的子文件夹，直到找到一个符合【suffix】键的一个原版。返回原版所对应的版本文件夹。
	如果找到了【suffix】键，但是未找到原版，则返回None！

func MergeMCJson(raw, ins string) map[string]any
	将原来的json与找到的原版json进行键值合并。
	raw填入【有inheritsFrom】键的json，ins填入【已经找到的原版版本json】的。
	填入的是两个json的内容，而不是填入路径。
	该函数将会依次按照两个json的【mainClass、arguments -> game、arguments -> jvm、libraries、minecraftArguments】键进行替换和添加。
	返回值则是新的JSON内容（不是返回字符串，请记住）！

func GetMCRealPath(versionPath, suffix string) (string, error)
	
	警告1：该函数为MMCLL共有的特性函数，并且在MMCLL启动的很多地方也用到了这个函数！该函数使MMCLL支持启动了很多别的启动器无法启动的版本！
	警告2：如果你仅修改了【.minecraft\versions\<版本名>】中的版本名，而没有修改里面的json名称、json->id键、主jar名称的话，该函数也同样会试图查询该文件夹中的版本json！
	警告3：不像别的启动器在改了版本名字后也同样会修改json内值，你甚至可以写的启动器仅允许修改【版本文件夹名称】，也同样可以使用该函数获取到里面的json！

    从一个文件夹中根据suffix获取一个准确的文件。
    其中当suffix为“.json”的时候逻辑可能会略有不同，请参考下列提示

    其中suffix为“.json”时，则会按照【查询版本json】来查询，会直接查询该文件夹下的所有文件，然后使用json对其进行格式化。
    如果找到一个格式化成功的文件，则查询里面是否包含id、mainClass、libraries键。如果这三个键都有的话，则查询成功，返回该json的路径，使用Some接收。反之这不是一个标准json，继续查找。

    suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
    目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回err

func JudgeArguments(realJson map[string]any, arg string) []string
	判断参数并且拼接成字符串切片并返回
	realJson需要填入你的版本json文件内容！
	key填入【game或jvm】。
	该函数会自动略过含有rules值的键，也就是说rules里包含的键需要各位自行判断架构！

func JudgeRule(rules []any) bool
	很抱歉，该函数由于为了方便！参数必须填入已经被转换成 []any 的值！
	该参数会在你遍历libraries的时候，自动判断rules的值是否允许windows进入cp！
	该函数会判断(rules->action if allow then rules->os->name if windows)(rules->action if disallow then rules->os->name if not windows)
	返回上述，如果为true则返回true，反之返回false！
	目前该函数已经读取了 info_${platform}.go 实现了跨平台

func LibIndexOf(libs []string, lib string) int
	用于获取 Libs 是否有重复，如果没有，则返回-1，否则返回索引

func GetMCLibs(realJson map[string]any, rootPath, versionPath string) ([]string, error)

	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该文件夹下也同样包含着一个SHA1值等于json->downloads->client->sha1的主jar文件！
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	通过raw_json，自动拼接所有cp值进入一个切片，然后再通过convert_name_to_path函数与【(root_path)\libraries】绑定到一起，随后判断该版本json里是否包含【jar】键，使用get_mc_inherits_from，如果包含jar，则查询到原版的主jar所在的原版版本文件夹。随后判断其里面是否有downloads->client->sha1值。随后按照GetMCRealPath进行查找。

	最后返回的值是一个包含所有类库的切片，你需要自行用 ${classpath_separator} 进行拼接。

	函数小解释：
	由于此时rawJson本应该是已经进行过MergeMCJson一次后了的，所以自然而然里面就已经包含了downloads键，能够获取到原版jar的sha1的。
	也就是说，虽然你的rawJson已经是Merge后的，但是文件夹的文件还没有改动，因此可能可以获取到原版的jar！
	然后再对其GetMCInheritsFrom对VersionPath执行后返回值进行GetMCInheritsFrom，如果err不为nil，则拼接，反之则不拼接最后一个jar值。

func UnzipNative(finalJson, rootPath, versionPath string) error
	该函数没有啥警告，但逻辑上与get_mc_libs差不多。
	只是这一次换成了查询natives，首先遍历libraries，找到所有包含natives字段的，如果有的话，则把name拼接到切片上。
	然后直接将natives解压到【versions_path】下，用extract_file_name获取到versions_path的名称，拼接上${launcher_name}-natives的文件夹。
	如果里面没有包含任何一个有natives字段的键，则不用解压

type LaunchOption struct
	启动信息类，可以在里面获取到一些启动信息。
        account LaunchAccount,	// 账号登录类【有一个专门的类，见下】
        java_path string,      // Java路径
        root_path string,		// mc根路径（需要里面包含assets、libraries两个文件夹）
        version_path string,	// 版本路径（需要里面包含版本json、版本主jar）
        game_path string,		// 游戏路径（里面啥都不需要有，用于存放游戏运行时路径）
        window_height uint32,	// 窗口高度（默认480）
        window_width uint32,	// 窗口宽度（默认854）
        min_memory uint32,		// 最小内存（默认256m）
        max_memory uint32,		// 最大内存（默认4096m）
        custom_info string,	// 自定义信息（默认Tank Launcher Module）
        additional_jvm string,	// 额外JVM参数（使用空格分开，默认空）
        additional_game string // 额外Game参数（使用空格分开，默认空）
    上述几个变量除了account、java_path、version_path、game_path只有get函数以外，别的都有set、get函数！

	pub fn new(account: LaunchAccount, java_path: &str, root_path: &str, version_path: &str, game_path: &str) -> Self
		初始化一个该类，必须传入的参数有【account、java_path、root_path、version_path】

	pub fn set_xxx(&self, xxx: type)
		设置上述的任意参数！你可以选择设置任意一个参数，如果不设置则会按照默认值来判断。

	其余的set、get函数暂不多说。

type LaunchGame struct
	私有自己的启动类。如果想调用该类，请往下看。

type LaunchAccount struct
	账号启动类，内有7个全局变量，其中6个可以被初始化
	均只能被get，只能通过初始化的方式赋值

    func NewLaunchAccountOffline(name, uuid string) LaunchAccount

    	警告1：如果你要做国际版启动器，你不允许使用这个函数！因为国外的政策不允许在未付款的情况下先行游玩MC！除非在用户暂未购买MC的情况下自行添加--demo的额外游戏参数！

    	初始化一个离线登录，第一个参数填入用户名，第二个参数填入用户uuid。
    	如果你不想自己手动生成用户uuid，你可以使用new_offline_default。

    func NewLaunchAccountMicrosoft(name, uuid, accessToken string) LaunchAccount
		新建一个微软登录实例，
		与离线登录不同的是，该函数多了个要求填入access_token的。

    func NewLaunchAccountThirdParty(name, uuid, accessToken, base, url string) LaunchAccount
		新建一个第三方登录实例，与微软不同的是，这里要求写入一个元数据base64码，以及一个第三方元数据登录网址。
		其中第三方元数据网址，末尾必须是api/yggdrasil可以直接获取到元数据的。并且末尾不能有/符号。
		填入示例：https://littleskin.cn/api/yggdrasil

func LaunchGame(option LaunchOption, isStrict bool, callback func([]string)) error
	启动游戏函数！

	option填入上述的option，callback填入一个lambda。lambda里有一个参数，为【参数拼接成功的切片】，返回值为error，如果参数检测无误，则返回nil，反之则NewMMCLLError
	isStrict 的意思是是否将 启动参数设置 交给 MMCLL 帮助你分析，如果你不需要帮助分析，请设置为 false，这样可以跳过分析，直接拼接启动参数并启动游戏、
	如果参数检测无误，并且启动参数拼接成功，则会执行闭包中的函数。
	闭包里的函数，其实就是拼接好的启动参数，你可以自行运行该参数~
```

## rust_lib::account_mod
```
type UrlMethod struct
	网络获取的类
	接收一个url作为struct的值。

    func NewUrlMethod(url string) *UrlMethod
    	创建一个该类，url填入网址。

    以下三个函数均会阻塞主线程进行获取，如果你需要异步，请自行使用go routiner！
    func (url *UrlMethod) Post(body string, isJSON bool) (string, error)
    	对网址进行post，key为post请求参数。
    	isJson为头声明，如果为false，则请求Content-type为：application/x-www-form-urlencoded;charset=utf-8
    	此时key必须为【aa=bb&cc=dd】这种形式。
    	反之如果为true，则请求Content-type以及Accept为：application/json;charset=utf-8
    	此时key必须为【{"aa":"bb","cc":"dd"}】这种形式。
    	返回请求后的网址内容！

    func (url *UrlMethod) Get(authorization string) (string, error)
    	对网址进行get请求。key为验证参数。
    	请求头：AUTHORIZATION，值：【Bearer {key}】。
    	返回get后的网址内容！

    func (url *UrlMethod) GetDefault() (string, error)
    	对网址进行默认抓取。
    	如果网址返回值为html，则也会返回html。如果网址为二进制下载文件，则返回下载内容。
		该函数返回值改成了[]byte字节数组，这意味着它不仅可以获取网络上的文本资源，还可以保存二进制文件。
		也就是既可以下载，也可以保存到内存里。

type AccountResult struct
	该实现构造函数为私有，如果你需要给其赋值，请自行添加pub关键字。
	微软正版登录会有：name、uuid、access_token、refresh_token（4个字段）
	第三方登录会有：name、uuid、access_token、client_token、base（5个字段）
	name string
		账号名称
	uuid string
		账号uuid
	access_token string
		账号验证密钥
	refresh_token string
		账号刷新密钥
	client_token string
		第三方登录的客户端密钥（通常是UUID，如果之前没填则没有。）
	base string
		第三方元数据的base64编码
	其中，refresh_token有一个set方法，但是是私有的。
	其余的仅能get。
type AccountLogin struct
	本类里面全部都是【异步函数】，你可能需要使用tokio自主实现并调用运行。

	pub fn NewAccountLogin(client_id: &str) -> Self
		构建一个AccountLogin类，填入一个client_id字段。
		client_id字段相信每个制作启动器的玩家都知道什么意思吧！这里不再赘述。

		如果将来制作了第三方登录的话，这里应该填入的是服务器地址（

	以下函数全部都是同步的，如果你想要异步，请自行使用 go routine
    func (account *AccountLogin) GetUserCode() (string, string, error)
		获取用户代码。
		通过client_id获取到一个用户代码用于登录，这个函数为两个返回值，另一个返回值是device_code。设备代码。
		当你获取了用户代码后，你可以循环通过device_code获取到access_token，但是请间隔5s获取一次。
		15分钟后，如果用户未登录完成，则返回一个指定的Err代码。
	
    func (account *AccountLogin) microsoft(loginCode string) (*AccountResult, error)
		私有函数，通过access_token获取到AccountResult实现。

    func (account *AccountLogin) LoginMicrosoft(deviceCode string) (*AccountResult, error)
		通过device_code获取到AccountResult实现。
		你可以轮询通过device_code然后实现该函数。该函数会返回一个AccountResult，如果不对，则会返回一个Err类型。
		其中，err类型描述了此时的错误信息。
		具体如果你需要忽略的值，请参阅 EXAMPLE.md

	[暂未实现]
	func (account *AccountLogin) RefreshMicrosoft()
		通过refresh_token获取到AccountResult实现
		该函数用于刷新你的access_token。你可以从第一次登录微软账号时，获取到的refresh_token进行刷新。
```

## info_${platform}.go

一个跨平台的编译文件，内容很简单，自己去看就好了（