# rpp

（施工当中.......）

rpp 是一个简单的命令行解析器，其名字时 Rust Parameter Parser 的缩写。主要用于为 shell 脚本提供命令行参数解析功能，但作为第三方库为其他 Rust 程序提供简易命令行参数解析功能。

## 前置说明

![命令行参数](doc/resource/parameter.png)

## 概述

![概述](doc/resource/rpp.png)

## 参数判断算法

- 开头不是 -
  - 进入参数（Argument）列表
- 开头是 -
  - 是一个等式
    - 进入选项（Option）列表
  - 不是等式则判断下一个元素
    - 下一个元素不以 - 开头，当前元素是选项(Option)，合并后一个参数进入选项（Option）列表
    - 下一个参数以 - 开头，当前元素是开关（Flag）,进入开关列表（Flags）
