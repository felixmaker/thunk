# thunk: Thunk the Rust program to support old Windows platforms!

## How does it work?

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build program that supports even Windows XP. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY-Thunks to remedy API that old platform that does not exist

Note: Thunk does not guarantee the compiled program work or work accurately on old platforms. 
**USE AT YOUR OWN RISK!**

## Usage

Step1: Ensure command line tools `curl` and `7z` could be found in `PATH`. (Needed if `VC_LTL` and `YY_THUNKS` not found in environment variables)

Step2: Add thunk as a build dependency:

```
cargo add thunk-rs --build
```

Step3: Create a build script build.rs:

```
fn main() {
    thunk::thunk();
}
```

Then, your program should run on Windows XP.

## Feature

 - windows_xp: Enables VC-LTL5 and YY-Thunks to support Windows XP (default)
 - windows_vista: Enables VC-LTL5 and YY-Thunks to support Windows Vista
 - vc_ltl_only: Enables VC-LTL5 to make the final executable run without VC runtime installed.
 - lib: Enables this when compiling a library.
 - subsystem_windows: Enables this when you want to hide console.
