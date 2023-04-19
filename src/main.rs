use std::{collections::HashMap, path::PathBuf, process::Command};

fn main() {
    let env_vars: HashMap<String, String> = std::env::vars().collect();

    let vc_ltl = env_vars
        .get("VC_LTL")
        .expect("You need to set VC_LTL environment variable.");
    let yy_thunks = env_vars
        .get("YY_THUNKS")
        .expect("You need to set VC_LTL environment variable.");

    let vc_ltl = PathBuf::from(vc_ltl);
    let yy_thunks = PathBuf::from(yy_thunks);

    let xp_lib = PathBuf::from("TargetPlatform\\5.1.2600.0\\lib\\Win32");
    let xp_thunks = PathBuf::from("objs\\x86\\YY_Thunks_for_WinXP.obj");

    let vcltl_lib = vc_ltl.join(xp_lib);
    let thunk_obj = yy_thunks.join(xp_thunks);

    let rustflags = [
        "-L",
        vcltl_lib.to_str().unwrap(),
        "-Clink-args=/subsystem:console,5.01",
        "-Clink-args=oldnames.lib",
        &format!("-Clink-args={}", thunk_obj.to_str().unwrap()),
    ]
    .join(" ");

    let cargo_args = [
        "build",
        "--release",
        "--target",
        "i686-pc-windows-msvc",
        "--target-dir",
        "./target/xp_build",
    ];

    println!("Start to build for Windows XP using VC-LTL and YY-Thunks: ");
    println!(" * VC_LTL = {}", vc_ltl.to_str().unwrap());
    println!(" * YY_THUNKS = {}", yy_thunks.to_str().unwrap());
    println!(" * RUSTFLAGS = {}", rustflags);
    println!(" * Command = cargo {}", cargo_args.join(" "));
    println!("Cargo Output:");

    let _status = Command::new("cargo")
        .env("RUSTFLAGS", rustflags)
        .args(cargo_args)
        .status()
        .expect("failed to execute cargo");

    println!("The release is build in .\\target\\xp_build\\i686-pc-windows-msvc\\release");
}
