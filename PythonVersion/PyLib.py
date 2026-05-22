#-------------------------------------------------------------------------------
# Name:        PyLib
# Purpose:     MC启动器类库
#
# Author:      xphost
#
# Created:     08/01/2024
# Copyright:   (c) xphost 2024
# Licence:     MIT
#-------------------------------------------------------------------------------
#!/usr/bin/python
# -*- coding: UTF-8 -*-
from json import JSONDecodeError  # JSON解析错误
from typing import *  # 类型模块
from zipfile import BadZipfile

import screeninfo  # 屏幕模块
import psutil  # 内存模块
import re  # 正则表达式模块
import os  # 系统模块
import json  # json模块
import hashlib  # 加密模块

import httpx  # 网络请求模块
import zipfile  # zip解压库

class LaunchException(Exception):
    """
    定义一个启动错误类型，以便于玩家不必使用太多的【异常语句过于宽泛】
    """
    ...

# 一些简单的定义常量，定义启动游戏时可能出现的错误。
# 以下是固定常量
OK = 0  # 参数检查无误
ERR_UNKNOWN_ERROR = 1  # 未知错误
LAUNCHER_NAME = "MMCLL"
LAUNCHER_VERSION = "0.0.1.9"
USER_AGENT = "MMCLL/0.0.1.9"
# 以下是启动游戏可能出现的错误
ERR_LAUNCH_ACCOUNT_USERNAME = -1  # 用户名错误
ERR_LAUNCH_ACCOUNT_USERUUID = -2  # 用户UUID错误
ERR_LAUNCH_ACCOUNT_ACCESS_TOKEN = -3  # 用户正版令牌错误
ERR_LAUNCH_ACCOUNT_NO_LEGAL = -4  # 用户未购买正版
ERR_LAUNCH_ACCOUNT_THIRDPARTY_ACCESS_TOKEN_OR_URL = -5  # 第三方外置登录的URL或者AccessToken的数据错误。
ERR_LAUNCH_ACCOUNT_THIRDPARTY_BASE = -6  # 第三方外置登录的Base64编码错误
ERR_LAUNCH_JAVA_PATH = -7  # Java路径错误
ERR_LAUNCH_ROOT_PATH = -8  # 根路径错误
ERR_LAUNCH_VERSION_PATH = -9  # 版本路径错误
ERR_LAUNCH_GAME_PATH = -10  # 游戏实例路径错误
ERR_LAUNCH_WINDOW_WIDTH = -11  # 窗口高度错误
ERR_LAUNCH_WINDOW_HEIGHT = -12  # 窗口宽度错误
ERR_LAUNCH_MIN_MEMORY = -13  # 最小内存错误
ERR_LAUNCH_MAX_MEMORY = -14  # 最大内存错误
ERR_LAUNCH_CUSTOM_INFO = -15  # 自定义信息错误

# 以下是部分全局变量，使用时需要使用global调用
download_source = 1  # 下载源（1是官方源，2是bmclapi）
mc_root_json = {}  # MC元数据（该变量为一个dict类型的！）
class UrlMethod:
    """
    网络请求类。
    """
    def __init__(self, url: str):
        """
        初始化一个url
        :param url:  填入url以初始化
        """
        self.url = url
    def get(self, key: str) -> str:
        """
        获取网络内容，但是需要输入一个令牌身份验证
        :param key:  令牌内容
        :return:  请求内容
        """
        # noinspection PyBroadException
        try:
            with httpx.Client() as h:
                headers = {
                    "User-Agent": USER_AGENT,
                    "Authorization": f"Bearer {key}"
                }
                res = h.get(self.url, headers=headers)
                return res.text
        except:
            return ""
    def post(self, key: str, that: bool) -> str:
        """
        进行post请求，但是需要输入一个data内容，可以是json也可以是网址后缀。
        :param key:  请求内容
        :param that:  是json则为false，否则为true。
        :return:  返回响应内容
        """
        # noinspection PyBroadException
        try:
            with httpx.Client() as h:
                headers = {
                    "User-Agent": USER_AGENT,
                    "Content-Type": "application/x-www-form-urlencoded" if that else "application/json",
                    "Accept": "application/json" if not that else ""
                }
                if not that:
                    key = json.loads(key)
                res = h.post(self.url, headers=headers, data=key)
                return res.text
        except:
            return ""
    def get_default(self) -> bytes:
        """
        以默认方式获取网络请求。但是返回的是二进制类型。既可以保存成文本，也可以保存到二进制文件。
        :return: 返回请求文本
        """
        # noinspection PyBroadException
        try:
            with httpx.Client() as h:
                headers = {
                    "User-Agent": USER_AGENT
                }
                res = h.get(self.url, headers=headers)
                return res.content
        except:
            return b""

    async def get_async(self, key) -> str:
        """
        带令牌的get请求，但是是异步的，下面三个相同。
        :param key:  令牌
        :return:  返回值
        """
        # noinspection PyBroadException
        try:
            async with httpx.AsyncClient() as h:
                headers = {
                    "User-Agent": USER_AGENT,
                    "Authorization": f"Bearer {key}"
                }
                res = await h.get(self.url, headers=headers)
                return res.text
        except:
            return ""

    async def post_async(self, key, that) -> str:
        """
        同上的post请求，但是是异步
        :param key:  请求内容
        :param that:  是json则为false，否则为true。
        :return:  返回响应内容
        """
        # noinspection PyBroadException
        try:
            async with httpx.AsyncClient() as h:
                headers = {
                    "User-Agent": USER_AGENT,
                    "Content-Type": "application/x-www-form-urlencoded" if that else "application/json",
                    "Accept": "application/json" if not that else ""
                }
                if not that:
                    key = json.loads(key)
                res = await h.post(self.url, headers=headers, data=key)
                return res.text
        except:
            return ""

    async def get_default_async(self) -> bytes:
        """
        以默认方式获取网络请求。但是是异步
        :return: 返回请求文本
        """
        # noinspection PyBroadException
        try:
            async with httpx.AsyncClient() as h:
                headers = {
                    "User-Agent": USER_AGENT
                }
                res = await h.get(self.url, headers=headers)
                return res.content
        except:
            return b""


