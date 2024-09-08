# MMCLL Rust 函数使用示例：

## 警告：在该类库里的任意函数，任何含有path的，末尾均不允许加“\”符号。而且必须保证路径分割符号都是“\”，如果是“/”的话则很有可能读取不了！【请使用两个\转义！】

## rust_lib::some_const

```
全局常量类，里面存放了一大堆的标准常量。使用时无需unsafe，

里面基本上都有注释啦！
```
#### 切记！里面有三个值需要注意：

```
pub const USER_AGENT: &str = "MMCLL/<版本>"
	这个常量需要替换成你的【<启动器名称>/<启动器版本>】，并且实时更新。各位也可以把该常量转移到变量里使用。（默认是MMCLL/<MMCLL的版本>）
pub const LAUNCHER_NAME: &str = "MMCLL"
	这个常量需要替换成你的【<启动器名称>】。在使用默认方式启动时，会自动将【${launcher_name}】替换成该值。
pub const LAUNCHER_NAME: &str = "<版本>"
	请自觉将此值改成你的【<启动器版本>】，并且实时更新。因为在默认方式启动替换（${launcher_version}）时用到这个值。各位可以自行去put_arguments查看逻辑以修改。
```

## rust_lib::some_var

```
全局变量类，如果需要获取则需要with_borrow包住！
如果需要修改里面的值，并且值是可变的，可以使用with_borrow_mut。
如果需要设置该变量请直接使用set函数。

pub static DOWNLOAD_SOURCE: RefCell<i32> = RefCell::new(1)
	下载源：目前仅支持两个数字，1：官方、2：BMCLAPI
	该变量目前用于下载等部分！
pub static MC_ROOT_JSON: RefCell<Value> = RefCell::new(Value::Null)
	mc的元数据（可以自己赋值也可以由类库帮忙赋值！）仅能赋值元数据值，如果赋上了别的值，后果自负！
	该值可以在适当情况下自行赋值，也可以使用类库随时赋值哦！
	当然了！该值填入一个serde_json::Value。
pub static AUTHLIB_PATH: RefCell<String> = RefCell::new(String::new())
	设置第三方登录的模块jar文件路径。在使用第三方登录的时候一定要设置该参数！
	如果找不到该jar文件，在启动游戏的时候返回Err(-15)（新增）

还有两个参数目前暂时是unused，将来会使用到的捏~
```

## rust_lib::main_method

```
pub fn get_file(path: &str) -> Option<String>
	返回指定path参数的文件内容
	None表示读取失败，多半是因为【权限不够】或者【路径乱填】
	Some表示读取成功，可以用 if let Some(e) 来获取文本内容。

pub fn set_file(path: &str, content: String) -> bool
	将来自content的字符串保存到执行path目录下。
	path后缀可以自行填写
	返回true则表示填写成功，false填写失败

pub fn delete_file(path: &str) -> bool 
	删除指定path下的文件
	返回true则表示填写成功，false填写失败

pub fn get_sha1(path: &str) -> Option<String>
	返回指定path的文件sha1值。
	None表示读取失败，多半是因为【权限不够】或者【路径乱填】
	Some表示读取成功，可以用 if let Some(e) 来获取文本内容。

pub fn generated_bukkit_uuid(name: &str) -> String
	返回指定离线账户用户名的标准bukkit的UUID
	返回空则表示读取失败【小概率事件】
	一般直接使用即可！无需unwrap！

pub fn generate_thirdparty_metadata_base64(url: &str) -> String
	返回第三方元数据的标准base64代码。【例如填写https://littleskin.cn/api/yggdrasil，返回元数据的base64。】
	如果网络不好，则返回空。
	该函数已废弃，因为该函数为阻塞主线程使用的，请不要使用该函数！

pub fn extract_file_name(file: &str) -> String
	返回一个文件的文件名。
	填入【D:\aa.txt】时返回【aa.txt】。
	如果填入的非路径，或者末尾带有“\”的路径，或者都没有出现过“\”的路径，则返回空。

pub fn hex_decode(raw: &str) -> Option<Vec<u8>> 
	将一个16进制字符串转换成u8数组。
	例如【aabbcc】转成【[170, 187, 204]】
	当字符串不符合【a-f0-9】并且是2的倍数时，返回None。

pub fn hex_encode(raw: Vec<u8>) -> String
	将u8数组转换成字符串。
	例如【[170, 187, 204]】转成【aabbcc】
	该函数不会抛出报错，仅需使用正常方式接收即可。

pub fn get_file_bits(file: String) -> Option<bool>
	返回文件的位数【为32位或64位】
	如果是64位则为true，反之则为false。
	该函数仅限Windows！！！

pub fn get_file_version(file: String) -> Option<String>
	返回文件的版本。
	该函数目前只返回String，如果想让其返回Vec，你需要自行修改该函数！
	该函数仅限Windows！！！
```

