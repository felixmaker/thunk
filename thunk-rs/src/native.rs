use std::env;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

use reqwest::blocking::Client;

use crate::CompressedType;

pub fn get_or_download(
    env_path: &str,
    env_url: &str,
    default_url: &str,
    out_dir: &Path,
    unpack_name: &str,
    compressed_type: CompressedType,
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
        env_url
    } else {
        default_url.to_string()
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(10 * 60))
        .build()
        .expect("Failed to build HTTP client");
    let reader = Cursor::new(
        client
            .get(&url)
            .send()
            .expect("Download libraries failed")
            .bytes()
            .expect("Failed to read downloaded bytes"),
    );
    match compressed_type {
        CompressedType::SevenZip => {
            sevenz_rust2::decompress(reader, &unpack_dir).expect("Failed to decompress 7z archive");
        }
        CompressedType::Zip => {
            zip::ZipArchive::new(reader)
                .expect("Failed to read zip archive")
                .extract(&unpack_dir)
                .expect("Failed to extract zip archive");
        }
    }

    unpack_dir
}
