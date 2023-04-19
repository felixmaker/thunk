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
thunk
```

After that, you may find a release in `./target/xp_build`

# Todo list

 - [x] Windows XP x86
 - [] Windows XP x64
 - [] Windows Vista x86
 - [] Windows Vista x64
 - [] Only VC-LTL

# Thanks
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
