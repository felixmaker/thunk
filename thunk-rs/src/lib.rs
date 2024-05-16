#![doc = include_str!("../README.md")]

mod error;
mod sys;

use std::env;
use std::path::{Path, PathBuf};

pub use error::*;
pub use sys::*;

const ENV_VAR_VC_LTL5: &'static str = "VC_LTL";
const ENV_VAR_YY_THUNKS: &'static str = "YY_THUNKS";

pub struct Thunk {
    subsystem: String,
    vc_ltl_path: String,
    yy_thunks_obj: Option<String>,
}

impl Thunk {
    /// Thunk the Rust program. Call it in build script.
    pub fn thunk(&self) {
        println!("cargo::warning=VC-LTL5 Enabled: {}", self.vc_ltl_path);
        println!("cargo::rustc-link-search={}", self.vc_ltl_path);

        if let Some(yy_thunks_obj) = &self.yy_thunks_obj {
            println!("cargo::warning=YY-Thunks Enabled: {}", yy_thunks_obj);
            println!("cargo::rustc-link-arg=/SUBSYSTEM:{}", self.subsystem);
            if self.subsystem.contains("WINDOWS") {
                // https://github.com/rust-lang/rust/blob/bf8801d36dfd28de7d3b0279b53d38593acdfd14/compiler/rustc_codegen_ssa/src/back/linker.rs#L1011
                println!("cargo::rustc-link-arg=/ENTRY:mainCRTStartup");
            }
            println!("cargo::rustc-link-arg={}", yy_thunks_obj);
        }
    }
}

#[derive(Default)]
pub struct ThunkBuilder {
    thunk: bool,
    os: Option<OS>,
    subsystem: Option<Subsystem>,
    vc_ltl_path: Option<PathBuf>,
    yy_thunks_path: Option<PathBuf>,
}

impl ThunkBuilder {
    /// Set which OS to support.
    ///
    /// Note: Windows XP and Windows Vista will auto use YY-Thunks.
    pub fn with_os(mut self, os: OS) -> Self {
        match os {
            OS::WindowsXP | OS::WindowsVista => self.thunk = true,
            _ => {}
        }
        self.os = Some(os);
        self
    }

    /// Enforce YY-Thunks enabled. Call this may cause the program failed to compile.
    pub fn with_thunk_enforced(mut self, thunk: bool) -> Self {
        self.thunk = thunk;
        self
    }

    /// Set the subsystem. Windows is for GUI. Console is dafault.
    pub fn with_subsystem(mut self, subsystem: Subsystem) -> Self {
        self.subsystem = Some(subsystem);
        self
    }

    /// Set the VC-LTL5 path. Default to `VC_LTL` from env.
    pub fn with_vc_ltl_path<P>(mut self, vc_ltl_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.vc_ltl_path = Some(PathBuf::from(vc_ltl_path.as_ref()));
        self
    }

    /// Set the YY-Thunks path. Default to `YY_THUNKS` from env.
    pub fn with_yy_thunks_path<P>(mut self, yy_thunks_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.thunk = true;
        self.yy_thunks_path = Some(PathBuf::from(yy_thunks_path.as_ref()));
        self
    }

    /// Build the Thunk.
    pub fn build(self) -> ThunkResult<Thunk> {
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

        if !target_os.contains("windows") || !target_env.contains("msvc") {
            return Err(ThunkError::UnsupportedPlatform);
        }

        let os = self.os.unwrap_or(OS::Windows7);
        let arch = Arch::from_rust_target(&env::var("CARGO_CFG_TARGET_ARCH").unwrap())?;
        let vc_ltl_lib = get_vc_ltl_os_lib_path(os, arch)?;

        let vc_ltl_path = self.vc_ltl_path.unwrap_or(PathBuf::from(
            env::var(ENV_VAR_VC_LTL5).map_err(|_| ThunkError::EnvNotFound(&ENV_VAR_VC_LTL5))?,
        ));

        let vc_ltl_path = vc_ltl_path.join(vc_ltl_lib).to_string_lossy().to_string();

        let yy_thunks_obj = if self.thunk {
            let yy_thunks_path = self.yy_thunks_path.unwrap_or(PathBuf::from(
                env::var(ENV_VAR_YY_THUNKS)
                    .map_err(|_| ThunkError::EnvNotFound(ENV_VAR_YY_THUNKS))?,
            ));
            let yy_thunks_obj = get_yy_thunks_obj_path(os, arch)?;
            Some(
                yy_thunks_path
                    .join(yy_thunks_obj)
                    .to_string_lossy()
                    .to_string(),
            )
        } else {
            None
        };

        let subsystem = self.subsystem.unwrap_or(Subsystem::Console);

        let os_version = get_os_version(os, arch).unwrap_or("6.00".into());
        let subsystem = format!("{},{}", subsystem.to_string(), os_version);

        Ok(Thunk {
            subsystem,
            vc_ltl_path,
            yy_thunks_obj,
        })
    }
}