## rust_lib::launcher_mod

```
pub fn convert_name_to_path(name: String) -> Option<String>
	将名称转成路径
	例如【org.slf4j:slf4j-api:2.0.9】返回【org\slf4j\slf4j-api\2.0.9\slf4j-api-2.0.9.jar】

pub fn get_mc_vanilla_version(json: String) -> Option<String>
	根据一个原版的json，准确的找到原版键值。（只能原版，如果不是原版，则必定返回None）
	会按照clientVersion、patches->game|version、metadata->versions->releaseTime、id值进行找。
	如果连最终的id值也没有，则返回必定返回None！
	但是最终的id值很可能不是代表着原版值，因为别的启动器很可能会修改文件夹的名字顺带把json里的id值也改了。
	所以各位一定要记得做判断！如果想自定义一个类来启动的而不是用game_launch类启动的话。当然也可以用catch_unwind来捕捉panic也就对了！

pub fn unzip(zipfile: String, extfile: String) -> bool
	读取zipfile的文件，并解压到extfile路径下。
	如果zip路径下的zip本身就不能解压，则返回false。
	zipfile是一个文件路径、extfile是一个文件夹路径。
	解压成功返回true

pub fn delete_file_keep(dir_path: String, suffix: &str) -> bool
	删除文件夹中的所有文件。但是保留suffix后缀的文件
	如果suffix填入空则默认保留所有文件夹，如果希望删掉整个文件夹，则需要对suffix乱填一个参数！

pub fn extract_number(ext: String, isnum: bool) -> String
	获取字符串中的所有数字或者字符
	如果isnum为true，则返回所有数字，反之则返回所有字符。

pub fn get_mc_inherits_from(version_path: String, ioj: &str) -> Option<String>

	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该路径的父路径一定是要为【存放了所有版本json的文件夹】。
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	根据mc版本json里的【ioj】键，准确找到对应的原版路径。
	如果ioj填入的是【inheritsFrom或jar】，则将会找到对应值所对应的原版路径。
	version_path应该填入你的版本文件夹路径，而非版本json或是别的路径。
	函数将会根据版本文件夹路径自动查询到版本json，随后判断里面是否包含【ioj】键，如果不包含，则默认返回Some(version_path)。
	如果包含，则会将版本文件夹路径往父文件夹退一格，然后遍历父文件夹的子文件夹，直到找到一个符合【ioj】键的一个原版。返回原版所对应的版本文件夹。
	如果找到了【ioj】键，但是未找到原版，则返回None！

pub fn replace_mc_inherits_from(mut raw_json: String, mut ins_json: String) -> Option<String>
	将原来的json与找到的原版json进行键值替换。
	raw_json填入【有inheritsFrom】键的json，ins_json填入【已经找到的原版版本json】的。
	填入的是两个json的内容，而不是填入路径。
	该函数将会依次按照两个json的【mainClass、arguments->game、arguments->jvm、libraries、minecraftArguments】键进行替换和添加。
	返回值则是新的JSON字符串！

pub fn get_mc_real_path(version_path: String, suffix: &str) -> Option<String>
	
	警告1：该函数为MMCLL共有的特性函数，并且在MMCLL启动的很多地方也用到了这个函数！该函数使MMCLL支持启动了很多别的启动器无法启动的版本！
	警告2：如果你仅修改了【.minecraft\versions\<版本名>】中的版本名，而没有修改里面的json名称、json->id键、主jar名称的话，该函数也同样会试图查询该文件夹中的版本json！
	警告3：不像别的启动器在改了版本名字后也同样会修改json内值，你甚至可以写的启动器仅允许修改【版本文件夹名称】，也同样可以使用该函数获取到里面的json！

    从一个文件夹中根据suffix获取一个准确的文件。
    其中当suffix为“.json”的时候逻辑可能会略有不同，请参考下列提示

    其中suffix为“.json”时，则会按照【查询版本json】来查询，会直接查询该文件夹下的所有文件，然后使用json对其进行格式化。
    如果找到一个格式化成功的文件，则查询里面是否包含id、mainClass、libraries键。如果这三个键都有的话，则查询成功，返回该json的路径，使用Some接收。反之这不是一个标准json，继续查找。

    suffix一般是以后缀为基础的。如果说不以后缀为基础，也可以用SHA1值做为基础。
    目前仅支持SHA1和后缀，如果不以这两个，则很可能会返回None

pub fn judge_arguments(args_json: String, key: &str) -> Option<Vec<String>>
	判断参数并且拼接成Vec<String>并返回
	args_json需要填入你的版本json文件内容！
	key填入【game或jvm】。
	该函数会自动略过含有rules值的键，也就是说rules里包含的键需要各位自行判断架构！

pub fn judge_mc_rules(root: &serde_json::Value) -> bool
	很抱歉，该函数由于为了方便！参数必须填入已经被serde_json给转成了Value后的值！
	该参数会在你遍历libraries的时候，自动判断rules的值是否允许windows进入cp！
	该函数会判断(rules->action if allow then rules->os->name if windows)(rules->action if disallow then rules->os->name if not windows)
	返回上述，如果为true则返回true，反之返回false！
	如果你需要别的架构，例如linux、macos等，你需要自行改写该函数。

pub fn get_mc_libs(raw_json: String, root_path: &str, version_path: &str) -> Option<String>

	警告1：该函数仅适用于在你的version_path下确定包含有【版本json】这个东西，并且该文件夹下也同样包含着一个SHA1值等于json->downloads->client->sha1的主jar文件！
	警告2：该函数不支持MultiMC、XMCL的启动格式！如果你需要适配这两个启动器的启动，你则需要自己实现一个类似的函数而非直接使用本函数！

	通过raw_json，自动拼接所有cp值进入一个Vec，然后再通过convert_name_to_path函数与【(root_path)\libraries】绑定到一起，随后判断该版本json里是否包含【jar】键，使用get_mc_inherits_from，如果包含jar，则查询到原版的主jar所在的原版版本文件夹。随后判断其里面是否有downloads->client->sha1值。随后按照get_mc_real_path进行查找。

	最后返回的值是一个包含${classpath_separator}的值。你需要自行按照电脑系统架构来判断该值应该replace成“:”还是“;”

	函数小解释：
	由于此时raw_json本应该是已经进行过replace_mc_inherits_from一次后了的，所以自然而然里面就已经包含了downloads键，能够获取到原版jar的sha1的。
	也就是说，虽然你的raw_json已经是replace后的，但是文件夹的文件还没有改动，因此可能可以获取到原版的jar！
	然后再对其get_mc_inherits_from对versions_path执行后返回值进行get_mc_real_path，如果返回Some，则拼接，反之则不拼接最后一个jar值。

pub fn unzip_native(raw_json: String, root_path: &str, version_path: &str) -> bool
	该函数没有啥警告，但逻辑上与get_mc_libs差不多。
	只是这一次换成了查询natives，首先遍历libraries，找到所有包含natives字段的，如果有的话，则把name拼接到Vec上。
	然后直接将natives解压到【versions_path】下，用extract_file_name获取到versions_path的名称，拼接上${launcher_name}-natives的文件夹。
	如果里面没有包含任何一个有natives字段的键，则不用解压

impl LaunchOption
	启动信息类，可以在里面获取到一些启动信息。
        account: LaunchAccount,	// 账号登录类【有一个专门的类，见下】
        java_path: String,      // Java路径
        root_path: String,		// mc根路径（需要里面包含assets、libraries两个文件夹）
        version_path: String,	// 版本路径（需要里面包含版本json、版本主jar）
        game_path: String,		// 游戏路径（里面啥都不需要有，用于存放游戏运行时路径）
        window_height: usize,	// 窗口高度（默认480）
        window_width: usize,	// 窗口宽度（默认854）
        min_memory: usize,		// 最小内存（默认256m）
        max_memory: usize,		// 最大内存（默认4096m）
        custom_info: String,	// 自定义信息（默认Tank Launcher Module）
        additional_jvm: String,	// 额外JVM参数（使用空格分开，默认空）
        additional_game: String // 额外Game参数（使用空格分开，默认空）
    上述几个变量除了account、java_path、version_path、game_path只有get函数以外，别的都有set、get函数！

	pub fn new(account: LaunchAccount, java_path: &str, root_path: &str, version_path: &str, game_path: &str) -> Self
		初始化一个该类，必须传入的参数有【account、java_path、root_path、version_path】

	pub fn set_xxx(&self, xxx: type)
		设置上述的任意参数！你可以选择设置任意一个参数，如果不设置则会按照默认值来判断。

	其余的set、get函数暂不多说。

impl LaunchGame
	私有自己的启动类。如果想调用该类，请往下看。

impl LaunchAccount
	账号启动类，内有7个全局变量，其中6个可以被初始化
	均只能被get，只能通过初始化的方式赋值

    pub fn new_offline(name: &str, uuid: &str) -> Self

    	警告1：如果你要做国际版启动器，你不允许使用这个函数！因为国外的政策不允许在未付款的情况下先行游玩MC！除非在用户暂未购买MC的情况下自行添加--demo的额外游戏参数！

    	初始化一个离线登录，第一个参数填入用户名，第二个参数填入用户uuid。
    	如果你不想自己手动生成用户uuid，你可以使用new_offline_default。

	#[deprecated(since = "0.0.8", note = "Please login thirdparty in account_mod, and auto get base64 code by sync.")]
    pub fn new_offline_default(name: &str) -> Self
    	警告同上。
    	UUID会自己按照bukkit方式生成
		该函数已废弃，因为uuid已经在上方有了一个generate_bukkit_uuid函数。请各位自行调用。

    pub fn new_microsoft(name: &str, uuid: &str, access_token: &str) -> Self
		新建一个微软登录实例，
		与离线登录不同的是，该函数多了个要求填入access_token的。

    pub fn new_thirdparty(name: &str, uuid: &str, access_token: &str, base: &str, url: &str) -> Self
		新建一个第三方登录实例，与微软不同的是，这里要求写入一个元数据base64码，以及一个第三方元数据登录网址。
		其中第三方元数据网址，末尾必须是api/yggdrasil可以直接获取到元数据的。并且末尾不能有/符号。
		填入示例：https://littleskin.cn/api/yggdrasil

	#[deprecated(since = "0.0.8", note = "Please login thirdparty in account_mod, and auto get base64 code by sync.")]
    pub fn new_thirdparty_default(name: &str, uuid: &str, access_token: &str, url: &str) -> Self
		新建一个第三方登录实例，与上述不同的是，该函数会自动通过url获取到元数据。
		该函数已废弃，

pub fn launch_game<F>(option: LaunchOption, callback: F) -> Result<(), i32>
where
	F: Fn(Vec<&str>) + 'static
	启动游戏函数！

	option填入上述的option，callback填入一个闭包。闭包里有一个参数，为【参数拼接成功的Vec】，返回值为Result，如果参数检测无误，则返回空，反之则返回some_const中的任意报错类型。
	如果参数检测无误，但是在拼接启动参数时出现错误，则会直接panic，所以你可能需要catch_unwind来保证不会直接退出程序。
	如果参数检测无误，并且启动参数拼接成功，则会执行闭包中的函数。
	各位可以直接使用std::process::Command来执行返回闭包中的Vec参数噢！如果参数字符数量超过了8192个字符，你可能就需要将参数输出到外部用bat执行了（）
```

