# Use Thunk to build your Rust program to support old Windows platforms

[中文自述文件](./readme-chinese.md)

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build programs that support even Windows XP. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY-Thunks to remedy API that old platform that does not exist

Note: Thunk does not guarantee the compiled program work or work accurately on old platforms. **USE AT YOUR OWN RISK**!

## Usage (As Command line tool)

## Preparation

Download VC-LTL5 and YY-Thunks Binary, unzip them and add environment variable:

| Binary | Environment Variable |
| --- | ---|
| VC-LTL-XXX-Binary.7z | VC_LTL |
| YY-Thunks-XXX-Binary.zip | YY_THUNKS |

Then add Thunk to run path. 


## Install Thunk

```
cargo install thunk-cli
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

Note: In order to distinguish the program build by Thunk, Thunk builds the release in `./target/*_build`.

# Usage (As Library)

Step1: Ensure command line tools `curl` and `7z` could be found in `PATH`. (Needed if `VC_LTL` and `YY_THUNKS` not found in environment variables)

Step2: Add thunk as a build dependency:

```
cargo add thunk-rs --build
```

Step3: Create a build script `build.rs`:

```
fn main() {
    thunk::thunk();
}
```

Then, your program should run on Windows XP. See [thunk-rs](./thunk-rs/README.md).

# Todo list

 - [x] Windows XP x86
 - [x] Windows XP x64
 - [x] Windows Vista x86
 - [x] Windows Vista x64
 - [x] Windows 7 x86 (v0.3.2)
 - [x] Windows 7 x64 (v0.3.2)
 - [x] Windows 8 x86 (v0.3.2)
 - [x] Windows 8 x64 (v0.3.2)
 - [x] Windows 10 x86 (v0.3.2)
 - [x] Windows 10 x64 (v0.3.2)
 - [x] Only VC-LTL
 - [ ] Scoop bucket


# Thanks
 
 - [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5)
 - [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks)