class MainClass:
    """
    该函数里存了很多static的函数，其中基本为启动或登录账号时需要用到的。
    """
    @staticmethod
    def get_file(path: str) -> str:
        """
        获取文件内容，通常获取的是文件的内容，非二进制文件。
        :param path:  文件路径
        :return:  文件内容
        """
        if os.path.exists(path) and os.path.isfile(path):
            try:
                with open(path, "r", encoding="utf-8") as fe:
                    return fe.read()
            except PermissionError:
                return ""
        else:
            return ""

    @staticmethod
    def generate_bukkit_uuid(name: str) -> str:
        name = "OfflinePlayer:" + name
        m = hashlib.md5()
        m.update(name.encode("utf-8"))
        l = list(m.digest())
        l[6] = l[6] & 0x0f | 0x30
        l[8] = l[8] & 0x3f | 0x80
        res = ""
        for i in l:
            res += hex(i).replace("0x", "").zfill(2).lower()
        return res

    @staticmethod
    def set_file(path: str, content: str):
        """
        写出内容到文件，通常写出的是普通文字而非二进制。
        :param path:  文件路径
        :param content:  内容字符串
        """
        try:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf-8") as fe:
                fe.write(content)
        except PermissionError:
            return ""

    @staticmethod
    def get_sha1(path: str) -> str:
        """
        获取文件sha1，可以填入二进制文件，也可以填入普通文件。
        :param path:  文件路径
        :return:  文件内容
        """
        if os.path.exists(path) and os.path.isfile(path):
            try:
                sha = hashlib.sha1()
                with open(path, "rb") as fe:
                    for chunk in iter(lambda: fe.read(4096), b""):
                        sha.update(chunk)
                return sha.hexdigest()
            except PermissionError:
                return ""
        else:
            return ""


    @staticmethod
    def get_nested_value(data, *keys, default_value=None):
        """
        通过一个json，安全的获取到里面的值，如果获取不到或者类型与default_value不匹配，则返回default_value。
        :param data: 原json
        :param keys: 要获取的字符串
        :param default_value: 默认值
        :return:
        """
        current = data
        try:
            for key in keys:
                if (isinstance(current, dict) and isinstance(key, str) and key in current) or (isinstance(current, list) and isinstance(key, int) and 0 <= key < len(current)):
                    current = current[key]
                else:
                    return default_value
            if not isinstance(current, type(default_value)):
                return default_value
        except ValueError:
            return default_value
        return current

    @staticmethod
    def delete_dir_retain(path: str, suffix: str = ""):
        """
        该函数将删除path中的所有文件，但是唯独保留了
        :param path:
        :param suffix:
        :return:
        """
        # 遍历目录中的所有文件和子目录
        if not os.path.exists(path):
            return False
        for filename in os.listdir(path):
            file_path = os.path.join(path, filename)
            if os.path.isfile(file_path):
                if os.path.splitext(file_path)[1] == suffix:
                    continue
                os.remove(file_path)
            elif os.path.isdir(file_path):
                MainClass.delete_dir_retain(file_path, suffix)
                try:
                    os.rmdir(file_path)
                except OSError:
                    continue
        return True

    @staticmethod
    def unzip(zippath: str, extpath: str) -> bool:
        if os.path.exists(zippath) and os.path.isfile(zippath):
            if not os.path.exists(extpath) or os.path.exists(extpath) and os.path.isfile(extpath):
                try:
                    os.makedirs(extpath)
                except OSError:
                    return False
            with zipfile.ZipFile(zippath, "r") as fe:
                try:
                    fe.extractall(extpath)
                except BadZipfile:
                    return False
                return True
        else:
            return False
