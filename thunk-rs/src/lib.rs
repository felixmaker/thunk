mod sys;
use std::env;
use std::path::PathBuf;

pub use sys::*;

const ENV_VAR_VC_LTL5: &str = "VC_LTL";
const ENV_VAR_YY_THUNKS: &str = "YY_THUNKS";

pub struct ThunkBuilder {
    os: Option<OS>,
    arch: Option<Arch>,
    vc_ltl_path: Option<PathBuf>,
    yy_thunks_path: Option<PathBuf>,
}

impl Default for ThunkBuilder {
    fn default() -> Self {
        let vc_ltl_path = env::var(ENV_VAR_VC_LTL5).expect(&format!(
            "You need to set `{}` environment variable.",
            ENV_VAR_VC_LTL5
        ));
        let yy_thunks_path = env::var(ENV_VAR_YY_THUNKS).expect(&format!(
            "You need to set `{}` environment variable.",
            ENV_VAR_YY_THUNKS
        ));
        let arch = Arch::from_rust_target(&env::var("CARGO_CFG_TARGET_ARCH").unwrap());
        Self {
            os: Some(OS::Windows7),
            arch,
            vc_ltl_path: Some(PathBuf::from(vc_ltl_path)),
            yy_thunks_path: Some(PathBuf::from(yy_thunks_path)),
        }
    }
}

impl ThunkBuilder {
    fn build(&self) {
        if let (Some(os), Some(arch)) = (self.os, self.arch) {
            todo!()
        }
    }
}
