# MMCLL Python 函数使用示例：

## 警告：在该类库里的任意函数，任何含有path的，末尾均不允许加“\”符号。而且必须保证路径分割符号都是“\”，如果是“/”的话则很有可能读取不了！【请使用两个\转义！】

## 警告：在使用PythonVersion的MMCLL时，可能有很多报错作者没考虑到。由于这不像Rust可以直接Result，因此可能会出发部分恐慌报错。如果你在使用过程中出现任何错误，欢迎反馈！

## 全局变量、常量、函数

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

```
全局变量类，如果需要使用则需要global！

download_source = 1
	下载源：目前仅支持两个数字，1：官方、2：BMCLAPI
	该变量目前用于下载等部分！
mc_root_json = {};
	mc的元数据（可以自己赋值也可以由类库帮忙赋值！）仅能赋值元数据值，如果赋上了别的值，后果自负！
	该值可以在适当情况下自行赋值，也可以使用类库随时赋值哦！
    值为一个字典形式。
```

## PyLib.MainClass

```
def get_file(path: str) -> str
	返回指定path参数的文件内容
	空值表示读取失败，多半是因为【权限不够】或者【路径乱填】
	有值表示读取成功，请直接获取文本内容。

def set_file(path: str, content: str) -> bool
	将来自content的字符串保存到执行path目录下。
	path后缀可以自行填写
	返回true则表示填写成功，false填写失败

def get_sha1(path: str) -> str
	返回指定path的文件sha1值。
	空值表示读取失败，多半是因为【权限不够】或者【路径乱填】
	有值表示读取成功，请直接获取文本内容。

def generated_bukkit_uuid(name: str) -> str
	返回指定离线账户用户名的标准bukkit的UUID
	返回空则表示读取失败【小概率事件】
	一般直接使用即可！

def get_nested_value(data, *keys, default_value=None)
    data为任意数据，可以是字典、列表、数字、字符串等。
    *key为你要通过data查询里面的值。
    default_value为如果你要查询到的值没有或与实际值不符合，则返回默认值。
    示例：
    a = {
        "aaa": {
            "bbb": "ccc"
        }
    }
    get_nested_value(a, "aaa", "bbb", default_value="") # 输出ccc
    get_nested_value(a, "aaa", "bbb", default_value={}) # 输出{}，因为获取到的值的类型与default_value的【类型】不一样。所以返回{}
    get_nested_value(a, "aaa", default_value={})  # 返回 {"bbb":"ccc"}，因为找到了aaa，并且与default_value的类型一致，都是列表。
    get_nested_value(a, "ddd", default_value=["bbb"])  # 返回 ["bbb"]，因为找不到ddd值，所以默认返回["bbb"]，默认值也可以包含值。

def delete_dir_retain(path: str, suffix: str = "") -> bool
    与RustVersion的delete_file_keep一致，不过这个函数在PythonVersion里被放在MainClass里。
	删除文件夹中的所有文件。但是保留suffix后缀的文件
    与Rust不同的是，如果suffix填入空，则默认删除所有文件（）


def unzip(zippath: str, extpath: str) -> bool
    默认阻塞主线程，对zip文件全部解压到文件夹后释放主线程。
	如果zip路径下的zip本身就不能解压，则返回false。
    解压成功返回true，否则返回false。
```

## PyLib.LaunchMethod

