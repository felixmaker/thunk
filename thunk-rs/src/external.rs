use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::CompressedType;

pub fn get_or_download(
    env_path: &str,
    env_url: &str,
    default_url: &str,
    out_dir: &Path,
    unpack_name: &str,
    _compressed_type: CompressedType, // 7z auto-detects format
) -> PathBuf {
    if let Ok(env_path) = env::var(env_path) {
        return PathBuf::from(env_path);
    }

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
        panic!("Unpack failed!")
    }

    unpack_dir
}
