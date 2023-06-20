# 使用 Thunk 帮助你编译能在 Windows XP 上运行的 Rust 程序

Thunk 主要帮你做了下面两件事：

 - 将 [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) 添加到库搜索路径中
 - 额外链接 [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)，以弥补 Vista 和 XP 上没有的 API

注意：Thunk **并不能保证所编译的软件可以成功编译或者编译后正常运行**。

为了区分 Thunk 编译出来的程序，程序生成在 `./target/*_build` 文件夹。


# 使用方法

## 准备工作（手动）

下载 VC-LTL5、 YY-Thunks Binary 文件，解压，并添加环境变量：

| Binary | 环境变量 |
| --- | ---|
| VC-LTL-5.0.8-Beta2-Binary.7z | VC_LTL |
| YY-Thunks-1.0.8-Beta4-Binary.zip | YY_Thunks |

再将 Thunk 添加到环境变量。

## 准备工作（Scoop）

你也可以直接通过 Scoop 包管理器安装：

```
scoop bucket add felixmaker 'https://github.com/felixmaker/scoop-felixmaker'
scoop install felixmaker/thunk
```

国内加速下载：

```
scoop bucket add sfm-cn 'https://github.com/felixmaker/sfm-cn'
scoop install sfm-cn/thunk
```

## 示例 1 编译一个可以在 XP 上运行的程序

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 --release
```

## 示例 2 编译一个可以在 XP 上使用的动态链接库

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 --lib -- --release
```


## 更多用法请查看帮助

```
thunk.exe --help
```


| 参数 | 说明 | 可能值 |
| --- | --- | --- |
| --os | 系统名称 | xp, vista, win7, win10, 20h1 |
| --arch | 系统架构 | x86, x64, arm64 |
| --lib | 是否为共享库，指定时 subsystem 将被忽略 | - |
| --subsystem | 设置 subsystem | console, windows |
| -- | 传到 cargo build 后面 | 自定义 |



# 任务清单

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Only VC-LTL
 - [ ] Scoop bucket


# 致谢
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
