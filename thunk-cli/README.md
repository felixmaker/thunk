# Thunk-cli

Thunk the Rust program to support old Windows platforms!

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
