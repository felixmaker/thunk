use anyhow::anyhow;
use clap::Parser;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

/// Use Thunk to build your Rust program that runs on old platforms.
#[derive(Debug, Parser)]
pub struct ThunkBuilder {
    /// VC-LTL folder, you may get it from https://github.com/Chuyu-Team/VC-LTL5/releases
    #[arg(short, long)]
    vc_ltl_path: Option<PathBuf>,
    /// Enable YY-Thunks to support Windows Vista or XP, enabled default.
    #[arg(long)]
    enable_thunks: bool,
    /// YY-Thunks folder, you may get it from https://github.com/Chuyu-Team/YY-Thunks/releases
    #[arg(short, long)]
    yy_thunks_path: Option<PathBuf>,
    /// Operating system names: xp, vista, win7, win10, 20h1
    #[arg(short, long)]
    os: Option<OS>,
    /// Operating system arch: x86, x64, arm64
    #[arg(short, long)]
    arch: Option<Arch>,
    /// Link arg: console, windows
    #[arg(short, long)]
    subsystem: Option<Subsystem>,
    /// Release build
    #[arg(long)]
    release: bool,
}

impl ThunkBuilder {
    pub fn new() -> Self {
        Self {
            vc_ltl_path: None,
            enable_thunks: false,
            yy_thunks_path: None,
            os: None,
            arch: None,
            subsystem: None,
            release: true,
        }
    }

    pub fn vc_ltl<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.vc_ltl_path = Some(path.as_ref().to_owned());
        self
    }

    pub fn enable_yy_thunks(mut self, enable: bool) -> Self {
        self.enable_thunks = enable;
        self
    }

    pub fn yy_thunks<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.yy_thunks_path = Some(path.as_ref().to_owned());
        self
    }

    pub fn os(mut self, os: &str) -> Self {
        self.os = Some(OS::from_str(os));
        self
    }

    pub fn arch(mut self, arch: &str) -> Self {
        self.arch = Some(Arch::from_str(arch));
        self
    }

    pub fn subsystem(mut self, subsystem: &str) -> Self {
        self.subsystem = Some(Subsystem::from_str(subsystem));
        self
    }

    pub fn release(mut self, enable: bool) -> Self {
        self.release = enable;
        self
    }

    pub fn build(self) -> anyhow::Result<Thunk> {
        let env_vars: HashMap<String, String> = std::env::vars().collect();

        let vc_ltl = match self.vc_ltl_path {
            Some(path) => path,
            None => {
                let vc_ltl_env_path = env_vars
                    .get("VC_LTL")
                    .ok_or_else(|| anyhow!("You need to set VC_LTL environment variable."))?;
                PathBuf::from(vc_ltl_env_path)
            }
        };

        let os = self.os.unwrap_or(OS::WindowsVista);
        let arch = self.arch.unwrap_or(Arch::Win32);

        let os_lib =
            get_vc_ltl_os_lib_path(os, arch).ok_or_else(|| anyhow!("os or arch is wrong"))?;

        let vcltl_lib = vc_ltl.join(os_lib);
        let vcltl_lib = vcltl_lib
            .to_str()
            .ok_or_else(|| anyhow!("VC_LTL path is not a UTF-8 valid path"))?;

        let subsystem = self.subsystem.unwrap_or(Subsystem::Console);
        let os_version =
            get_os_version(os, arch).ok_or_else(|| anyhow!("failed to get os version"))?;
        let subsystem_args = format!(
            "-Clink-args=/SUBSYSTEM:{},{}",
            subsystem.to_string(),
            os_version
        );

        let mut rust_flags = vec!["-L".to_string(), vcltl_lib.to_string(), subsystem_args];
        let enable_thunks = self.enable_thunks;

        if enable_thunks {
            let thunks_path = match self.yy_thunks_path {
                Some(path) => path,
                None => {
                    let yy_thunks_env_path = env_vars.get("YY_THUNKS").ok_or_else(|| {
                        anyhow!("You need to set YY_THUNKS environment variable.")
                    })?;
                    PathBuf::from(yy_thunks_env_path)
                }
            };

            let os_obj = get_yy_thunks_obj_path(os, arch).ok_or_else(|| anyhow!(""))?;
            let thunk_obj = thunks_path.join(os_obj);
            let thunk_obj = thunk_obj
                .to_str()
                .ok_or_else(|| anyhow!("YY_THUNKS path is not a UTF-8 valid path"))?;

            rust_flags.push("-Clink-args=oldnames.lib".to_string());
            rust_flags.push(format!("-Clink-args={}", thunk_obj));
        }

        let target = arch
            .to_rust_target()
            .ok_or_else(|| anyhow!("failed to support arm arch"))?;

        let target_dir = format!("./target/win{}_build", os.to_string().to_ascii_lowercase());

        let mut cargo_args = vec![
            "build".to_owned(),
            "--target".to_owned(),
            target,
            "--target-dir".to_owned(),
            target_dir.clone(),
        ];

        if self.release {
            cargo_args.push("--release".to_owned());
        }

        let thunk = Thunk {
            rust_flags,
            cargo_args,
            os,
            arch,
            target_dir,
        };

        Ok(thunk)
    }
}

