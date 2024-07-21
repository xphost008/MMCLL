unit DelphiLauncherLib;
interface
uses
  Classes, SysUtils, JSON, IOUtils, StrUtils, Character, RegularExpressions;
type
  TLauncherLoginType = class
  private
    server, playerName, UUID, accessToken, baseCode: String;
  public
    constructor CreateOffline(playerName, UUID: String); overload;
    constructor CreateMicrosoft(playerName, UUID, accessToken: String); overload;
    constructor CreateThird(playerName, UUID, accessToken, server, baseCode: String); overload;
  end;
  TLauncherParam = class
  private
    height, Width, port: Integer;
    customInfo, AdditionJVM, AdditionGame, server: String;
  public
    procedure SetWindowHeight(height: Integer);
    procedure SetWindowWidth(width: Integer);
    procedure SetCustomInfo(customInfo: String);
    procedure SetAdditionJVM(AdditionJVM: String);
    procedure SetAdditionGame(AdditionGame: String);
    procedure SetServerPort(server: String; port: Integer);
  TLauncherMain = class
  private
    javaPath: String;
    json: TJSONObject;
    function GetLibraries(mcpath, mcselpath: String): String;
    function ConvertNameToPath(name: string): string;
    function ExtractNumber(str: String; bo: Boolean): String;
  public
    constructor Create(json: String); overload;
    constructor Create(json: TJsonObject); overload;
    procedure SetJavaPath(javaPath: String);
    procedure StartLaunch(mcPath, mcSelPath: String; maxMemory: Integer; LoginType: TLauncherLoginType; ParamIn: TLauncherParam);
  end;
  TLauncherOrigin = class
  public
    class function GetFile(path: String): String;
    class function GetVersionList(mcpath: String): TStringList;
  end;
implementation
/// <summary>
/// 公有函数
/// 设置窗口高度
/// </summary>
/// <param name="height">输入窗口高度</param>
procedure TLauncherParam.SetWindowHeight(height: Integer);
begin
  self.height := height;
end;
/// <summary>
/// 公有函数
/// 设置窗口宽度
/// </summary>
/// <param name="width">输入窗口宽度</param>
procedure TLauncherParam.SetWindowWidth(width: Integer);
begin
  self.width := width;
end;
/// <summary>
/// 公有函数
/// 设置自定义信息
/// </summary>
/// <param name="customInfo">输入自定义信息</param>
procedure TLauncherParam.SetCustomInfo(customInfo: Integer);
begin
  self.height := height;
end;
/// <summary>
/// 公有函数
/// 设置额外JVM参数
/// </summary>
/// <param name="AdditionJVM">输入额外JVM参数</param>
procedure TLauncherParam.SetAdditionJVM(AdditionJVM: String);
begin
  self.AdditionJVM := AdditionJVM;
end;
/// <summary>
/// 公有函数
/// 设置额外Game参数
/// </summary>
/// <param name="AdditionGame">输入额外Game参数</param>
procedure TLauncherParam.SetAdditionGame(AdditionGame: String);
begin
  self.AdditionGame := AdditionGame;
end;
/// <summary>
/// 公有函数
/// 设置服务器地址、端口
/// </summary>
/// <param name="server">输入服务器地址</param>
/// <param name="port">输入服务器端口</param>
/// <raise Exception>如果服务器端口小于1024或者大于65535则报错。</raise>
procedure TLauncherParam.SetServerPort(server: String; port: Integer);
begin
  if port < 1024 or port > 65535 then raise Exception.Create('port wrong!');
  self.server := server;
  self.port := port;
end;
/// <summary>
/// 公有函数
/// 调用所有的参数，然后启动游戏。
/// </summary>
/// <param name="mcPath">MC根目录</param>
/// <param name="mcSelPath">MC版本目录“.minecraft/version/<版本名>”</param>
/// <param name="maxMemory">填入最大内存</param>
/// <param name="LoginType">登录方式，填入TLauncherLoginType中的任意一个</param>
/// <param name="ParamIn">启动参数，设置一个TLauncherParam的类，然后在里面添加参数，最后将一整个类填入即可。</param>
procedure TLauncherMain.StartLaunch(mcPath, mcSelPath: String; maxMemory: Integer; LoginType: TLauncherLoginType; ParamIn: TLauncherParam);
begin
  // {TODO: Launcher Game}
