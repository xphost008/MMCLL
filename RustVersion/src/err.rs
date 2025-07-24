use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MMCLLError {
    UnknownError(i32),
    LaunchAccountUsername,
    LaunchAccountUserUUID,
    LaunchAccountAccessToken,
    LaunchAccountNoLegal,
    LaunchAccountThirdpartyAccessTokenOrURL,
    LaunchAccountThirdpartyBase,
    LaunchJavaPath,
    LaunchRootPath,
    LaunchVersionPath,
    LaunchGamePath,
    LaunchWidth,
    LaunchHeight,
    LaunchMinMemory,
    LaunchMaxMemory,
    LaunchCustomInfo,
    LaunchAccountAuthlib,
    GameEligibleJsonNotFound,
    GameInheritsFromVersionLose,
    GameCannotUnzipNative,
    GameInheritsJsonStructure,
    GameRawJsonStructure,
    LoginCannotGetUserCode,
    LoginDeviceCodeInvalid,
    LoginTimeout,
    LoginRefreshTokenExpire,
    LoginXboxLiveInvalid,
    LoginXstsLiveInvalid,
    LoginXstsNoXbox,
    LoginXstsIllegal,
    LoginXstsNoAdult,
    LoginXstsUnder18,
    LoginXboxXstsUsercode,
    LoginMcInvalid,
    LoginNoMinecraft,
    LoginCannotGetMetadata,
    LoginUsernameOrPassword,
    LoginInvalidAccessToken,
    LoginAccessTokenExpire,
    DownloadCannotGetMetadata,
    DownloadForgeVersionNotFound,
    DownloadFabricVersionNotFound,
    DownloadQuiltVersionNotFound,
    DownloadNeoforgeVersionNotFound,
    DownloadArgumentsError,
    DownloadFileExists,
    DownloadCannotCreateDir,
    DownloadNotSupportSystem,
    DownloadFileDownloadFailure,
    DownloadDownloadFailure,
    UnreachablePosition(i32),
}

impl std::error::Error for MMCLLError {}