## rust_lib::account_mod
```
impl UrlMethod
	网络获取的类
	接收一个url作为struct的值。

    pub fn new(url: &str) -> Self
    	创建一个该类，url填入网址。

    以下三个函数会阻塞主线程进行获取！
    pub fn post(&self, key: &str, that: bool) -> Option<String>
    	对网址进行post，key为post请求参数。
    	that为头声明，如果为true，则请求Content-type为：application/x-www-form-urlencoded;charset=utf-8
    	此时key必须为【aa=bb&cc=dd】这种形式。
    	反之如果为false，则请求Content-type以及Accept为：application/json;charset=utf-8
    	此时key必须为【{"aa":"bb","cc":"dd"}】这种形式。
    	返回请求后的网址内容！

    pub fn get(&self, key: &str) -> Option<String>
    	对网址进行get请求。key为验证参数。
    	请求头：AUTHORIZATION，值：【Bearer {key}】。
    	返回get后的网址内容！

    pub fn get_default(&self) -> Option<Vec<u8>>
    	对网址进行默认抓取。
    	如果网址返回值为html，则也会返回html。如果网址为二进制下载文件，则返回下载内容。
		该函数返回值改成了Vec<u8>字节数组，这意味着它不仅可以获取网络上的文本资源，还可以保存二进制文件。
		也就是既可以下载，也可以保存到内存里。

    以下三个函数使用异步运行！
	pub async fn post_async(&self, key: &str, that: bool) -> Option<String>
		异步post。函数内容与post几乎一样，唯一不同的就是添加了await进行异步。

	pub async fn get_async(&self, key: &str) -> Option<String>
		异步get，但是有key验证参数。

	pub async fn get_default_async(&self) -> Option<Vec<u8>>
		异步get_default，返回Vec<u8>。

impl AccountResult
	该实现构造函数为私有，如果你需要给其赋值，请自行添加pub关键字。
	微软正版登录会有：name、uuid、access_token、refresh_token（4个字段）
	第三方登录会有：name、uuid、access_token、client_token、base（5个字段）
	name: String
		账号名称
	uuid: String
		账号uuid
	access_token: String
		账号验证密钥
	refresh_token: String
		账号刷新密钥
	client_token: String
		第三方登录的客户端密钥（通常是UUID，如果之前没填则没有。）
	base: String
		第三方元数据的base64编码
	其中，refresh_token有一个set方法，但是是私有的。
	其余的仅能get。
impl AccountLogin
	本类里面全部都是【异步函数】，你可能需要使用tokio自主实现并调用运行。

	pub fn new_ms(client_id: &str) -> Self
		构建一个AccountLogin类，填入一个client_id字段。
		client_id字段相信每个制作启动器的玩家都知道什么意思吧！这里不再赘述。

	pub fn new_tp(server: &str) -> Self
		构建一个AccountLogin类，填入一个server字段。
		server为你的外置登录服务器，需要精确到api/yggdrasil。
		其实这个与上面那个一样的（）你甚至可以用上面那个来构建这个，也可以用这个构建上面那个。。
	
    pub async fn get_user_code(&self) -> Result<(String, String), i32>
		获取用户代码。
		通过client_id获取到一个用户代码用于登录，这个函数为两个返回值，另一个返回值是device_code。设备代码。
		当你获取了用户代码后，你可以循环通过device_code获取到access_token，但是请间隔5s获取一次。
		15分钟后，如果用户未登录完成，则返回一个指定的Err代码。
	
    async fn microsoft(&self, access_token: &str) -> Result<AccountResult, i32>
		私有函数，通过access_token获取到AccountResult实现。

    pub async fn login_microsoft(&self, device_code: String) -> Result<AccountResult, i32>
		通过device_code获取到AccountResult实现。
		你可以循环通过device_code然后实现该函数。该函数会返回一个AccountResult，如果不对，则会返回一个Err类型。
		其中，i32类型描述了此时的错误信息。请参阅some_const常量池。

	pub async fn refresh_microsoft(&self, refresh_token: String) -> Result<AccountResult, i32>
		通过refresh_token获取到AccountResult实现
		该函数用于刷新你的access_token。你可以从第一次登录微软账号时，获取到的refresh_token进行刷新。

	async fn thirdparty() -> Result<Vec<AccountResult>, i32>
		私有函数，通过（）获取到AccountResult实现数组。

	pub async fn login_thirdparty(username: &str, password: &str, client_token: &str) -> Result<Vec<AccountResult>, i32>
		通过用户名和密码登录外置服务器，client_token为可选项，你当然可以填空来确保其没有。
		一般来说会是有多个用户，所以这里采用了返回一个Vec<AccountResult>作为返回值。
		如果里面只有一个用户，则返回len为1，如果还并没有使用任何一个皮肤，则会返回一个指定Err常量，因此，该Vec最小数量只能是1。

	pub async fn refresh_thirdparth(access_token: String, client_token: String) -> Result<AccountResult, i32>
		刷新第三方登录，这里仅会返回一个AccountResult，为刷新后的值。
		这里采用的是access_token进行刷新，因为第三方登录暂时没有refresh_token。
		如果你有提供client_token的话，也可以使用client_token进行刷新。
		最终返回的AccountResult里，仅包含了新的access_token、以及client_token（如果你提供了的话。）
```

## rust_lib::download_mod

```
以下函数均使用异步，望周知。
以下五个函数均受到【rust_lib::some_var::DOWNLOAD_SOURCE】的影响，下载源为1时是官方下载源，为2时是BMCLAPI镜像。
pub async fn get_mc_versions() -> Result<serde_json::Value, i32> 
	获取MC原始metadata数据，并返回serde_json的Value值。
	该值可以直接赋值给MC_ROOT_JSON

pub async fn get_forge_versions(mcversion: &str) -> Result<serde_json::Value, i32>
	该值可以通过给予一个MC的确切版本，准确获取到一个TLM格式的serde_json的Value值。
	如果获取不到或者内容为空，则返回Err
	该TLM标准格式，可以通过查看该库的README.md进行查看，当然你也可以直接运行一次查看。

pub async fn get_fabric_version(mcversion: &str) -> Result<serde_json::Value, i32>
	同上。但是是Fabric的。
	installer键值改成profile，因为这个是直接获取原始JSON的。

pub async fn get_quilt_version(mcversion: &str) -> Result<serde_json::Value, i32>
	同上。但是是Quilt的。

pub async fn get_neoforge_version(mcversion: &str) -> Result<serde_json::Value, i32>
	同Forge，但是是NeoForge的。
```