class LaunchMethod:
    """
    该类全部都是静态函数，为启动游戏专用的函数，各位如果想实现自己的启动逻辑，可以参考下方的LaunchGame类进行调用本类的函数。
    """
    @staticmethod
    def get_mc_vanilla_version(root: str) -> str:
        root = json.loads(root)
        cid = MainClass.get_nested_value(root, "clientVersion", default_value="")
        if cid:
            return cid
        patch = MainClass.get_nested_value(root, "patches", default_value=[])
        for i in patch:
            idid = MainClass.get_nested_value(i, "id", default_value="")
            if not idid: continue
            if idid == "game":
                mcid = MainClass.get_nested_value(i, "version", default_value="")
                if not mcid: continue
                return mcid
        raw_release_time = MainClass.get_nested_value(root, "releaseTime", default_value="")
        if not raw_release_time:
            global download_source
            global mc_root_json
            if download_source == 2:
                v = "https://bmclapi2.bangbang93.com/mc/game/version_manifest.json"
            else:
                v = "https://piston-meta.mojang.com/mc/game/version_manifest.json"
            if not mc_root_json:
                url = UrlMethod(v)
                pos = url.get_default()
                if pos != b"":
                    mc_root_json = json.loads(pos.decode("utf-8"))
            if mc_root_json:
                ver = MainClass.get_nested_value(mc_root_json, "versions", default_value=[])
                for i in ver:
                    rel = MainClass.get_nested_value(i, "releaseTime", default_value="")
                    if rel == raw_release_time:
                        return MainClass.get_nested_value(i, "id", default_value="")
        idid = MainClass.get_nested_value(root, "id", default_value="")
        if idid:
            return idid
        return ""
    @staticmethod
    def get_mc_real_path(version_path: str, suffix: str) -> str:
        """
        通过version_path以及suffix准确找到文件。
        你可以讲suffix指定为【.json】或者【文件的sha1】以供精确查找。
        当suffix为后缀且等于.json的时候，
        该函数不会递归查询，请注意！
        :param version_path:  要查询的文件夹
        :param suffix:  该文件的某个特殊信息【例如后缀或sha1】
        :return:  从文件夹中遍历找到的第一个符合该suffix的文件。
        """
        if os.path.exists(version_path) and os.path.isdir(version_path):
            for i in os.listdir(version_path):
                i = os.path.join(version_path, i)
                if os.path.isdir(i): continue
                if suffix in i:
                    if suffix == ".json" and i[i.rindex("."):] == ".json":
                        file_content = MainClass.get_file(i)
                        try:
                            root = json.loads(file_content)
                        except JSONDecodeError:
                            continue
                        libr = MainClass.get_nested_value(root, "libraries", default_value=[])
                        if not libr: continue
                        mics = MainClass.get_nested_value(root, "mainClass", default_value="")
                        if not mics: continue
                        idid = MainClass.get_nested_value(root, "id", default_value="")
                        if not idid: continue
                        return i
                    else:
                        return i
                elif "." not in suffix:
                    sha = MainClass.get_sha1(i)
                    if not sha: continue
                    if sha == suffix:
                        return i

        return ""
    @staticmethod
    def get_mc_inherits_from(version_path: str, ioj: str) -> str:
        """
        通过json中的inheritsFrom键，准确找到对应的原版键所在的文件夹。
        :param version_path: 原来的版本文件夹
        :param ioj: 是inheritsFrom还是jar
        :return: 如果找到了inheritsFrom，则返回原版文件夹，否则直接返回原始文件夹。
        """
        if os.path.exists(version_path) and os.path.isdir(version_path):
            real_path = LaunchMethod.get_mc_real_path(version_path, ".json")
            real_file = MainClass.get_file(real_path)
            try:
                root = json.loads(real_file)
            except JSONDecodeError:
                return ""
            fe = MainClass.get_nested_value(root, ioj, default_value="")
            if fe:
                parent = os.path.dirname(version_path)
                for i in os.listdir(parent):
                    i = os.path.join(parent, i)
                    if os.path.isfile(i): continue
                    version_json = LaunchMethod.get_mc_real_path(i, ".json")
                    if version_json == "": continue
                    json_content = MainClass.get_file(version_json)
                    if json_content == "": continue
                    vanilla_version = LaunchMethod.get_mc_vanilla_version(json_content)
                    if vanilla_version == "": continue
                    if vanilla_version == fe: return i
            else:
                return version_path
        return ""

    @staticmethod
    def convert_name_to_path(name: str) -> str:
        """
        讲libraries中的name转换成path形式
        :param name: 要转换的name
        :return: 转换成功的path
        """
        if name.find("@") != -1:
            rf = name.rfind("@")
            suffix = name[rf + 1:]
            name = name[:rf]
        else:
            suffix = "jar"
        spl = name.split(":")
        if len(spl) == 4:
            return "{}\\{}\\{}\\{}-{}-{}.{}".format(spl[0].replace(".", "\\"), spl[1], spl[2], spl[1], spl[2], spl[3], suffix)
        elif len(spl) == 3:
            return "{}\\{}\\{}\\{}-{}.{}".format(spl[0].replace(".", "\\"), spl[1], spl[2], spl[1], spl[2], suffix)
        else:
            return ""

    @staticmethod
    def extract_number(ext: str, is_num: bool) -> str:
        """
        截取数字和字符
        :param ext: 要截取的字符串
        :param is_num: 如果为True，则截取字符串里的所有数字，否则截取字符串中的所有非数字。
        :return: 截取后的字符串。
        """
        res = ""
        if len(ext) == 0: return ""
        for i in ext:
            if is_num == i.isnumeric():
                res += i
        return res

    @staticmethod
    def judge_arguments(root: dict, key: str) -> list:
        """
        判断argument启动参数，并返回根列表
        :param root: 根json
        :param key: 判断的键（一般是game或jvm）
        :return: 返回参数列表
        """
        arg = list(MainClass.get_nested_value(root, "arguments", key, default_value=[]))
        if not arg:
            return []
        res = []
        for i in arg:
            # 此处将rules参数全部去掉，无论game还是jvm。
            if "rules" in str(i).lower():
                continue
            # 在添加得时候记得将空格全部去掉。
            res.append(str(i).replace(" ", ""))
        return res

    @staticmethod
    def judge_mc_rules(root: dict) -> bool:
        """
        判断MC的规则，如果规则允许Window，则返回True，反之返回False
        :param root: 根json
        :return: 是否确认规则
        """
        rules = list(MainClass.get_nested_value(root, "rules", default_value=[]))
        for i in rules:
            action = str(MainClass.get_nested_value(i, "action", default_value=""))
            if action == "allow":
                osi = str(MainClass.get_nested_value(i, "os", "name", default_value="windows"))
                if osi != "windows":
                    return False
            elif action == "disallow":
                osi = str(MainClass.get_nested_value(i, "os", "name", default_value=""))
                if osi == "windows":
                    return False
        return True

    @staticmethod
    def get_mc_libs(real_json: str, root_path: str, version_path: str) -> str:
        """
        通过已替换的inheritsFrom的原始json，获取到cp后跟的libraries字符串。
        :param real_json: 已替换了inheritsFrom的json
        :param root_path: mc根路径
        :param version_path: mc版本路径
        :return: 已拼接的libraries，以${classpath_separator}作为分隔符。
        """
        raw_list = []
        no_list = []
        no_low = []
        temp_list = []
        no_opt = []
        try:
            root = json.loads(real_json)
        except JSONDecodeError:
            return ""
        json_lib = MainClass.get_nested_value(root, "libraries", default_value=[])
        for i in json_lib:
            name = MainClass.get_nested_value(i, "name", default_value="")
            if name == "": continue
            expect_rule = LaunchMethod.judge_mc_rules(i)
            expect_download = True
            e = MainClass.get_nested_value(i, "downloads", default_value={})
            fe = MainClass.get_nested_value(e, "classifiers", default_value={})
            if fe:
                expect_download = False
            fe = MainClass.get_nested_value(e, "artifact", default_value={})
            if fe:
                expect_download = True
            if expect_rule and expect_download: raw_list.append(name)
        for i in raw_list:
            if i not in no_list:
                no_list.append(i)
        for i in no_list:
            nocom = str(i) \
                        .replace(".", "") \
                        .replace(":", "") \
                        .replace("-", "") \
                        .replace("/", "") \
                        .replace("@jar", "") \
                        .replace("@zip", "") \
                        .replace("@", "")
            nonum = LaunchMethod.extract_number(nocom, False)
            noword = int(LaunchMethod.extract_number(nocom, True))
            if not nonum in temp_list:
                temp_list.append(nonum)
                no_low.append(i)
            else:
                try:
                    num = temp_list.index(nonum)
                except IndexError:
                    continue
                pos = no_low[num]
                nonum2 = int(LaunchMethod.extract_number(pos, True))
                if nonum2 < noword:
                    no_low.pop(num)
                    no_low.insert(num, i)
        temp_list.clear()
        for i in no_low:
            if "optifine" in i:
                temp_list.append(i)
            else:
                no_opt.append(i)
        no_opt.extend(temp_list)
        res = ""
        for i in no_opt:
            path = LaunchMethod.convert_name_to_path(i)
            if path != "":
                res += "{}\\libraries\\{}{}".format(root_path, path, "${classpath_separator}")
        inh = LaunchMethod.get_mc_inherits_from(version_path, "inheritsFrom")
        sha = MainClass.get_nested_value(root, "downloads", "client", "sha1", default_value="")
        if sha == "":
            return ""
        real = LaunchMethod.get_mc_real_path(inh, sha)
        if real == "":
            res = res[0:res.rfind("$")]
        else:
            res += real
        return res
    @staticmethod
    def replace_mc_inherits_from(raw_json: str, ins_json: str) -> str:
        """
        替换mc原版的json与forge专属附带了inheritsFrom的json。将二者合并成一个json。
        :param raw_json:  带有inheritsFrom的json
        :param ins_json:  原版json
        :return:  合并后的json
        """
        if raw_json == "" or ins_json == "":
            return ""
        raw_json = raw_json.replace("\\", "")
        ins_json = ins_json.replace("\\", "")
        if raw_json == ins_json:
            return raw_json
        try:
            raw_rt = dict(json.loads(raw_json))
        except JSONDecodeError:
            return ""
        try:
            ins_rt = dict(json.loads(ins_json))
        except JSONDecodeError:
            return ""
        mc = MainClass.get_nested_value(raw_rt, "mainClass", default_value="")
        if mc == "":
            return ""
        ins_rt["mainClass"] = mc
        idid = MainClass.get_nested_value(raw_rt, "id", default_value="")
        if idid == "":
            return ""
        ins_rt["id"] = idid
        raw_lib = list(MainClass.get_nested_value(raw_rt, "libraries", default_value=[]))
        for i in raw_lib:
            if "libraries" not in ins_rt:
                ins_rt["libraries"] = []
            ins_rt["libraries"].append(i)
        raw_jvm = list(MainClass.get_nested_value(raw_rt, "arguments", "jvm", default_value=[]))
        for i in raw_jvm:
            if "arguments" not in ins_rt:
                ins_rt["arguments"] = {}
            if "jvm" not in ins_rt["arguments"]:
                ins_rt["arguments"]["jvm"] = []
            ins_rt["arguments"]["jvm"].append(i)
        raw_game = list(MainClass.get_nested_value(raw_rt, "arguments", "game", default_value={}))
        for i in raw_game:
            if "arguments" not in ins_rt:
                ins_rt["arguments"] = {}
            if "game" not in ins_rt["arguments"]:
                ins_rt["arguments"]["game"] = []
            ins_rt["arguments"]["game"].append(i)
        raw_arg = str(MainClass.get_nested_value(raw_rt, "minecraftArguments", default_value=""))
        if raw_arg != "":
            ins_rt["minecraftArguments"] = raw_arg
        return str(ins_rt)
    @staticmethod
    def unzip_native(raw_json: str, root_path: str, version_path: str) -> bool:
        """
        解压json->libraries下的所有natives文件。如果没有任何一个natives，则仅新建文件夹。一般将其解压在【.minecraft/versions/<版本名称>/<版本名称>-<启动器名称>-native】下。
        :param raw_json:  已经被整合过的json
        :param root_path:  根路径
        :param version_path:  版本路径
        :return:  解压成功返回true，否则返回false
        """
        raw_list = []
        no_list = []
        no_low = []
        temp_list = []
        try:
            root = json.loads(raw_json)
        except JSONDecodeError:
            return False
        json_lib = MainClass.get_nested_value(root, "libraries", default_value=[])
        for i in json_lib:
            name = MainClass.get_nested_value(i, "name", default_value="")
            if name == "": continue
            expect_rule = LaunchMethod.judge_mc_rules(i)
            native = str(MainClass.get_nested_value(i, "natives", "windows", default_value=""))
            if native == "": continue
            name += ":" + native
            if expect_rule: raw_list.append(name)
        for i in raw_list:
            if i not in no_list:
                no_list.append(i)
        for i in no_list:
            nocom = str(i) \
                        .replace(".", "") \
                        .replace(":", "") \
                        .replace("-", "") \
                        .replace("/", "") \
                        .replace("@jar", "") \
                        .replace("@zip", "") \
                        .replace("@", "")
            nonum = LaunchMethod.extract_number(nocom, False)
            noword = int(LaunchMethod.extract_number(nocom, True))
            if not nonum in temp_list:
                temp_list.append(nonum)
                no_low.append(i)
            else:
                try:
                    num = temp_list.index(nonum)
                except IndexError:
                    continue
                pos = no_low[num]
                nonum2 = int(LaunchMethod.extract_number(pos, True))
                if nonum2 < noword:
                    no_low.pop(num)
                    no_low.insert(num, i)
        dir_path = "{}\\{}-{}-natives".format(version_path, os.path.basename(version_path), LAUNCHER_NAME)
        if not os.path.exists(dir_path) or os.path.exists(dir_path) and os.path.isfile(dir_path):
            os.makedirs(dir_path, exist_ok=True)
        MainClass.delete_dir_retain(dir_path, ".dll")
        if len(no_low) > 0:
            for c in no_low:
                cvn = LaunchMethod.convert_name_to_path(c)
                if cvn == "": continue
                rpath = "{}\\libraries\\{}".format(root_path, cvn)
                if not MainClass.unzip(rpath, dir_path):
                    continue
        return True
