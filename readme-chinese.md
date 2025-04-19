# 使用 Thunk 帮助你编译能在 Windows XP 上运行的 Rust 程序

Thunk 主要帮你做了下面两件事：

 - 将 [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) 添加到库搜索路径中
 - 额外链接 [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)，以弥补 Vista 和 XP 上没有的 API

注意：Thunk **并不能保证所编译的软件可以成功编译或者编译后正常运行**。

# 作为命令行工具使用

## 准备工作

下载 VC-LTL5、 YY-Thunks Binary 文件，解压，并添加环境变量：

| Binary | 环境变量 |
| --- | ---|
| VC-LTL-5.0.8-Beta2-Binary.7z | VC_LTL |
| YY-Thunks-1.0.8-Beta4-Binary.zip | YY_THUNKS |

## 安装

```
cargo install thunk
```

## 示例 1 编译一个可以在 XP 上运行的程序

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 -- --release
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
| -- | -- 后面的参数会传到 cargo build 后面 | 自定义 |

注：为了区分 Thunk 编译出来的程序，程序生成在 `./target/*_build` 文件夹。

# 作为类库使用

步骤1：确保 `curl` 和 `7z` 工具放置在运行目录里（如果设置了 `VC_LTL` 和 `YY_THUNKS` 环境变量可省去此步）

步骤2：添加 `thunk-rs` 依赖：

```
cargo add thunk-rs --build
```

步骤3：添加生成脚本 `build.rs`：

```
fn main() {
    thunk::thunk();
}
```

不出意外，编程出来程序可以在 XP 上运行，查看 [thunk-rs](./thunk-rs/README.md).


# 任务清单

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Only VC-LTL
 - [x] Scoop bucket


# 致谢
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
