# Use Thunk to build your Rust program that runs on old Windows platforms

[中文自述文件](./readme-chinese.md)

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build programs that support even Windows XP. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY-Thunks to remedy API that old platform that does not exist

Note: Thunk do not guarantee the compiled program work or work accurately on old platform. USE AT YOUR OWN RISK!

In order to distinguish the program build by Thunk, Thunk builds the release in `./target/*_build`.

# How to use?

## Preparation (Manual)

Download VC-LTL5 and YY-Thunks Binary, unzip them and add environment variable:

| Binary | Environment Variable |
| --- | ---|
| VC-LTL-5.0.8-Beta2-Binary.7z | VC_LTL |
| YY-Thunks-1.0.8-Beta4-Binary.zip | YY_THUNKS |

Then add Thunk to run path. 

## Preparation (Scoop)

You can just install with scoop:

```
scoop bucket add felixmaker 'https://github.com/felixmaker/scoop-felixmaker'
scoop install felixmaker/thunk
```

## Sample 1. Build for Windows XP

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 -- --release
```

## Sample 2. Build a shared library for Windows XP

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 --lib -- --release
```

## Show help

Use the following command to show help:

```
thunk.exe --help
```


# Todo list

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Only VC-LTL
 - [x] Scoop bucket


# Thanks
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
