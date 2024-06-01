use anyhow::anyhow;
use clap::Parser;
use std::{collections::HashMap, path::PathBuf, process::Command};

mod sys;
use sys::*;

const ENV_VAR_VC_LTL5: &str = "VC_LTL";
const ENV_VAR_YY_THUNKS: &str = "YY_THUNKS";

/// Use Thunk to build your Rust program that runs on old Windows platforms.
#[derive(Debug, Parser)]
pub struct ThunkBuilder {
    /// Operating system: xp, vista, win7, win10, 20h1 (dafault: win7)
    #[arg(short, long, value_name = "OS")]
    os: Option<OS>,
    /// Operating system arch: x86, x64, arm64 (dafault: current os arch)
    #[arg(short, long)]
    arch: Option<Arch>,
    /// To build a shared library
    #[arg(long, value_name = "IS_LIB")]
    lib: bool,
    /// Link arg: console, windows (default: console)
    #[arg(short, long)]
    subsystem: Option<Subsystem>,
    /// Args pass to cargo: cargo build <CARGO_ARGS>
    #[arg(last = true, value_name = "CARGO_ARGS")]
    cargo_args: Vec<String>,
}

impl ThunkBuilder {
    pub fn build(mut self) -> anyhow::Result<Thunk> {
        let env_vars: HashMap<String, String> = std::env::vars().collect();

        let mut vc_ltl = {
            let vc_ltl_env_path = env_vars.get(ENV_VAR_VC_LTL5).ok_or_else(|| {
                anyhow!("You need to set {} environment variable.", ENV_VAR_VC_LTL5)
            })?;
            PathBuf::from(vc_ltl_env_path)
        };

        let os = self.os.unwrap_or(OS::Windows7);

        let arch = if let Ok(arch_from_args) = get_arch_from_args(self.cargo_args.as_slice()) {
            arch_from_args
        } else {
            let un_arch = self.arch.unwrap_or(get_default_arch()?);
            let target = un_arch
                .to_rust_target()
                .ok_or_else(|| anyhow!("arch {} fail translate to target", un_arch.to_string()))?;
            self.cargo_args.extend(["--target".to_owned(), target]);
            un_arch
        };

        let os_lib =
            get_vc_ltl_os_lib_path(os, arch).ok_or_else(|| anyhow!("os or arch is wrong"))?;

        vc_ltl.push(os_lib);

        let os_version =
            get_os_version(os, arch).ok_or_else(|| anyhow!("failed to get os version"))?;

        let is_lib = { get_is_lib_from_args(self.cargo_args.as_slice()) || self.lib };

        let mut subsystem = Some(self.subsystem.unwrap_or(Subsystem::Console));

        if is_lib {
            subsystem = None;
        }

        let subsystem_args =
            subsystem.map(|x| format!("-Clink-args=/SUBSYSTEM:{},{}", x.to_string(), os_version));

        let mut rust_flags = vec!["-L".into(), format!("{}", vc_ltl.to_string_lossy())];

        if let Some(args) = subsystem_args {
            rust_flags.push(args.into());

            if let Some(Subsystem::Windows) = subsystem {
                rust_flags.push("-Clink-args=/ENTRY:mainCRTStartup".into())
            }
        }

        let thunks_obj = {
            let mut thunks = {
                let yy_thunks_env_path = env_vars.get(ENV_VAR_YY_THUNKS).ok_or_else(|| {
                    anyhow!(
                        "You need to set {} environment variable.",
                        ENV_VAR_YY_THUNKS
                    )
                })?;
                PathBuf::from(yy_thunks_env_path)
            };

            let os_obj = get_yy_thunks_obj_path(os, arch).ok_or_else(|| anyhow!(""))?;
            thunks.push(os_obj);
            Some(thunks)
        };

        if let Some(obj) = thunks_obj {
            rust_flags.push(format!("-Clink-args={}", obj.to_string_lossy()));
        }

        let target_dir = format!("./target/win{}_build", os.to_string().to_ascii_lowercase());

        let mut cargo_args = vec![
            "build".to_owned(),
            "--target-dir".to_owned(),
            target_dir.clone(),
        ];

        cargo_args.extend(self.cargo_args);

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
