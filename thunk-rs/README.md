# thunk: Thunk the Rust program to support old Windows platforms!

Thunk uses [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) and [YY-Thunks](https://github.com/Chuyu-Team/YY-Thunks) to build programs that support even Windows XP. So, how does it work?

 - Add VC-LTL to the library search path
 - Use YY-Thunks to remedy API that old platform that does not exist

Note: Thunk do not guarantee the compiled program work or work accurately on old platform. USE AT YOUR OWN RISK!

# Usage

## Preparation (Manual)

Download VC-LTL5 and YY-Thunks Binary, unzip them and add environment variable:

| Binary | Environment Variable |
| --- | ---|
| VC-LTL-XXX-Binary.7z | VC_LTL |
| YY-Thunks-XXX.zip | YY_THUNKS |

## Step

Step 1. Add thunk-rs into build dependencies:

```
cargo add thunk-rs --build
```

Step 2. Create a build script `build.rs`:

```
use thunk::{ThunkBuilder, OS};

fn main() {
    if let Ok(thunk) = ThunkBuilder::default().with_os(OS::WindowsXP).build()
    {
        thunk.thunk();
    }
}
```

Then, the executable should run on Windows XP!