end;
/// <summary>
/// 私有函数
/// TLauncherMain的截取字符串中的所有数字或所有字符
/// </summary>
/// <param name="str">填入原字符串</param>
/// <param name="bo">如果为true，则摘取数字，反之摘取字符。</param>
/// <return>摘取字符串后的字符串</return>
function TLauncherMain.ExtractNumber(str: String; bo: Boolean): String;
begin
  var Temp := '';
  if str = '' then
  begin
    result := '';
    exit;
  end;
  for var I in str do
  begin
    if bo then
    begin
      if I.IsNumber then
        Temp := Concat(Temp, I);
    end
    else
    begin
      if not I.IsNumber then
        Temp := Concat(Temp, I);
    end;
  end;
  result := Temp;
end;
/// <summary>
/// 私有函数
/// TLauncherMain的将libraries中的name键转换成路径
/// </summary>
/// <param name="name">填入libraries中的name键</param>
/// <return>返回path路径</return>
function TLauncherMain.ConvertNameToPath(name: string): string;
begin
  var c1 := TStringList.Create;
  var c2 := TStringList.Create;
  var all := TStringList.Create;
  var sb := TStringBuilder.Create;
  try
    var n1 := name.Substring(0, name.IndexOf(':'));
    var n2 := name.Substring(name.IndexOf(':') + 1, name.Length);
    ExtractStrings(['.'], [], pchar(n1), c1);
    for var I in c1 do all.Add(Concat(I, '\\'));
    ExtractStrings([':'], [], pchar(n2), c2);
    for var I := 0 to c2.Count - 1 do begin
      if c2.Count >= 3 then begin
        if I < c2.Count - 1 then begin
          all.Add(Concat(c2[I], '\\'));
        end;
      end else all.Add(Concat(c2[I], '\\'));
    end;
    for var I := 0 to c2.Count - 1 do begin
      if I < c2.Count - 1 then begin
        all.Add(Concat(c2[I], '-'));
      end else begin
        all.Add(Concat(c2[I], '.jar'));
      end;
    end;
    for var I in all do sb.Append(I);
    result := sb.ToString;
  finally
    c1.Free;
    c2.Free;
    all.Free;
    sb.Free;
  end;
