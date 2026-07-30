use crate::setting::FUSION_TOOL_VERSION;
use anyhow::{anyhow, bail, Context, Result};
use console::style;
use serde::Deserialize;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::{env, fs};

const REPO: &str = "cipherunits/fusion-tool";

const USER_AGENT: &str = concat!("fusion-tool/", env!("CARGO_PKG_VERSION"));

/// Release archives are limited to a size that a CLI binary cannot exceed
const DOWNLOAD_LIMIT: u64 = 64 * 1024 * 1024;

#[cfg(windows)]
const BINARY_NAME: &str = "fusion.exe";

#[cfg(not(windows))]
const BINARY_NAME: &str = "fusion";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn update() -> Result<()> {
    println!();
    println!("{}", style("Checking for updates...").cyan().bold());
    println!();

    let release = latest_release()?;

    let latest = release.tag_name.trim_start_matches('v');

    println!("  Installed: {}", style(FUSION_TOOL_VERSION).yellow());
    println!("  Latest:    {}", style(latest).yellow());
    println!();

    if !is_newer(latest, FUSION_TOOL_VERSION) {
        println!(
            "{}",
            style("✔ Already on the latest version!").green().bold()
        );
        println!();

        return Ok(());
    }

    let target = target()?;

    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name.contains(target))
        .ok_or_else(|| {
            anyhow!(
                "Release {} has no prebuilt binary for {}",
                release.tag_name,
                target
            )
        })?;

    println!("Downloading {}...", asset.name);

    let archive = download(&asset.browser_download_url)?;

    let binary = extract_binary(&archive, &asset.name)?;

    let path = replace_running_binary(&binary)?;

    println!();
    println!(
        "{}",
        style(format!("✔ Updated to v{}!", latest)).green().bold()
    );
    println!();
    println!("  Location: {}", style(path.display()).yellow());
    println!();

    Ok(())
}

fn latest_release() -> Result<Release> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let body = ureq::get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
        .with_context(|| format!("Could not reach {}", url))?
        .body_mut()
        .read_to_vec()
        .context("Could not read the release information")?;

    serde_json::from_slice(&body).context("Could not parse the release information")
}

fn download(url: &str) -> Result<Vec<u8>> {
    ureq::get(url)
        .header("User-Agent", USER_AGENT)
        .call()
        .with_context(|| format!("Could not download {}", url))?
        .body_mut()
        .with_config()
        .limit(DOWNLOAD_LIMIT)
        .read_to_vec()
        .context("Could not read the downloaded archive")
}

/// Release target of this binary. These are compile time constants, so an x86
/// build running on arm keeps updating to the build it already is.
fn target() -> Result<&'static str> {
    Ok(match (env::consts::OS, env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",

        ("macos", "x86_64") => "x86_64-apple-darwin",

        ("windows", "x86_64") => "x86_64-pc-windows-msvc",

        (os, arch) => bail!(
            "No prebuilt release for {} {}. Update from source with `cargo install --path .`",
            os,
            arch
        ),
    })
}

fn extract_binary(archive: &[u8], name: &str) -> Result<Vec<u8>> {
    if name.ends_with(".zip") {
        return extract_from_zip(archive);
    }

    if name.ends_with(".tar.gz") {
        return extract_from_tar_gz(archive);
    }

    bail!("Unsupported archive format: {}", name)
}

fn extract_from_zip(archive: &[u8]) -> Result<Vec<u8>> {
    let mut zip =
        zip::ZipArchive::new(Cursor::new(archive)).context("Could not open the zip archive")?;

    let mut file = zip
        .by_name(BINARY_NAME)
        .with_context(|| format!("{} is missing from the archive", BINARY_NAME))?;

    let mut binary = Vec::new();

    file.read_to_end(&mut binary)
        .context("Could not read the binary from the zip archive")?;

    Ok(binary)
}

fn extract_from_tar_gz(archive: &[u8]) -> Result<Vec<u8>> {
    let decoder = flate2::read::GzDecoder::new(Cursor::new(archive));

    let mut tar = tar::Archive::new(decoder);

    for entry in tar.entries().context("Could not open the tar archive")? {
        let mut entry = entry.context("Could not read the tar archive")?;

        let is_binary = {
            let path = entry.path().context("Could not read an archive entry")?;

            path.file_name().and_then(|name| name.to_str()) == Some(BINARY_NAME)
        };

        if is_binary {
            let mut binary = Vec::new();

            entry
                .read_to_end(&mut binary)
                .context("Could not read the binary from the tar archive")?;

            return Ok(binary);
        }
    }

    bail!("{} is missing from the archive", BINARY_NAME)
}

/// Swap the downloaded binary in for the one that is currently running
fn replace_running_binary(binary: &[u8]) -> Result<PathBuf> {
    let current = env::current_exe().context("Could not locate the running fusion binary")?;

    let staged = env::temp_dir().join(format!("fusion-update-{}", std::process::id()));

    fs::write(&staged, binary)
        .with_context(|| format!("Could not write to {}", staged.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(&staged, fs::Permissions::from_mode(0o755))
            .context("Could not mark the downloaded binary as executable")?;
    }

    let replaced = self_replace::self_replace(&staged).map_err(|error| {
        anyhow!(
            "Could not replace {}: {}\n\
             If fusion is installed system wide, run the update with elevated permissions.",
            current.display(),
            error
        )
    });

    let _ = fs::remove_file(&staged);

    replaced?;

    Ok(current)
}

/// Compare dot separated versions numerically, so 1.10.0 outranks 1.9.0
fn is_newer(candidate: &str, current: &str) -> bool {
    numbers(candidate) > numbers(current)
}

fn numbers(version: &str) -> Vec<u64> {
    // A prerelease such as 1.0.3-beta.1 counts as 1.0.3, so it never outranks
    // the final 1.0.3.
    let release = version.split('-').next().unwrap_or(version);

    release
        .split('.')
        .map(|part| part.parse().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("1.0.4", "1.0.3"));
        assert!(is_newer("1.1.0", "1.0.9"));
        assert!(is_newer("1.10.0", "1.9.0"));
        assert!(is_newer("2.0.0", "1.99.99"));

        assert!(!is_newer("1.0.3", "1.0.3"));
        assert!(!is_newer("1.0.2", "1.0.3"));
        assert!(!is_newer("0.9.9", "1.0.0"));
        assert!(!is_newer("1.0.3-beta.1", "1.0.3"));
    }

    /// The zip path is what Windows updates go through
    #[test]
    fn test_extract_from_zip() {
        use std::io::Write;

        let mut buffer = Cursor::new(Vec::new());

        {
            let mut writer = zip::ZipWriter::new(&mut buffer);

            writer
                .start_file(BINARY_NAME, zip::write::SimpleFileOptions::default())
                .unwrap();

            writer.write_all(b"a fusion binary").unwrap();

            writer.finish().unwrap();
        }

        let archive = buffer.into_inner();

        assert_eq!(extract_from_zip(&archive).unwrap(), b"a fusion binary");
        assert!(extract_binary(&archive, "fusion-v1.0.0-target.zip").is_ok());
    }

    #[test]
    fn test_numbers_drops_prerelease_suffix() {
        assert_eq!(numbers("1.0.3"), vec![1, 0, 3]);
        assert_eq!(numbers("1.0.3-beta.1"), vec![1, 0, 3]);
        assert_eq!(numbers("nonsense"), vec![0]);
    }
}
