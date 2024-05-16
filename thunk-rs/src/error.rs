use crate::{Arch, OS};

pub type ThunkResult<T> = std::result::Result<T, ThunkError>;

#[non_exhaustive]
#[derive(Debug)]
pub enum ThunkError {
    /// Raised when using thunk on unsupported platform. Currently, thunk only support windows(msvc).
    UnsupportedPlatform,
    /// Raised when using thunk on unsupported arch. Currently, thunk only support x86, x86_64 and aarch64.
    UnsupportedArch,
    /// Raised when VC-LTL5 failed to support.
    UnsupportedByVCLTL(OS, Arch),
    /// Raised when YY-Thunk failed to support.
    UnsupportedByYYThunks,
    /// Raised when no default environment variable found.
    EnvNotFound(&'static str),
}

impl ThunkError {
    pub(crate) fn to_string(&self) -> String {
        use ThunkError::*;
        match self {
            UnsupportedPlatform => "Only `Windows(MSVC)` is supported!".into(),
            UnsupportedArch => "Only `x86`, `x86_64` and `aarch64` are supported!".into(),
            UnsupportedByVCLTL(os, arch) => format!(
                "`Windows {}({})` is unsupported by VC-LTL5!",
                os.to_string(),
                arch.to_string()
            ),
            UnsupportedByYYThunks => "Only `x86` and `x86_64` are supported!".into(),
            EnvNotFound(env) => {
                format!("You need to set environment variable `{}` as default!", env)
            }
        }
    }
}

impl std::fmt::Display for ThunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl std::error::Error for ThunkError {}