#[derive(Debug)]
pub struct Thunk {
    rust_flags: Vec<String>,
    cargo_args: Vec<String>,
    os: OS,
    arch: Arch,
    target_dir: String,
}

impl Thunk {
    pub fn run(self) {
        let rust_flags = self.rust_flags.join(" ");
        let cargo_args = self.cargo_args;

        println!(
            "Start to build for Windows {}({}) using VC-LTL and YY-Thunks: ",
            self.os.to_string(),
            self.arch.to_string(),
        );
        println!(" * RUSTFLAGS = {}", rust_flags);
        println!(" * Command = cargo {}", cargo_args.join(" "));
        println!("Cargo Output:");

        let _status = Command::new("cargo")
            .env("RUSTFLAGS", rust_flags)
            .args(cargo_args)
            .status()
            .unwrap();

        println!(
            "You can find the builds in target directory: {}",
            self.target_dir
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OS {
    WindowsXP,
    WindowsVista,
    Windows7,
    Windows8,
    Windows10,
    Windows10_20H1,
}

impl OS {
    pub fn from_str(text: &str) -> Self {
        match text.to_lowercase().as_str() {
            "windows xp" | "winxp" | "xp" | "5.1" | "5.2" | "x" | "2600" | "3790" | "2003" => {
                OS::WindowsXP
            }
            "windows vista" | "winvista" | "vista" | "6.0" | "v" | "6000" | "2008" => {
                OS::WindowsVista
            }
            "windows 7" | "win7" | "7" | "6.1" | "7600" | "2008r2" => OS::Windows7,
            "windows 8" | "win8" | "8" | "6.2" | "9200" | "2012" => OS::Windows8,
            "windows 10" | "win10" | "10" | "10240" | "2016" | "2019" => OS::Windows10,
            "windows 10 20h1" | "win10 20h1" | "20h1" | "19041" | "2020" => OS::Windows10_20H1,
            _ => OS::WindowsXP,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            OS::WindowsXP => "XP".to_string(),
            OS::WindowsVista => "Vista".to_string(),
            OS::Windows7 => "7".to_string(),
            OS::Windows8 => "8".to_string(),
            OS::Windows10 => "10".to_string(),
            OS::Windows10_20H1 => "10_20h1".to_string(),
        }
    }
}

impl From<String> for OS {
    fn from(value: String) -> Self {
        Self::from_str(&value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    Win32,
    X64,
    ARM,
    ARM64,
}

impl Arch {
    pub fn from_str(text: &str) -> Self {
        match text.to_ascii_lowercase().as_str() {
            "win32" | "32" | "x86" | "86" | "i686" | "x32" | "3" | "8" => Arch::Win32,
            "6" | "64" | "x64" | "x86_64" | "x8664" | "amd64" => Arch::X64,
            "arm" | "a" | "ar" => Arch::ARM,
            "aarch64" | "arm64" | "a64" | "ar64" => Arch::ARM64,
            _ => Arch::Win32,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Win32 => "Win32".to_owned(),
            Self::X64 => "x64".to_owned(),
            Self::ARM64 => "ARM64".to_owned(),
            Self::ARM => "ARM".to_owned(),
        }
    }

    pub fn to_rust_target(&self) -> Option<String> {
        match self {
            Arch::Win32 => Some("i686-pc-windows-msvc".to_owned()),
            Arch::X64 => Some("x86_64-pc-windows-msvc".to_owned()),
            Arch::ARM64 => Some("aarch64-pc-windows-msvc".to_owned()),
            _ => None,
        }
    }
}

impl From<String> for Arch {
    fn from(value: String) -> Self {
        Self::from_str(&value)
    }
}

pub fn get_vc_ltl_os_lib_path(os: OS, arch: Arch) -> Option<PathBuf> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Some(PathBuf::from("TargetPlatform/5.1.2600.0/lib/Win32")),
        (OS::WindowsXP, Arch::X64) => Some(PathBuf::from("TargetPlatform/5.2.3790.0/lib/x64")),

        (OS::WindowsVista | OS::Windows7, Arch::Win32) => {
            Some(PathBuf::from("TargetPlatform/6.0.6000.0/lib/Win32"))
        }
        (OS::WindowsVista | OS::Windows7, Arch::X64) => {
            Some(PathBuf::from("TargetPlatform/6.0.6000.0/lib/x64"))
        }

        (OS::Windows8, Arch::Win32) => Some(PathBuf::from("TargetPlatform/6.2.9200.0/lib/Win32")),
        (OS::Windows8, Arch::X64) => Some(PathBuf::from("TargetPlatform/6.2.9200.0/lib/x64")),
        (OS::Windows8, Arch::ARM) => Some(PathBuf::from("TargetPlatform/6.2.9200.0/lib/ARM")),

        (OS::Windows10, Arch::Win32) => {
            Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/Win32"))
        }
        (OS::Windows10, Arch::X64) => Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/x64")),
        (OS::Windows10, Arch::ARM) => Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/ARM")),
        (OS::Windows10, Arch::ARM64) => {
            Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/ARM64"))
        }

        (OS::Windows10_20H1, Arch::Win32) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/Win32"))
        }
        (OS::Windows10_20H1, Arch::X64) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/X64"))
        }
        (OS::Windows10_20H1, Arch::ARM) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/ARM"))
        }
        (OS::Windows10_20H1, Arch::ARM64) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/ARM64"))
        }
        _ => None,
    }
}

