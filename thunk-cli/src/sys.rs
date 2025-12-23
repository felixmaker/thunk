use anyhow::Result;
use std::path::PathBuf;

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
            "windows xp" | "winxp" | "xp" | "5.1" | "5.2" | "2600" | "3790" | "2003" => {
                OS::WindowsXP
            }
            "windows vista" | "winvista" | "vista" | "6.0" | "6000" | "2008" => OS::WindowsVista,
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
    ARM64,
}

impl Arch {
    pub fn from_str(text: &str) -> Self {
        match text.to_ascii_lowercase().as_str() {
            "win32" | "32" | "x86" | "86" | "i686" | "x32" => Arch::Win32,
            "64" | "x64" | "x86_64" | "x8664" | "amd64" => Arch::X64,
            "arm" | "aarch64" | "arm64" => Arch::ARM64,
            _ => Arch::Win32,
        }
    }

    pub fn from_rust_target(host: &str) -> Result<Arch> {
        match host.to_ascii_lowercase().as_str() {
            "i686-pc-windows-msvc" => Ok(Arch::Win32),
            "x86_64-pc-windows-msvc" => Ok(Arch::X64),
            "aarch64-pc-windows-msvc" => Ok(Arch::ARM64),
            _ => Err(anyhow::anyhow!("Host {} is not support!", host)),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Win32 => "Win32".to_owned(),
            Self::X64 => "x64".to_owned(),
            Self::ARM64 => "ARM64".to_owned(),
        }
    }

    pub fn to_rust_target(&self) -> Option<String> {
        match self {
            Arch::Win32 => Some("i686-pc-windows-msvc".to_owned()),
            Arch::X64 => Some("x86_64-pc-windows-msvc".to_owned()),
            Arch::ARM64 => Some("aarch64-pc-windows-msvc".to_owned()),
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
        (OS::WindowsXP, Arch::Win32) => Some("TargetPlatform/5.1.2600.0/lib/Win32"),
        (OS::WindowsXP, Arch::X64) => Some("TargetPlatform/5.2.3790.0/lib/x64"),

        (OS::WindowsVista | OS::Windows7, Arch::Win32) => {
            Some("TargetPlatform/6.0.6000.0/lib/Win32")
        }
        (OS::WindowsVista | OS::Windows7, Arch::X64) => Some("TargetPlatform/6.0.6000.0/lib/x64"),

        (OS::Windows8, Arch::Win32) => Some("TargetPlatform/6.2.9200.0/lib/Win32"),
        (OS::Windows8, Arch::X64) => Some("TargetPlatform/6.2.9200.0/lib/x64"),
        // (OS::Windows8, Arch::ARM) => Some("TargetPlatform/6.2.9200.0/lib/ARM"),
        (OS::Windows10, Arch::Win32) => Some("TargetPlatform/10.0.10240.0/lib/Win32"),
        (OS::Windows10, Arch::X64) => Some("TargetPlatform/10.0.10240.0/lib/x64"),
        (OS::Windows10, Arch::ARM64) => Some("TargetPlatform/10.0.10240.0/lib/ARM64"),

        (OS::Windows10_20H1, Arch::Win32) => Some("TargetPlatform/10.0.19041.0/lib/Win32"),
        (OS::Windows10_20H1, Arch::X64) => Some("TargetPlatform/10.0.19041.0/lib/X64"),
        (OS::Windows10_20H1, Arch::ARM64) => Some("TargetPlatform/10.0.19041.0/lib/ARM64"),
        _ => None,
    }
    .map(PathBuf::from)
}

pub fn get_yy_thunks_obj_path(os: OS, arch: Arch) -> Option<PathBuf> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Some("objs/x86/YY_Thunks_for_WinXP.obj"),
        (OS::WindowsXP, Arch::X64) => Some("objs/x64/YY_Thunks_for_WinXP.obj"),
        (OS::WindowsVista, Arch::Win32) => Some("objs/x86/YY_Thunks_for_Vista.obj"),
        (OS::WindowsVista, Arch::X64) => Some("objs/x64/YY_Thunks_for_Vista.obj"),
        (OS::Windows7, Arch::Win32) => Some("objs/x86/YY_Thunks_for_Win7.obj"),
        (OS::Windows7, Arch::X64) => Some("objs/x64/YY_Thunks_for_Win7.obj"),
        (OS::Windows8, Arch::Win32) => Some("objs/x86/YY_Thunks_for_Win8.obj"),
        (OS::Windows8, Arch::X64) => Some("objs/x64/YY_Thunks_for_Win8.obj"),
        (OS::Windows10, Arch::Win32) => Some("objs/x86/YY_Thunks_for_Win10.0.10240.obj"),
        (OS::Windows10, Arch::X64) => Some("objs/x64/YY_Thunks_for_Win10.0.10240.obj"),
        (OS::Windows10_20H1, Arch::Win32) => Some("objs/x86/YY_Thunks_for_Win10.0.19041.obj"),
        (OS::Windows10_20H1, Arch::X64) => Some("objs/x64/YY_Thunks_for_Win10.0.19041.obj"),
        _ => None,
    }
    .map(PathBuf::from)
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

    pub fn to_string(&self) -> String {
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

pub fn get_default_arch() -> Result<Arch> {
    get_default_arch_from_rustup()
}

fn get_default_arch_from_rustup() -> Result<Arch> {
    let rustup_show = std::process::Command::new("rustup").arg("show").output()?;
    let output = String::from_utf8(rustup_show.stdout)?;
    let first: Vec<&str> = output
        .lines()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Thunk is outdated"))?
        .split(':')
        .map(|x| x.trim())
        .collect();

    let host = first
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Thunk is outdated"))?;
    Arch::from_rust_target(host)
}

pub fn get_arch_from_args<I, S>(args: I) -> Result<Arch>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut host: Option<String> = None;
    let mut has_target = false;
    for arg in args {
        if arg.as_ref().starts_with("--target") {
            if arg.as_ref().contains("=") {
                let target: Vec<&str> = arg.as_ref().split('=').collect();
                let target = target.get(1);
                host = target.map(|x| x.to_string());
                break;
            } else {
                has_target = true;
                continue;
            };
        }
        if has_target {
            host = Some(arg.as_ref().to_owned());
            break;
        }
    }

    let host = host.ok_or_else(|| anyhow::anyhow!("Do not know arch"))?;
    Arch::from_rust_target(&host)
}

pub fn get_is_lib_from_args<I, S>(args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for arg in args {
        if arg.as_ref() == "--lib" {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arch_from_args() {
        let args = vec!["--target", "i686-pc-windows-msvc"];
        let result = get_arch_from_args(&args).unwrap();
        assert_eq!(Arch::Win32, result);

        let args = vec!["--target=i686-pc-windows-msvc"];
        let result = get_arch_from_args(&args).unwrap();
        assert_eq!(Arch::Win32, result);
    }

    #[test]
    fn test_get_arch_from_wrong_args() {
        let args: Vec<&str> = vec![];
        assert_eq!(get_arch_from_args(&args).is_ok(), false);

        let args = vec!["target", "i686-pc-windows-msvc"];
        assert_eq!(get_arch_from_args(&args).is_ok(), false);
    }

    #[test]
    fn test_get_is_lib_from_args() {
        let args: Vec<&str> = vec!["--lib"];
        assert_eq!(get_is_lib_from_args(&args), true);

        let args = vec![""];
        assert_eq!(get_is_lib_from_args(&args), false);
    }
}
