# Use Thunk to build your Rust program that runs on old platforms.

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build programs that support old platforms. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY-Thunks to remedy API that old platform that does not exist

Note: Thunk do not guarantee the compiled program work or work accurately on old platform. USE AT YOUR OWN RISK!

In order to distinguish the program build by Thunk, Thunk builds the release in `./target/*_build`.

有没有可能让 Rust 程序在 Windows XP 上运行？有的，只需要你的程序链接到 [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) 和 [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)。

Thunk 主要做了两件事：

 - 将 VC-LTL5 添加到包含库中，以便 Cargo 链接
 - 链接 YY-Thunks，以弥补 Vista 和 XP 上没有的 API

注意：Thunk 不能保证所编译的软件可以成功编译或者编译后正常运行，所以**使用风险自负**。为了区分 Thunk 编译出来的程序，程序生成在 `./target/*_build` 文件夹。

# How to use?

## Preparation

Download VC-LTL5, YY-Thunks Binary, unzip them and add environment variable:

下载 VC-LTL5、 YY-Thunks Binary 文件，解压，并添加环境变量：

| Binary | Environment Variable |
| --- | ---|
| VC-LTL-5.0.6-Beta5-Binary.7z | VC_LTL |
| YY-Thunks-1.0.7-Beta4-Binary.zip | YY_Thunks |

Then add Thunk to run path. Or you can just install with scoop (todo!):

将 Thunk 添加到环境变量。通过 Scoop 包管理器可以一步到位安装（未完成！）：

```
scoop bucket add felixmaker 'https://github.com/felixmaker/scoop-felixmaker'
scoop install felixmaker/thunk
```

## Sample 1. Build for Windows XP

示例 1 编译一个可以在 XP 上跑的程序：

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 --release
```

Then, you may find release in `./target/winxp_build`

编译结果在 `./target/winxp_build` 文件夹

# Usage

Use the following command to show help:

使用下面的指令查看更多帮助：

```
thunk.exe --help
```


# Todo list

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Only VC-LTL


# Thanks
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