# 启动账号类
class LaunchAccount:
    # 由于Python不支持函数重载，因此该构造函数不能随意调用。
    # 最后一个online参数是个方便参数，0时是离线登录，1时是微软正版登录，2时是第三方外置登录。
    def __init__(self, name: str, uuid: str, access_token: str, atype: str, url: str, base: str, online: int):
        self.name = name
        self.uuid = uuid
        self.access_token = access_token
        self.atype = atype
        self.url = url
        self.base = base
        self.online = online
    @staticmethod
    # 由于目前暂时只用离线登录，因此这里只设置一个静态函数作为构造函数即可。
    # 后期这里会多出许多函数的！
    def new_offline(name: str, uuid: str):
        # 离线登录的accesstoken建议设置与uuid同值。然后账号类型是Legacy。
        return LaunchAccount(name, uuid, uuid, "Legacy", "", "", 0)
    def get_name(self) -> str:
        return self.name
    def get_uuid(self) -> str:
        return self.uuid
    def get_access_token(self) -> str:
        return self.access_token
    def get_atype(self) -> str:
        return self.atype
    def get_url(self) -> str:
        return self.url
    def get_base(self) -> str:
        return self.base
    def get_online(self) -> int:
        return self.online
# 启动设置类
class LaunchOption:
    """
    该设置类的构造函数有4个参数，分别是Account类、Java路径、mc根路径、mc版本路径、游戏路径
    其中必须确保Java路径精确到java.exe、根路径下要有assets、libraries文件夹，版本路径下要有版本json和主jar、游戏路径是存放游戏实例的，可以是空文件夹。
    除了这4个参数以外，别的参数都可以设置默认值。因此我们暂时只用这一个构造函数。
    """
    def __init__(self, account: LaunchAccount, java_path: str, root_path: str, version_path: str,
                 game_path: str):
        self.account = account
        self.java_path = java_path
        self.root_path = root_path
        self.version_path = version_path
        self.game_path = game_path
        self.window_height = 480
        self.window_width = 854
        self.min_memory = 256
        self.max_memory = 4096
        self.custom_info = "MMCLL"
        self.additional_jvm = ""
        self.additional_game = ""
    # 以下是设置该类的默认值的函数
    def set_window_height(self, window_height: int):
        self.window_height = window_height
    def set_window_width(self, window_width: int):
        self.window_width = window_width
    def set_min_memory(self, min_memory: int):
        self.min_memory = min_memory
    def set_max_memory(self, max_memory: int):
        self.max_memory = max_memory
    def set_custom_info(self, custom_info: str):
        self.custom_info = custom_info
    def set_additional_jvm(self, additional_jvm: str):
        self.additional_jvm = additional_jvm
    def set_additional_game(self, additional_game: str):
        self.additional_game = additional_game
    def get_account(self) -> LaunchAccount:
        return self.account
    def get_java_path(self) -> str:
        return self.java_path
    def get_root_path(self) -> str:
        return self.root_path
    def get_version_path(self) -> str:
        return self.version_path
    def get_game_path(self) -> str:
        return self.game_path
    def get_window_height(self) -> int:
        return self.window_height
    def get_window_width(self) -> int:
        return self.window_width
    def get_min_memory(self) -> int:
        return self.min_memory
    def get_max_memory(self) -> int:
        return self.max_memory
    def get_custom_info(self) -> str:
        return self.custom_info
    def get_additional_jvm(self) -> str:
        return self.additional_jvm
    def get_additional_game(self) -> str:
        return self.additional_game
