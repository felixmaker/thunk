#![doc = include_str!("../README.md")]

use std::{env, path::PathBuf, thread};

#[cfg(feature = "native-download")]
mod native;

#[cfg(all(feature = "external-download", not(feature = "native-download")))]
mod external;

#[cfg(feature = "native-download")]
use native::get_or_download;

#[cfg(all(feature = "external-download", not(feature = "native-download")))]
use external::get_or_download;

#[cfg(not(any(feature = "native-download", feature = "external-download")))]
#[allow(unreachable_code)]
fn get_or_download(
    _env_path: &str,
    _env_url: &str,
    _default_url: &str,
    _out_dir: &std::path::Path,
    _unpack_name: &str,
    _compressed_type: CompressedType,
) -> PathBuf {
    panic!(
        "No extraction feature enabled! Enable either 'native-download' or 'external-download'."
    );
}

const VC_LTL_DOWNLOAD_VERSION_DEFAULT: &str = "5.3.1";
const YY_THUNKS_DOWNLOAD_VERSION_DEFAULT: &str = "1.1.9";

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
    let vc_ltl_platform = if cfg!(feature = "xp") || cfg!(feature = "win2k") {
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

    thread::scope(|s| {
        s.spawn(|| {
            let vcltl_download_version = if let Ok(version) = env::var("VC_LTL_DOWNLOAD_VERSION") {
                version
            } else {
                VC_LTL_DOWNLOAD_VERSION_DEFAULT.to_string()
            };

            let vc_ltl = get_or_download(
                "VC_LTL",
                "VC_LTL_URL",
                &format!(
                    "https://github.com/Chuyu-Team/VC-LTL5/releases/download/v{}/VC-LTL-Binary.7z",
                    vcltl_download_version
                ),
                &out_dir,
                &format!("VC-LTL-{}", vcltl_download_version),
                CompressedType::SevenZip,
            );

            let vc_ltl_path = vc_ltl.join(format!(
                "TargetPlatform/{}/lib/{}",
                vc_ltl_platform, vc_ltl_arch
            ));

            println!("cargo::rustc-link-search={}", vc_ltl_path.to_string_lossy());
            println!(
                "cargo::warning=VC-LTL5 Enabled: {}({})",
                vc_ltl_platform, vc_ltl_arch
            );
        });

        s.spawn(|| {
            if cfg!(feature = "vc_ltl_only") {
                println!("cargo::warning=YY-Thunks Skipped: Nothing to do!!");
                return;
            }

            // Enable YY-Thunks
            let yy_thunks_arch = if target_arch == "x86" { "x86" } else { "x64" };
            let yy_thunks_platform = if cfg!(feature = "win2k") && target_arch == "x86" {
                "Win2K"
            } else if cfg!(feature = "xp") {
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

            let yy_thunks_download_version =
                if let Ok(version) = env::var("YY_THUNKS_DOWNLOAD_VERSION") {
                    version
                } else {
                    YY_THUNKS_DOWNLOAD_VERSION_DEFAULT.to_string()
                };
            let yy_thunks = get_or_download(
                "YY_THUNKS",
                "YY_THUNKS_URL",
                &format!(
                    "https://github.com/Chuyu-Team/YY-Thunks/releases/download/v{}/YY-Thunks-Objs.zip",
                    yy_thunks_download_version
                ),
                &out_dir,
                &format!("YY-Thunks-{}", yy_thunks_download_version),
                CompressedType::Zip,
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
            } else {
                println!("cargo::rustc-link-arg=/SUBSYSTEM:CONSOLE{}", os_version);
            }
        });
    });
}

pub(crate) enum CompressedType {
    SevenZip,
    Zip,
}
