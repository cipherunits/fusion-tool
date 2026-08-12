use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::io::Cursor;
use std::path::{Component, Path, PathBuf};

const USER_AGENT: &str = concat!("fusion-tool/", env!("CARGO_PKG_VERSION"));
const DOWNLOAD_LIMIT: u64 = 128 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubSpec {
    pub owner: String,
    pub repo: String,
    pub reference: Option<String>,
}

impl GitHubSpec {
    pub fn source_label(&self) -> String {
        match &self.reference {
            Some(reference) => format!("github:{}/{}@{}", self.owner, self.repo, reference),
            None => format!("github:{}/{}", self.owner, self.repo),
        }
    }

    pub fn zipball_url(&self) -> String {
        match &self.reference {
            Some(reference) => format!(
                "https://api.github.com/repos/{}/{}/zipball/{}",
                self.owner, self.repo, reference
            ),
            None => format!(
                "https://api.github.com/repos/{}/{}/zipball",
                self.owner, self.repo
            ),
        }
    }
}

/// Parse `owner/repo`, `owner/repo@ref`, or a github.com URL.
pub fn parse_github_spec(input: &str) -> Result<GitHubSpec> {
    let trimmed = input.trim().trim_end_matches('/');

    if trimmed.is_empty() {
        bail!("GitHub module spec must not be empty");
    }

    let without_proto = trimmed
        .strip_prefix("https://")
        .or_else(|| trimmed.strip_prefix("http://"))
        .unwrap_or(trimmed);

    let without_www = without_proto
        .strip_prefix("www.")
        .unwrap_or(without_proto);

    let path = if let Some(rest) = without_www.strip_prefix("github.com/") {
        rest
    } else if without_www.contains("://") || without_www.starts_with("github.com") {
        bail!(
            "Unsupported GitHub URL '{}'. Use owner/repo or https://github.com/owner/repo",
            input
        );
    } else {
        without_www.strip_prefix("github:").unwrap_or(without_www)
    };

    let path = path.strip_prefix('/').unwrap_or(path);
    let (path, reference_from_url) = split_url_tree(path);

    let (repo_part, reference_from_at) = match path.split_once('@') {
        Some((repo, reference)) => (repo, Some(reference.to_string())),
        None => (path, None),
    };

    let mut parts = repo_part.split('/').filter(|p| !p.is_empty());
    let owner = parts
        .next()
        .ok_or_else(|| anyhow!("Missing owner in '{}'", input))?
        .to_string();
    let repo = parts
        .next()
        .ok_or_else(|| anyhow!("Missing repo in '{}'", input))?
        .trim_end_matches(".git")
        .to_string();

    if parts.next().is_some() && reference_from_url.is_none() {
        bail!(
            "Unexpected path in '{}'. Use owner/repo or owner/repo@ref",
            input
        );
    }

    if owner.is_empty() || repo.is_empty() {
        bail!("Invalid GitHub spec '{}'", input);
    }

    Ok(GitHubSpec {
        owner,
        repo,
        reference: reference_from_at.or(reference_from_url),
    })
}

fn split_url_tree(path: &str) -> (&str, Option<String>) {
    // github.com/owner/repo/tree/ref or /archive/ref.zip style — keep owner/repo
    let segments: Vec<&str> = path.split('/').collect();

    if segments.len() >= 4 && (segments[2] == "tree" || segments[2] == "commit") {
        return (
            &path[..segments[0].len() + 1 + segments[1].len()],
            Some(segments[3].to_string()),
        );
    }

    if segments.len() >= 2 {
        return (
            &path[..segments[0].len() + 1 + segments[1].len()],
            None,
        );
    }

    (path, None)
}

/// Download a repository zipball and extract it into `dest`, returning the
/// extracted root directory (GitHub wraps files in a top-level folder).
pub fn download_repo(spec: &GitHubSpec, dest: &Path) -> Result<PathBuf> {
    fs::create_dir_all(dest).with_context(|| format!("Could not create {}", dest.display()))?;

    let url = spec.zipball_url();

    let archive = ureq::get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/vnd.github+json")
        .call()
        .with_context(|| format!("Could not download {}", url))?
        .body_mut()
        .with_config()
        .limit(DOWNLOAD_LIMIT)
        .read_to_vec()
        .context("Could not read the downloaded module archive")?;

    extract_zip_to(&archive, dest)
}

fn extract_zip_to(archive: &[u8], dest: &Path) -> Result<PathBuf> {
    let mut zip =
        zip::ZipArchive::new(Cursor::new(archive)).context("Could not open the module zip")?;

    let mut root_name: Option<String> = None;

    for index in 0..zip.len() {
        let mut file = zip
            .by_index(index)
            .with_context(|| format!("Could not read zip entry {index}"))?;

        let Some(enclosed) = file.enclosed_name().map(|path| path.to_path_buf()) else {
            continue;
        };

        if root_name.is_none() {
            if let Some(Component::Normal(first)) = enclosed.components().next() {
                root_name = Some(first.to_string_lossy().into_owned());
            }
        }

        let out_path = dest.join(&enclosed);

        if file.is_dir() {
            fs::create_dir_all(&out_path)
                .with_context(|| format!("Could not create {}", out_path.display()))?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Could not create {}", parent.display()))?;
        }

        let mut outfile = fs::File::create(&out_path)
            .with_context(|| format!("Could not create {}", out_path.display()))?;

        std::io::copy(&mut file, &mut outfile)
            .with_context(|| format!("Could not write {}", out_path.display()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).ok();
            }
        }
    }

    let root = root_name
        .map(|name| dest.join(name))
        .filter(|path| path.is_dir())
        .unwrap_or_else(|| dest.to_path_buf());

    Ok(root)
}

/// Copy a directory tree into `dest`, replacing it if it already exists.
pub fn copy_dir(src: &Path, dest: &Path) -> Result<()> {
    if dest.exists() {
        fs::remove_dir_all(dest)
            .with_context(|| format!("Could not replace {}", dest.display()))?;
    }

    copy_recursive(src, dest)
}

fn copy_recursive(src: &Path, dest: &Path) -> Result<()> {
    fs::create_dir_all(dest).with_context(|| format!("Could not create {}", dest.display()))?;

    for entry in fs::read_dir(src).with_context(|| format!("Could not read {}", src.display()))? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dest.join(entry.file_name());

        if file_type.is_dir() {
            copy_recursive(&from, &to)?;
        } else if file_type.is_file() {
            fs::copy(&from, &to)
                .with_context(|| format!("Could not copy {} → {}", from.display(), to.display()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_owner_repo() {
        let spec = parse_github_spec("cipherunits/fusion-mod-jwt").unwrap();
        assert_eq!(spec.owner, "cipherunits");
        assert_eq!(spec.repo, "fusion-mod-jwt");
        assert!(spec.reference.is_none());
    }

    #[test]
    fn test_parse_with_ref() {
        let spec = parse_github_spec("acme/mod@v1.2.3").unwrap();
        assert_eq!(spec.reference.as_deref(), Some("v1.2.3"));
        assert_eq!(spec.source_label(), "github:acme/mod@v1.2.3");
    }

    #[test]
    fn test_parse_url() {
        let spec = parse_github_spec("https://github.com/acme/mod.git").unwrap();
        assert_eq!(spec.owner, "acme");
        assert_eq!(spec.repo, "mod");
    }

    #[test]
    fn test_parse_tree_url() {
        let spec = parse_github_spec("https://github.com/acme/mod/tree/develop").unwrap();
        assert_eq!(spec.reference.as_deref(), Some("develop"));
    }
}
