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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    Win32,
    X64,
    ARM64,
}

impl Arch {
    pub(crate) fn from_rust_target(target: &str) -> Option<Self> {
        match target {
            "x86" => Some(Arch::Win32),
            "x86_64" => Some(Arch::X64),
            "aarch64" => Some(Arch::ARM64),
            _ => None
        }
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
        // (OS::Windows8, Arch::ARM) => Some(PathBuf::from("TargetPlatform/6.2.9200.0/lib/ARM")),
        (OS::Windows10, Arch::Win32) => {
            Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/Win32"))
        }
        (OS::Windows10, Arch::X64) => Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/x64")),
        (OS::Windows10, Arch::ARM64) => {
            Some(PathBuf::from("TargetPlatform/10.0.10240.0/lib/ARM64"))
        }

        (OS::Windows10_20H1, Arch::Win32) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/Win32"))
        }
        (OS::Windows10_20H1, Arch::X64) => {
            Some(PathBuf::from("TargetPlatform/10.0.19041.0/lib/X64"))
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