```
def convert_name_to_path(name: str) -> str
	将名称转成路径
	例如【org.slf4j:slf4j-api:2.0.9】返回【org\slf4j\slf4j-api\2.0.9\slf4j-api-2.0.9.jar】
    如果名称不正确，则返回空。

def get_mc_vanilla_version(root: str) -> str
	根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
	会按照clientVersion、patches->game|version、metadata->versions->releaseTime、id值进行找。
	如果连最终的id值也没有，则返回必定返回None！
	但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
	所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。当然也可以用catch_unwind来捕捉panic也就对了！

def extract_number(ext: str, is_num: bool) -> str:
	获取字符串中的所有数字或者字符
	如果is_num为true，则返回所有数字，反之则返回所有字符。

def get_mc_inherits_from(version_path: str, ioj: str) -> str
	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该路径的父路径一定是要为【存放了所有版本json的文件夹】。
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	根据mc版本json里的【ioj】键，准确找到对应的原版路径。
	如果ioj填入的是【inheritsFrom或jar】，则将会找到对应值所对应的原版路径。
	version_path应该填入你的版本文件夹路径，而非版本json或是别的路径。
	函数将会根据版本文件夹路径自动查询到版本json，随后判断里面是否包含【ioj】键，如果不包含，则默认返回Some(version_path)。
	如果包含，则会将版本文件夹路径往父文件夹退一格，然后遍历父文件夹的子文件夹，直到找到一个符合【ioj】键的一个原版。返回原版所对应的版本文件夹。
	如果找到了【ioj】键，但是未找到原版，则返回None！

def replace_mc_inherits_from(raw_json: str, ins_json: str) -> str
	将原来的json与找到的原版json进行键值替换。
	raw_json填入【有inheritsFrom】键的json，ins_json填入【已经找到的原版版本json】的。
	填入的是两个json的内容，而不是填入路径。
	该函数将会依次按照两个json的【mainClass、arguments->game、arguments->jvm、libraries、minecraftArguments】键进行替换和添加。
	返回值则是新的JSON字符串！

def get_mc_real_path(version_path: str, suffix: str) -> str
	警告1：该函数为MMCLL共有的特性函数，并且在MMCLL启动的很多地方也用到了这个函数！该函数使MMCLL支持启动了很多别的启动器无法启动的版本！
	警告2：如果你仅修改了【.minecraft\versions\<版本名>】中的版本名，而没有修改里面的json名称、json->id键、主jar名称的话，该函数也同样会试图查询该文件夹中的版本json！
	警告3：不像别的启动器在改了版本名字后也同样会修改json内值，你甚至可以写的启动器仅允许修改【版本文件夹名称】，也同样可以使用该函数获取到里面的json！

    从一个文件夹中根据suffix获取一个准确的文件。
    其中当suffix为“.json”的时候逻辑可能会略有不同，请参考下列提示

    其中suffix为“.json”时，则会按照【查询版本json】来查询，会直接查询该文件夹下的所有文件，然后使用json对其进行格式化。
    如果找到一个格式化成功的文件，则查询里面是否包含id、mainClass、libraries键。如果这三个键都有的话，则查询成功，返回该json的路径，返回值有值。反之这不是一个标准json，继续查找。

    suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
    目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回空值

def judge_arguments(root: dict, key: str) -> list
	判断参数并且拼接成Vec<String>并返回
	args_json需要填入你的版本json文件内容！
	key填入【game或jvm】。
	该函数会自动略过含有rules值的键，也就是说rules里包含的键需要各位自行判断架构！

def judge_mc_rules(root: dict) -> bool
	参数必须填入字典
	该参数会在你遍历libraries的时候，自动判断rules的值是否允许windows进入cp！
	该函数会判断(rules->action if allow then rules->os->name if windows)(rules->action if disallow then rules->os->name if not windows)
	返回上述，如果为windows则返回true，反之返回false！
	如果你需要别的架构，例如linux、macos等，你需要自行改写该函数。

def get_mc_libs(real_json: str, root_path: str, version_path: str) -> str

	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该文件夹下也同样包含着一个SHA1值等于json->downloads->client->sha1的主jar文件！
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	通过raw_json，自动拼接所有cp值进入一个Vec，然后再通过convert_name_to_path函数与【(root_path)\libraries】绑定到一起，随后判断该版本json里是否包含【jar】键，使用get_mc_inherits_from，如果包含jar，则查询到原版的主jar所在的原版版本文件夹。随后判断其里面是否有downloads->client->sha1值。随后按照get_mc_real_path进行查找。

	最后返回的值是一个包含${classpath_separator}的值。你需要自行按照电脑系统架构来判断该值应该replace成“:”还是“;”

	函数小解释：
	由于此时raw_json本应该是已经进行过replace_mc_inherits_from一次后了的，所以自然而然里面就已经包含了downloads键，能够获取到原版jar的sha1的。
	也就是说，虽然你的raw_json已经是replace后的，但是文件夹的文件还没有改动，因此可能可以获取到原版的jar！
	然后再对其get_mc_inherits_from对versions_path执行后返回值进行get_mc_real_path，如果返回Some，则拼接，反之则不拼接最后一个jar值。

def unzip_native(raw_json: str, root_path: str, version_path: str) -> bool
	该函数没有啥警告，但逻辑上与get_mc_libs差不多。
	只是这一次换成了查询natives，首先遍历libraries，找到所有包含natives字段的，如果有的话，则把name拼接到Vec上。
	然后直接将natives解压到【versions_path】下，用extract_file_name获取到versions_path的名称，拼接上${launcher_name}-natives的文件夹。
	如果里面没有包含任何一个有natives字段的键，则不用解压

class LaunchOption
	启动信息类，可以在里面获取到一些启动信息。
        account: LaunchAccount,	# 账号登录类【有一个专门的类，见下】
        java_path: str,      # Java路径
        root_path: str,		# mc根路径（需要里面包含assets、libraries两个文件夹）
        version_path: str,	# 版本路径（需要里面包含版本json、版本主jar）
        game_path: str,		# 游戏路径（里面啥都不需要有，用于存放游戏运行时路径）
        window_height: int,	# 窗口高度（默认480）
        window_width: int,	# 窗口宽度（默认854）
        min_memory: int,		# 最小内存（默认256m）
        max_memory: int,		# 最大内存（默认4096m）
        custom_info: str,	# 自定义信息（默认Tank Launcher Module）
        additional_jvm: str,	# 额外JVM参数（使用空格分开，默认空）
        additional_game: str # 额外Game参数（使用空格分开，默认空）
    上述几个变量除了account、java_path、version_path、game_path只有get函数以外，别的都有set、get函数！

	def __init__(account: LaunchAccount, java_path: str, root_path: str, version_path: str, game_path: str)
		初始化一个该类，必须传入的参数有【account、java_path、root_path、version_path】
        返回该类的实例

	pub fn set_xxx(&self, xxx: type)
		设置上述的任意参数！你可以选择设置任意一个参数，如果不设置则会按照默认值来判断。

	其余的set、get函数暂不多说。

class LaunchGame
	私有自己的启动类。如果想调用该类，请往下看。

class LaunchAccount
	账号启动类，内有7个全局变量，其中6个可以被初始化
	均只能被get，只能通过初始化的方式赋值

    def new_offline(name: str, uuid: str)

    	警告1：如果你要做国际版启动器，你不允许使用这个函数！因为国外的政策不允许在未付款的情况下先行游玩MC！除非在用户暂未购买MC的情况下自行添加--demo的额外游戏参数！

    	初始化一个离线登录，第一个参数填入用户名，第二个参数填入用户uuid。
    	如果你不想自己手动生成用户uuid，你可以使用new_offline_default。
        以下函数均同上返回一个本类的实例

    def new_microsoft(name: &str, uuid: &str, access_token: &str)
		新建一个微软登录实例，
		与离线登录不同的是，该函数多了个要求填入access_token的。

    def new_thirdparty(name: &str, uuid: &str, access_token: &str, base: &str, url: &str)
		新建一个第三方登录实例，与微软不同的是，这里要求写入一个元数据base64码，以及一个第三方元数据登录网址。
		其中第三方元数据网址，末尾必须是api/yggdrasil可以直接获取到元数据的。并且末尾不能有/符号。
		填入示例：https://littleskin.cn/api/yggdrasil

def launch_game(option: LaunchOption, callback: Callable[[list], None]) -> i32
	启动游戏函数！

	option填入上述的option，callback填入一个lambda。lambda里有一个参数，为【参数拼接成功的Vec】，返回值为i32，如果参数检测无误，则返回0，反之则返回常量池中的报错信息
	如果参数检测无误，但是在拼接启动参数时出现错误，则会直接raise，所以你可能需要try-except来保证不会直接退出程序。
	如果参数检测无误，并且启动参数拼接成功，则会执行lambda中的函数。
```

