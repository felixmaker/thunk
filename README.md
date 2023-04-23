# Use Thunk to build your Rust program that runs on old platforms.

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build programs that support old platforms. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY_Thunks to remedy API that old platform that does not exist

Note: Thunk do not guarantee the compiled program work or work accurately on old platform. USE AT YOUR OWN RISK!

In order to distinguish the program build by thunks, thunk build the release in it `./target/*_build`.

# How to use?

1. Download VC-LTL, maybe `VC-LTL-5.0.6-Beta5-Binary.7z` from VC-LTL release. Unzip it to a folder. Make sure the folder contains TargetPlatform folder. Then set environment variable VC_LTL = this folder.
2. Download YY-Thunks, maybe `YY-Thunks-1.0.7-Beta4-Binary.zip` from YY-Thunks release. Unzip it to a folder. Make sure the folder contains objs folder. Then set environment variable YY_Thunks = this folder.
3. Maybe add thunk to Path.

Then, it's the time to use thunk.

```
cargo new build_for_xp
cd build_for_xp
thunk --os xp --arch x86 --release
```

After that, you may find a release in `./target/winxp_build`

# Usage
```
Use Thunk to build your Rust program that runs on old platforms

Usage: thunk.exe [OPTIONS]

Options:
  -v, --vc-ltl-path <VC_LTL_PATH>
          VC-LTL folder, you may get it from https://github.com/Chuyu-Team/VC-LTL5/releases
      --enable-thunks
          Enable YY-Thunks to support Windows Vista or XP
  -y, --yy-thunks-path <YY_THUNKS_PATH>
          YY-Thunks folder, you may get it from https://github.com/Chuyu-Team/YY-Thunks/releases
  -o, --os <OS>
          Operating system names: xp, vista, win7, win10, 20h1
  -a, --arch <ARCH>
          Operating system arch: x86, x64, arm64
  -s, --subsystem <SUBSYSTEM>
          Link arg: console, windows
      --release
          Release build
  -h, --help
          Print help
```

# Todo list

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Only VC-LTL

Note: Currently, thunk only supports Windows XP x86. It does not have any arguments.

# Thanks
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