pub fn get_yy_thunks_obj_path(os: OS, arch: Arch) -> Option<PathBuf> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Some(PathBuf::from("objs/x86/YY_Thunks_for_WinXP.obj")),
        (OS::WindowsXP, Arch::X64) => Some(PathBuf::from("objs/x64/YY_Thunks_for_WinXP.obj")),
        (OS::WindowsVista, Arch::Win32) => Some(PathBuf::from("objs/x86/YY_Thunks_for_Vista.obj")),
        (OS::WindowsVista, Arch::X64) => Some(PathBuf::from("objs/x64/YY_Thunks_for_Vista.obj")),
        _ => None,
    }
}

pub fn get_os_version(os: OS, arch: Arch) -> Option<String> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Some("5.01".to_owned()),
        (OS::WindowsXP, Arch::X64) => Some("5.02".to_owned()),
        (OS::WindowsVista, _) => Some("6.00".to_owned()),
        (OS::Windows7, _) => Some("6.01".to_owned()),
        (OS::Windows8, _) => Some("6.02".to_owned()),
        (OS::Windows10, _) => Some("10.0".to_owned()),
        (OS::Windows10_20H1, _) => Some("10.0".to_owned()),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subsystem {
    Windows,
    Console,
}

impl Subsystem {
    pub fn from_str(subsystem: &str) -> Self {
        match subsystem.to_lowercase().as_ref() {
            "window" | "windows" | "win" | "w" | "gui" | "g" | "ui" | "u" => Self::Windows,
            "console" | "command" | "line" | "c" => Self::Console,
            _ => Self::Console,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Subsystem::Windows => "WINDOWS".to_owned(),
            Subsystem::Console => "CONSOLE".to_owned(),
        }
    }
}

impl From<String> for Subsystem {
    fn from(value: String) -> Self {
        Self::from_str(&value)
    }
}