class LaunchGame:
    """
    该类就是专门启动游戏的类啦！
    但是请尽量不要直接调用这个类。请使用下方有一个def函数进行调用。
    如果你真的很想自己实现一个启动过程的话，可以查看下面的语法规则进行调用！
    """
    def __init__(self, option: LaunchOption, callback: Callable[[list], None]):
        self.account = option.get_account()
        self.java_path = option.get_java_path()
        self.root_path = option.get_root_path()
        self.version_path = option.get_version_path()
        self.game_path = option.get_game_path()
        self.window_height = option.get_window_height()
        self.window_width = option.get_window_width()
        self.min_memory = option.get_min_memory()
        self.max_memory = option.get_max_memory()
        self.custom_info = option.get_custom_info()
        self.additional_jvm = option.get_additional_jvm()
        self.additional_game = option.get_additional_game()
        self.callback = callback
    # 以下三个函数是我们即将要实现的所有启动逻辑。
    def check_error(self) -> int:
        screen = screeninfo.get_monitors()[0]  # 获取主屏幕长宽
        memory = psutil.virtual_memory().total // (1024 ** 2)  # 获取内存总量
        if self.account.get_online() == 0:
            # 使用正则表达式判断UUID和用户名是否符合规范。
            if not re.match("^[0-9a-f]{32}$", self.account.get_uuid()):
                return ERR_LAUNCH_ACCOUNT_USERUUID
            if not re.match("^[0-9a-zA-Z]{3,16}$", self.account.get_name()):
                return ERR_LAUNCH_ACCOUNT_USERNAME
        #  TODO: 微软登录和外置登录
        if not os.path.exists(self.java_path):
            return ERR_LAUNCH_JAVA_PATH
        if not os.path.exists(self.root_path):
            return ERR_LAUNCH_ROOT_PATH
        if not os.path.exists(self.version_path):
            return ERR_LAUNCH_VERSION_PATH
        if not os.path.exists(self.game_path):
            return ERR_LAUNCH_GAME_PATH
        if self.window_width < 854 or self.window_width > screen.width:
            return ERR_LAUNCH_WINDOW_WIDTH
        if self.window_height < 480 or self.window_height > screen.height:
            return ERR_LAUNCH_WINDOW_HEIGHT
        if self.min_memory < 256 or self.min_memory > 1024:
            return ERR_LAUNCH_MIN_MEMORY
        if self.max_memory < 1024 or self.max_memory > memory:
            return ERR_LAUNCH_MAX_MEMORY
        if self.custom_info == "":
            return ERR_LAUNCH_CUSTOM_INFO
        return OK
    def put_arguments(self, real_json: str) -> list:
        # root = {}
        # 为了防止在获取JSON时出现错误，固使用一个try-except
        try:
            root = json.loads(real_json)
        except JSONDecodeError:
            return []
        # 首先判断该json中是否含有id键。
        mc_id = MainClass.get_nested_value(root, "id", default_value="")
        if mc_id == "":
            return []
        # 再判断是否有mainClass键。
        main_class = MainClass.get_nested_value(root, "mainClass", default_value="")
        if main_class == "":
            return []
        # 再判断是否有assetIndex键。
        asset_index = MainClass.get_nested_value(root, "assetIndex", "id", default_value="")
        if asset_index == "":
            return []
        # 设立一个返回值
        result = []
        # # 拼接一次Java路径。
        # result.append(self.java_path)
        # # 如果需要填写多个额外JVM参数，请尝试自己添加更多的append。
        # result.extend(["-XX:+UseG1GC", "-XX:-UseAdaptiveSizePolicy", "-XX:-OmitStackTraceInFastThrow", "-Dfml.ignoreInvalidMinecraftCertificates=true", "-Dfml.ignorePatchDiscrepancies=true", "-Dlog4j2.formatMsgNoLookups=true"])
        # 手动的self.additional_jvm可能是含有空格的一整串字符串，你需要自己手动的将其按照空格切开。
        if self.additional_jvm != "":
            # 请注意，这里仅是一个示例，因为list的extend函数接受的是另一个list，而不是一个字符串。下同。
            result.extend(self.additional_jvm.split(" "))
        # 此处首先获取一遍arguments->jvm参数下的默认jvm参数。然后额外jvm参数需要放在默认jvm参数上面。
        jvm = LaunchMethod.judge_arguments(root, "jvm")
        if not jvm:
            # 如果没有检测到，则置一些默认参数。此处适配于1.12.2
            result.extend(["-Djava.library.path=${natives_directory}", "-cp", "${classpath}"])
        else:
            result.extend(jvm)
        # TODO: 此处需要预留一些空间置第三方外置登录的部分JVM参数，这个我们第二章教到外置登录再说。
        # 以下是添加最大最小内存，紧接着就要添加Game参数了。
        result.extend([f"-Xmn{self.min_memory}m", f"-Xmx{self.max_memory}m"])
        # 下面拼接主类，然后就开始拼接游戏参数。
        result.append(main_class)
        # 如果找不到1.12.2以下的minecraftArguments参数，则尝试按照1.13版本以上的方式拼接。如果1.13以上也不行，则返回空列表。
        game = str(MainClass.get_nested_value(root, "minecraftArguments", default_value=""))
        if game != "":
            result.extend(game.split(" "))
            # 这里还要多判断一次arguments里的参数，以适配LiteLoader。
            game = LaunchMethod.judge_arguments(root, "game")
            if game:
                result.extend(game)
        else:
            game = LaunchMethod.judge_arguments(root, "game")
            if game:
                result.extend(game)
            else:
                return []
        # 如果额外game参数里不存在--fullScreen全屏参数，则拼接width和height。
        if not "--fullScreen" in self.additional_game:
            result.extend(["--width", str(self.window_width), "--height", str(self.window_height)])
        if self.additional_game != "":
            result.extend(self.additional_game.split(" "))
        # 解决optifine的一些破问题。。
        if "optifine.OptiFineForgeTweaker" in result:
            pos = result.index("optifine.OptiFineForgeTweaker")
            result.pop(pos - 1)
            result.pop(pos - 1)
            result.append("--tweakClass")
            result.append("optifine.OptiFineForgeTweaker")
        if "optifine.OptiFineTweaker" in result:
            pos = result.index("optifine.OptiFineTweaker")
            result.pop(pos - 1)
            result.pop(pos - 1)
            result.append("--tweakClass")
            result.append("optifine.OptiFineTweaker")
        # 替换列表里的替换模板值
        for i in range(len(result)):
            result[i] = result[i] \
                .replace("${natives_directory}", self.version_path + r"\{}-{}-natives".format(os.path.basename(self.version_path), LAUNCHER_NAME)) \
                .replace("${launcher_name}", LAUNCHER_NAME) \
                .replace("${launcher_version}", LAUNCHER_VERSION) \
                .replace("${classpath}", LaunchMethod.get_mc_libs(real_json, self.root_path, self.version_path)) \
                .replace("${library_directory}", self.root_path + r"\libraries") \
                .replace("${auth_player_name}", self.account.get_name()) \
                .replace("${auth_uuid}", self.account.get_uuid()) \
                .replace("${version_name}", mc_id) \
                .replace("${game_directory}", self.game_path) \
                .replace("${assets_root}", self.root_path + r"\assets") \
                .replace("${assets_index_name}", asset_index) \
                .replace("${auth_access_token}", self.account.get_access_token()) \
                .replace("${user_type}", self.account.get_atype()) \
                .replace("${version_type}", self.custom_info) \
                .replace("${user_properties}", "{}") \
                .replace("${uuid}", self.account.get_uuid()) \
                .replace("${game_assets}", self.root_path + r"\assets") \
                .replace("${classpath_separator}", ";")
        return result
    def game_launch(self):
        def_jvm = ["-XX:+UseG1GC", "-XX:-UseAdaptiveSizePolicy", "-XX:-OmitStackTraceInFastThrow", "-Dfml.ignoreInvalidMinecraftCertificates=true", "-Dfml.ignorePatchDiscrepancies=true", "-Dlog4j2.formatMsgNoLookups=true"]
        defn_jvm = ["-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump"]
        version_json_path = LaunchMethod.get_mc_real_path(self.version_path, ".json")
        if version_json_path == "":
            raise LaunchException("Cannot get version json path!")
        raw_json = MainClass.get_file(version_json_path)
        inherits_from_path = LaunchMethod.get_mc_inherits_from(self.version_path, "inheritsFrom")
        if inherits_from_path == "":
            raise LaunchException("Cannot get inheritsFrom json path!")
        if inherits_from_path == self.version_path:
            file = LaunchMethod.get_mc_real_path(inherits_from_path, ".json")
            final_json = MainClass.get_file(file)
        else:
            final_json = raw_json
        real_json = LaunchMethod.replace_mc_inherits_from(raw_json, final_json)
        if real_json == "":
            raise LaunchException("Cannot replace mc inherits from!")
        if not LaunchMethod.unzip_native(real_json, self.root_path, self.version_path):
            raise LaunchException("Cannot unzip native file")
        param = [self.java_path]
        param.extend(def_jvm)
        param.extend(defn_jvm)
        put = self.put_arguments(real_json)
        param.extend(put)
        self.callback(param)