impl Display for MMCLLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MMCLLError::UnknownError(id) => write!(f, "未知错误 编号: {}", id),
            MMCLLError::LaunchAccountUsername => write!(f, "账号名称格式错误"),
            MMCLLError::LaunchAccountUserUUID => write!(f, "账号UUID格式错误"),
            MMCLLError::LaunchAccountAccessToken => {
                write!(f, "账号AccessToken错误（仅在登录微软正版时触发）")
            }
            MMCLLError::LaunchAccountNoLegal => {
                write!(f, "账号未购买正版（仅在登录微软正版时触发）")
            }
            MMCLLError::LaunchAccountThirdpartyAccessTokenOrURL => write!(
                f,
                "账号第三方的AccessToken或者URL错误。（仅在登录第三方时触发）"
            ),
            MMCLLError::LaunchAccountThirdpartyBase => {
                write!(f, "账号base64编码错误（仅在登录第三方时触发）")
            }
            MMCLLError::LaunchJavaPath => write!(f, "Java路径错误（文件未找到）"),
            MMCLLError::LaunchRootPath => write!(f, "游戏根路径错误（文件夹未找到）"),
            MMCLLError::LaunchVersionPath => write!(f, "游戏版本路径错误（文件夹未找到）"),
            MMCLLError::LaunchGamePath => write!(f, "游戏实际路径错误（文件夹未找到）"),
            MMCLLError::LaunchWidth => write!(f, "窗口宽度错误（小于854或大于屏幕宽度）"),
            MMCLLError::LaunchHeight => write!(f, "窗口高度错误（小于480或大于屏幕高度）"),
            MMCLLError::LaunchMinMemory => write!(f, "最小内存错误（小于256或大于1024）"),
            MMCLLError::LaunchMaxMemory => write!(f, "最大内存错误（小于1024或大于系统内存）"),
            MMCLLError::LaunchCustomInfo => write!(f, "自定义信息错误（未填写，必须要个默认值！）"),
            MMCLLError::LaunchAccountAuthlib => {
                write!(f, "Authlib-Injector.jar未找到。（仅在登录第三方时触发）")
            }
            MMCLLError::GameEligibleJsonNotFound => write!(
                f,
                "无法在版本文件夹中读取到版本JSON。（需要重新下载一遍MC）"
            ),
            MMCLLError::GameInheritsFromVersionLose => write!(
                f,
                "已从版本json中找到inheritsFrom键，但是无法找到依赖版本。（需要重新下载一遍MC）"
            ),
            MMCLLError::GameCannotUnzipNative => write!(f, "无法解压natives文件，请重试！"),
            MMCLLError::GameInheritsJsonStructure => write!(
                f,
                "已获取到inheritsFrom键下的版本json，但是结构不对。（比如找不到mainClass等。）"
            ),
            MMCLLError::GameRawJsonStructure => write!(f, "JSON的结构不对。"),
            MMCLLError::LoginCannotGetUserCode => write!(f, "未成功获取用户代码"),
            MMCLLError::LoginDeviceCodeInvalid => write!(f, "微软登录，暂未完成登录。"),
            MMCLLError::LoginTimeout => {
                write!(f, "微软登录，登录超时（15分钟未完成登录），请重试！")
            }
            MMCLLError::LoginRefreshTokenExpire => write!(f, "微软登录，刷新密钥也同样过期了。"),
            MMCLLError::LoginXboxLiveInvalid => {
                write!(f, "在进行xbox登录时出现了错误，可能是没挂vβn的原因。")
            }
            MMCLLError::LoginXstsLiveInvalid => {
                write!(f, "在进行xsts登录时出现了错误，可能是没挂vβn的原因。")
            }
            MMCLLError::LoginXstsNoXbox => write!(
                f,
                "在进行xsts登录时，由于该账户没有xbox账号，你可能需要自己注册一个。"
            ),
            MMCLLError::LoginXstsIllegal => {
                write!(f, "在进行xsts登录时，由于该国家/地区被禁止，无法登录。")
            }
            MMCLLError::LoginXstsNoAdult => write!(f, "该账号需要成人验证（韩国）。"),
            MMCLLError::LoginXstsUnder18 => {
                write!(f, "该账号设置未满18周岁，需要成人将该账户添加到家庭组中。")
            }
            MMCLLError::LoginXboxXstsUsercode => write!(
                f,
                "你请求的xbox usercode与xsts usercode二者不一致，请重新尝试！"
            ),
            MMCLLError::LoginMcInvalid => {
                write!(f, "在进行mc登录时出现了错误，可能是没挂vβn的原因。")
            }
            MMCLLError::LoginNoMinecraft => write!(f, "该账号暂未购买mc，请重新尝试！"),
            MMCLLError::LoginCannotGetMetadata => write!(f, "第三方登录：无法获取皮肤站元数据。"),
            MMCLLError::LoginUsernameOrPassword => write!(f, "第三方登录：账号密码错误。"),
            MMCLLError::LoginInvalidAccessToken => write!(f, "第三方登录：无效的令牌。"),
            MMCLLError::LoginAccessTokenExpire => {
                write!(f, "第三方登录，用于刷新的登录密钥已过期很久。")
            }
            MMCLLError::DownloadCannotGetMetadata => write!(f, "获取元数据失败（适用于任何情况）"),
            MMCLLError::DownloadForgeVersionNotFound => write!(f, "forge版本未找到"),
            MMCLLError::DownloadFabricVersionNotFound => write!(f, "fabric版本未找到"),
            MMCLLError::DownloadQuiltVersionNotFound => write!(f, "quilt版本未找到"),
            MMCLLError::DownloadNeoforgeVersionNotFound => write!(f, "neoforge版本未找到"),
            MMCLLError::DownloadArgumentsError => write!(f, "填入参数有误"),
            MMCLLError::DownloadFileExists => write!(f, "文件已存在"),
            MMCLLError::DownloadCannotCreateDir => write!(f, "无法创建父文件夹"),
            MMCLLError::DownloadNotSupportSystem => write!(f, "不支持的系统"),
            MMCLLError::DownloadFileDownloadFailure => write!(f, "文件下载失败"),
            MMCLLError::DownloadDownloadFailure => write!(f, "整体某一线程出现失误"),
            MMCLLError::UnreachablePosition(i) => write!(f, "xphost认为不可达的位置 编号: {}", i),
        }
    }
}

impl From<i32> for MMCLLError {
    fn from(value: i32) -> Self {
        if value > 0 {
            MMCLLError::UnreachablePosition(value)
        } else {
            MMCLLError::UnknownError(value)
        }
    }
}

pub type MMCLLResult<T> = Result<T, MMCLLError>;