end;
/// <summary>
/// 私有函数
/// TLauncherMain的获取Libraries库的函数
/// </summary>
/// <param name="mcpath">填入mc根目录</param>
/// <param name="mcselpath">填入mc版本目录</param>
/// <return>返回所有库的连接后</return>
function TLauncherMain.GetLibraries(mcpath, mcselpath: String): String;
begin
  var sb := TStringBuilder.Create;
  var Yuan := TStringList.Create;
  var LibNo := TStringList.Create;
  var NoRe := TStringList.Create;
  var ReTemp := TStringList.Create;
  try
    for var i in (json.GetValue('libraries') as TJsonArray) do begin
      var key := i as TJsonObject;
      var judge := true;
      try
        var rl := key.GetValue('rules') as TJsonArray;
        for var J in rl do begin
          var r1 := J as TJsonObject;
          var an := r1.GetValue('action').Value;
          if an = 'allow' then begin
            var r2 := r1.GetValue('os') as TJsonObject;
            var r3 := r2.GetValue('name').Value;
            if r3 <> 'windows' then begin judge := false; end;
          end else if an = 'disallow' then begin
            var r2 := r1.GetValue('os') as TJsonObject;
            var r3 := r2.GetValue('name').Value;
            if r3 = 'windows' then begin judge := false; end;
          end;
        end;
      except
      end;
      try
        var r1 := key.GetValue('natives').ToString;
        judge := false;
      except end;
      try
        var r1 := key.GetValue('downloads') as TJsonObject;
        var r2 := r1.GetValue('classifiers').ToString;
        judge := false;
        var r3 := r1.GetValue('artifact').ToString;
        judge := true;
      except end;
      if not judge then continue;
      try
        Yuan.Add(key.GetValue('name').Value);
      except
        continue;
      end;
    end;
    for var i in Yuan do
      if LibNo.IndexOf(i) = -1 then
        LibNo.Add(i);
    for var i in LibNo do begin
      var KN := i.Replace('.', '').Replace(':', '').Replace('-', '');
      var KW := ExtractNumber(KN, false);
      var KM := ExtractNumber(KN, true);
      if ReTemp.IndexOf(KW) = -1 then begin
        ReTemp.Add(KW);
        NoRe.Add(i);
      end else if strtoint64(ExtractNumber(NoRe[ReTemp.IndexOf(KW)], true)) <= strtoint64(KM) then begin
        NoRe.Delete(ReTemp.IndexOf(KW));
        NoRe.Insert(ReTemp.IndexOf(KW), i);
      end;
    end;
    for var I in NoRe do sb.Append(Concat(mcpath, '\\libraries\\', ConvertNameToPath(I), ';'));
    sb.Append(Concat(mcselpath, '\', ExtractFileName(mcselpath), '.jar'));
    result := sb.ToString;
  finally
    sb.Free;
    Yuan.Free;
    LibNo.Free;
    NoRe.Free;
    ReTemp.Free;
  end;
end;
/// <summary>
/// 公有函数
/// TLauncherMain的设置Java路径
/// </summary>
/// <param name="javaPath">填入Java路径，精确到java或者javaw的exe文件。</param>
/// <raise EFileNotFountException>如果文件路径末尾不为java.exe或者javaw.exe，则报错。</raise>
procedure TLauncherMain.SetJavaPath(javaPath: String);
begin
  if (RightStr(javaPath, 9) <> 'javaw.exe') and (RightStr(javaPath, 8) <> 'java.exe') then raise EFileNotFoundException.Create('Java path not right!');
  self.javaPath := javaPath;
end;
/// <summary>
/// 公有函数
/// TLauncherLoginType的构造函数，此为离线版本的构造函数。
/// </summary>
/// <param name="playerName">填入玩家名称</param>
/// <param name="UUID">填入玩家UUID</param>
constructor TLauncherLoginType.CreateOffline(playerName, UUID: String);
begin
  if not TRegex.IsMatch() then raise 
  self.playerName := playerName;
  self.UUID := UUID;
end;
/// <summary>
/// 公有函数
/// TLauncherLoginType的构造函数，此为微软版本的构造函数。
/// </summary>
/// <param name="playerName">填入玩家名称</param>
/// <param name="UUID">填入玩家UUID</param>
/// <param name="accessToken">填入玩家登录令牌</param>
constructor TLauncherLoginType.CreateMicrosoft(playerName, UUID, accessToken: String);
begin
  self.playerName := playerName;
  self.UUID := UUID;
  self.accessToken := accessToken;
end;
/// <summary>
/// 公有函数
/// TLauncherLoginType的构造函数，此为第三方版本的构造函数。
/// </summary>
/// <param name="server">填入认证服务器地址</param>
/// <param name="playerName">填入玩家名称</param>
/// <param name="UUID">填入玩家UUID</param>
/// <param name="accessToken">填入玩家登录令牌</param>
/// <param name="accessToken">填入认证服务器元数据被base64编码后的数据</param>
constructor TLauncherLoginType.CreateThird(playerName, UUID, accessToken, server, baseCode: String);
begin
  self.server := server;
  self.playerName := playerName;
  self.UUID := UUID;
  self.accessToken := accessToken;
  self.baseCode := baseCode;
end;
/// <summary>
/// 公有函数
/// TLauncherMain的构造函数，需要填入版本主json文件。
/// </summary>
/// <param name="json">填入json字符串。</param>
constructor TLauncherMain.Create(json: String);
begin
  self.json := TJsonObject.ParseJSONValue(json) as TJsonObject;
end;
/// <summary>
/// 公有函数
/// TLauncherMain的构造函数，需要填入版本主json的对象形式。
/// </summary>
/// <param name="json">填入Json对象</param>
constructor TLauncherMain.Create(json: TJsonObject);
begin
  self.json := json;
end;
/// <summary>
/// 公有函数
/// TLauncherMain的获取外部文件函数。
/// 此为一个静态类，可以无需初始化即可调用。
/// </summary>
/// <param name="path">填入外部文件路径</param>
/// <return>返回path中文件内的内容</return>
/// <raise EFileNotFoundException>如果文件不存在，则会发出报错。</raise>
class function TLauncherOrigin.GetFile(path: String): String;
begin
  if not FileExists(path) then raise EFileNotFoundException.Create('File Not Found');
  var ss := TStringStream.Create('', TEncoding.UTF8, False);
  try
    ss.LoadFromFile(path);
    result := ss.DataString;
  finally
    ss.Free;
  end;
end;
/// <summary>
/// 公有函数
/// TLauncherMain的获取Minecraft根目录下的所有版本
/// 此为一个静态类，可以无需初始化即可调用。
/// </summary>
/// <param name="mcpath">填入Minecraft根路径（末尾可以不用加左划号）</param>
/// <return>返回一个字符串列表，可用for循环批量读取。</return>
/// <raise EDirectoryNotFoundException>如果文件夹不存在，则会发出报错。</raise>
class function TLauncherOrigin.GetVersionList(mcpath: String): TStringList;
var
  Dir: TArray<String>;
begin
  if mcpath.LastIndexOf('/') <> mcpath.Length then mcpath := Concat(mcpath, '/');
  if not DirectoryExists(mcpath) then raise EDirectoryNotFoundException.Create('Directory Not Found');
  result := TStringList.Create;
  mcpath := Concat(mcpath, 'versions/');
  Dir := TDirectory.GetDirectories(mcpath);
  for var I in Dir do begin
    result.Add(I);
  end;
end;
end.