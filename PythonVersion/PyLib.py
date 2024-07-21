#!/usr/bin/python
# -*- coding: UTF-8 -*-
import os
import sys


def private_method():
    """
    私有函数注解
    """

    def decorate(func):
        def wrap(*args, **kwargs):
            if os.path.basename(sys._getframe(1).f_code.co_filename) != 'PyLib.py':
                raise Exception("This is a Private Function!")
            return func(*args, **kwargs)

        return wrap

    return decorate


class LauncherLoginType:
    """
    构造函数
    设置玩家的登录
    :param player_name: 填入玩家名称【从不可空】
    :param uuid: 填入玩家UUID【从不可空】
    :param access_token: 填入玩家登录密钥【离线可空】【微软不可空】【外置不可空】
    :param server: 填入玩家使用外置时的登录服务器【离线可空】【微软可空】【外置不可空】
    :param base_code: 填入玩家使用外置登录时的认证服务器元数据Base64编码【离线可空】【微软可空】【外置不可空】
    :type player_name: str
    :type uuid: str
    :type access_token: str
    :type server: str
    :type base_code: str
    """

    def __init__(self, player_name, uuid, access_token='', server='', base_code=''):
        self.__player_name = player_name
        self.__uuid = uuid
        self.__access_token = access_token
        self.__server = server
        self.__base_code = base_code

    @private_method()
    def get_player_name(self):
        """
        私有函数
        获取玩家的名称
        """
        return self.__player_name

    @private_method()
    def get_uuid(self):
        """
        私有函数
        获取玩家的UUID
        """
        return self.__uuid

    @private_method()
    def get_access_token(self):
        """
        私有函数
        获取玩家的登录密钥
        """
        return self.__access_token

    @private_method()
    def get_server(self):
        """
        私有函数
        获取玩家的认证服务器
        """
        return self.__server

    @private_method()
    def get_base_code(self):
        """
        私有函数
        获取玩家的认证服务器元数据Base64编码
        """
        return self.__base_code


class LauncherMain:
    def __init__(self):
        pass

    @staticmethod
    def create():
        a = LauncherLoginType('a', 'c', 'v')
        print(a.get_uuid())


class LauncherOrigins:
    @staticmethod
    def get_version_list(mc_path):
        """
        公有函数
        可以获取mc根目录下的versions文件夹下的所有文件夹名称。
        :param mc_path: 填入Minecraft根目录路径。
        :type mc_path: str
        :return: 返回versions文件夹下所有目录的绝对路径。
        :rtype: list
        :raise FileNotFoundError: 如果路径找不到可能会抛出。
        """
        mc_path = os.path.join(mc_path, 'versions')
        li = []
        for ii in os.listdir(mc_path):
            li.append(os.path.join(mc_path, ii))
        return li

    @staticmethod
    def get_file(path):
        """
        公有函数
        获取外部文件的内容的函数。
        :param path: 填入外部文件路径
        :rtype path: str
        :return: 返回文件内容
        :rtype: str
        :raise FileNotFoundError: 如果路径找不到可能会抛出。
        """
        if not os.path.exists(path):
            raise FileNotFoundError()
        a = open(path, encoding='utf-8')
        f = a.read()
        a.close()
        return f