## PyLib.AccountClass
```
class UrlMethod
	网络获取的类
	接收一个url作为struct的值。

    def __init__(url: str)=
    	初始化一个该类，url填入网址。

    以下三个函数会阻塞主线程进行获取！
    def post(self, key: str, that: bool) -> str
    	对网址进行post，key为post请求参数。
    	that为头声明，如果为true，则请求Content-type为：application/x-www-form-urlencoded;charset=utf-8
    	此时key必须为【aa=bb&cc=dd】这种形式。
    	反之如果为false，则请求Content-type以及Accept为：application/json;charset=utf-8
    	此时key必须为【{"aa":"bb","cc":"dd"}】这种形式。
    	返回请求后的网址内容！如果网络连接不好，则返回空

    def get(self, key: str) -> str
    	对网址进行get请求。key为验证参数。
    	请求头：AUTHORIZATION，值：【Bearer {key}】。
    	返回get后的网址内容！

    def get_default(self) -> bytes
    	对网址进行默认抓取。
    	如果网址返回值为html，则也会返回html。如果网址为二进制下载文件，则返回下载内容。
		该函数返回值改成了bytes字节数组，这意味着它不仅可以获取网络上的文本资源，还可以保存二进制文件。
		也就是既可以下载，也可以保存到内存里。

    以下三个函数使用异步运行！
	async def post_async(&self, key: &str, that: bool) -> str
		异步post。函数内容与post几乎一样，唯一不同的就是添加了await进行异步。

	async def get_async(&self, key: &str) -> str
		异步get，但是有key验证参数。

	async def get_default_async(&self) -> bytes
		异步get_default，返回bytes。

```

