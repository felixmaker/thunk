#![doc = include_str!("../README.md")]

use std::{env, path::PathBuf, process::Command};

const VC_LTL_VERSION: &'static str = "5.2.2-Beta2";
const YY_THUNKS_VERSION: &'static str = "1.1.7";

/// This function should be call in build.rs.
pub fn thunk() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if target_os != "windows" || target_env != "msvc" {
        println!("cargo::warning=Skipped! Only Windows(MSVC) is supported!");
        return;
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Enable VC-LTL5
    let vc_ltl_arch = if target_arch == "x86" { "Win32" } else { "x64" };
    let vc_ltl_platform = if cfg!(feature = "xp") {
        if vc_ltl_arch == "Win32" {
            "5.1.2600.0"
        } else {
            "5.2.3790.0"
        }
    } else if cfg!(feature = "vista") || cfg!(feature = "win7") {
        "6.0.6000.0"
    } else if cfg!(feature = "win8") {
        "6.2.9200.0"
    } else if cfg!(feature = "win10_10240") {
        "10.0.10240.0"
    } else if cfg!(feature = "win10_19041") {
        "10.0.19041.0"
    } else if cfg!(feature = "vc_ltl_only") {
        "6.0.6000.0"
    } else {
        println!("cargo::warning=VC-LTL5 Skipped: Nothing to do!");
        return;
    };

    let vc_ltl = get_or_download(
        "VC_LTL",
        "VC_LTL_URL",
        &format!(
            "https://github.com/Chuyu-Team/VC-LTL5/releases/download/v{}/VC-LTL-Binary.7z",
            VC_LTL_VERSION
        ),
        &out_dir,
        &format!("VC-LTL-{}", VC_LTL_VERSION),
    );

    let vc_ltl_path = vc_ltl.join(&format!(
        "TargetPlatform/{}/lib/{}",
        vc_ltl_platform, vc_ltl_arch
    ));

    println!("cargo::rustc-link-search={}", vc_ltl_path.to_string_lossy());
    println!(
        "cargo::warning=VC-LTL5 Enabled: {}({})",
        vc_ltl_platform, vc_ltl_arch
    );

    // Enable YY-Thunks
    let yy_thunks_arch = if target_arch == "x86" { "x86" } else { "x64" };
    let yy_thunks_platform = if cfg!(feature = "xp") {
        "WinXP"
    } else if cfg!(feature = "vista") {
        "Vista"
    } else if cfg!(feature = "win7") {
        "Win7"
    } else if cfg!(feature = "win8") {
        "Win8"
    } else if cfg!(feature = "win10_10240") {
        "Win10.0.10240"
    } else if cfg!(feature = "win10_19041") {
        "Win10.0.19041"
    } else {
        println!("cargo::warning=YY-Thunks Skipped: Nothing to do!!");
        return;
    };

    let yy_thunks = get_or_download(
        "YY_THUNKS",
        "YY_THUNKS_URL",
        &format!(
            "https://github.com/Chuyu-Team/YY-Thunks/releases/download/v{}/YY-Thunks-Objs.zip",
            YY_THUNKS_VERSION
        ),
        &out_dir,
        &format!("YY-Thunks-{}", YY_THUNKS_VERSION),
    );

    let yy_thunks = yy_thunks.join(format!(
        "objs/{}/YY_Thunks_for_{}.obj",
        yy_thunks_arch, yy_thunks_platform
    ));

    println!("cargo::rustc-link-arg={}", yy_thunks.to_string_lossy());
    println!(
        "cargo::warning=YY-Thunks Enabled: {}({})",
        yy_thunks_platform, yy_thunks_arch
    );

    // Return if is lib mode
    if cfg!(feature = "lib") {
        println!("cargo::warning=Lib Mode Enabled!");
        return;
    }

    // Set subsystem to windows
    let os_version = if cfg!(feature = "xp") {
        if target_arch == "x86" {
            ",5.01"
        } else {
            ",5.02"
        }
    } else {
        ""
    };

    if cfg!(feature = "subsystem_windows") && env::var("PROFILE").unwrap() != "debug" {
        println!("cargo::rustc-link-arg=/SUBSYSTEM:WINDOWS{}", os_version);
        println!("cargo::rustc-link-arg=/ENTRY:mainCRTStartup");
        println!("cargo::warning=Subsystem is set to WINDOWS");
    } else if cfg!(feature = "subsystem_console"){
        println!("cargo::rustc-link-arg=/SUBSYSTEM:CONSOLE{}", os_version);
        println!("cargo::warning=Subsystem is set to CONSOLE");
    }
}

fn get_or_download(
    env_path: &str,
    env_url: &str,
    default_url: &str,
    out_dir: &PathBuf,
    unpack_name: &str,
) -> PathBuf {
    if let Ok(env_path) = env::var(env_path) {
        PathBuf::from(env_path)
    } else {
        let unpack_dir = out_dir.join(unpack_name);

        // Skip download if unpack dir exists.
        if unpack_dir.exists() {
            return unpack_dir;
        }

        let url = if let Ok(env_url) = env::var(env_url) {
            PathBuf::from(env_url)
        } else {
            PathBuf::from(default_url)
        };

        let curl_status = Command::new("curl")
            .args(["-LOkf", url.to_str().unwrap()])
            .current_dir(out_dir)
            .status()
            .expect("Curl is needed to download binaries!");

        if !curl_status.success() {
            panic!("Download libraries from {:?} failed", url)
        }

        let extract_status = Command::new("7z")
            .args([
                "x",
                "-aoa",
                url.file_name().unwrap().to_str().unwrap(),
                &format!("-o{}", unpack_name),
            ])
            .current_dir(out_dir)
            .status()
            .expect("7z is needed to unpack libraries!");

        if !extract_status.success() {
            panic!("Unpack YY-Thunks failed!")
        }

        unpack_dir
    }
}