def launch_game(option: LaunchOption, callback: Callable[[list], None]) -> int:
    """
    该函数可以直接调用，接受一个LaunchOption和一个闭包作为参数。
    如果在检测参数没有问题后，则会尝试拼接启动参数。
    如果参数拼接没有问题，但是启动过程中出现了问题，则会直接raise一个错误！
    所以你需要使用try-except包围住这个函数以供调试。
    如果检测参数没有错误，并且也能成功拼接启动参数，则会执行该闭包函数。并将拼接好的启动参数以list的形式作为参数。无返回值。
    :param option:  基本启动设置
    :param callback:  返回命令的lambda表达式
    """
    res = LaunchGame(option, callback)
    code = res.check_error()
    if code == OK:
        res.game_launch()
    return code

# import subprocess
# def gen(aa: list):
#     for i in aa:
#         print(i)
#     subprocess.run(aa)

# if __name__ == '__main__':
#     a = LaunchAccount.new_offline("SSSteve", "1234567890abcdef1234567890abcdef")
#     b = LaunchOption(a, r"D:\Languages\Java\jdk-21.0.4\bin\java.exe", r"D:\mc\testmc\.minecraft", r"D:\mc\testmc\.minecraft\versions\1.21.1-Forge_52.0.3", r"D:\mc\testmc\.minecraft\versions\1.21.1-Forge_52.0.3")
#     c = launch_game(b, gen)
#     print(c)
