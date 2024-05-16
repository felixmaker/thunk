use crate::error::{ThunkError, ThunkResult};
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
    pub(crate) fn to_string(&self) -> String {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    Win32,
    X64,
    ARM64,
}

impl Arch {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Self::Win32 => "Win32".to_owned(),
            Self::X64 => "x64".to_owned(),
            Self::ARM64 => "ARM64".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subsystem {
    Windows,
    Console,
}

impl Subsystem {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Subsystem::Windows => "WINDOWS".to_owned(),
            Subsystem::Console => "CONSOLE".to_owned(),
        }
    }
}

impl Arch {
    pub(crate) fn from_rust_target(target: &str) -> ThunkResult<Self> {
        match target {
            "x86" => Ok(Arch::Win32),
            "x86_64" => Ok(Arch::X64),
            "aarch64" => Ok(Arch::ARM64),
            _ => Err(ThunkError::UnsupportedArch),
        }
    }
}

pub(crate) fn get_vc_ltl_os_lib_path(os: OS, arch: Arch) -> ThunkResult<PathBuf> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Ok(PathBuf::from("TargetPlatform/5.1.2600.0/lib/Win32")),
        (OS::WindowsXP, Arch::X64) => Ok(PathBuf::from("TargetPlatform/5.2.3790.0/lib/x64")),

        (OS::WindowsVista | OS::Windows7, Arch::Win32) => {
            Ok(PathBuf::from("TargetPlatform/6.0.6000.0/lib/Win32"))
        }
        (OS::WindowsVista | OS::Windows7, Arch::X64) => {
            Ok(PathBuf::from("TargetPlatform/6.0.6000.0/lib/x64"))
        }

        (OS::Windows8, Arch::Win32) => Ok(PathBuf::from("TargetPlatform/6.2.9200.0/lib/Win32")),
        (OS::Windows8, Arch::X64) => Ok(PathBuf::from("TargetPlatform/6.2.9200.0/lib/x64")),
        // (OS::Windows8, Arch::ARM) => Some(PathBuf::from("TargetPlatform/6.2.9200.0/lib/ARM")),
        (OS::Windows10, Arch::Win32) => Ok(PathBuf::from("TargetPlatform/10.0.10240.0/lib/Win32")),
        (OS::Windows10, Arch::X64) => Ok(PathBuf::from("TargetPlatform/10.0.10240.0/lib/x64")),
        (OS::Windows10, Arch::ARM64) => Ok(PathBuf::from("TargetPlatform/10.0.10240.0/lib/ARM64")),

        (OS::Windows10_20H1, Arch::Win32) => {
            Ok(PathBuf::from("TargetPlatform/10.0.19041.0/lib/Win32"))
        }
        (OS::Windows10_20H1, Arch::X64) => Ok(PathBuf::from("TargetPlatform/10.0.19041.0/lib/X64")),
        (OS::Windows10_20H1, Arch::ARM64) => {
            Ok(PathBuf::from("TargetPlatform/10.0.19041.0/lib/ARM64"))
        }
        _ => Err(ThunkError::UnsupportedByVCLTL(os, arch)),
    }
}

pub(crate) fn get_yy_thunks_obj_path(os: OS, arch: Arch) -> ThunkResult<PathBuf> {
    match (os, arch) {
        (OS::WindowsXP, Arch::Win32) => Ok(PathBuf::from("objs/x86/YY_Thunks_for_WinXP.obj")),
        (OS::WindowsXP, Arch::X64) => Ok(PathBuf::from("objs/x64/YY_Thunks_for_WinXP.obj")),
        (_, Arch::Win32) => Ok(PathBuf::from("objs/x86/YY_Thunks_for_Vista.obj")),
        (_, Arch::X64) => Ok(PathBuf::from("objs/x64/YY_Thunks_for_Vista.obj")),
        _ => Err(ThunkError::UnsupportedByYYThunks),
    }
}

pub(crate) fn get_os_version(os: OS, arch: Arch) -> Option<String> {
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
