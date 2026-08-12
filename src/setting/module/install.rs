use super::github::{copy_dir, download_repo, GitHubSpec};
use super::manifest::{
    load_manifest, supports_host_language, ModuleImplLanguage, ModuleManifest, MANIFEST_FILE,
};
use crate::setting::config::{Config, InstalledModule};
use crate::setting::get;
use anyhow::{bail, Context, Result};
use console::style;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const VENDOR_ROOT: &str = ".fusion/modules";

pub struct InstallResult {
    pub id: String,
    pub path: PathBuf,
    pub import_hint: String,
}

/// Download, vendor, build/install a GitHub module into the current project.
pub fn install_module(spec: &GitHubSpec) -> Result<InstallResult> {
    let project_root = get::get_project_root();
    let config_path = get::config_path_from(&project_root);

    if !config_path.exists() {
        bail!(
            "No fusion-framework.toml in {}. Run this from a Fusion project.",
            project_root.display()
        );
    }

    let host_language = get::language_from(&project_root)?;

    if host_language != "python" && host_language != "typescript" {
        bail!(
            "Host language '{}' cannot install Fusion modules yet. Use a Python or TypeScript project.",
            host_language
        );
    }

    println!();
    println!(
        "{}",
        style(format!("Fetching {}...", spec.source_label()))
            .cyan()
            .bold()
    );
    println!();

    let staging = std::env::temp_dir().join(format!(
        "fusion-add-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    ));

    fs::create_dir_all(&staging)?;

    let extracted = match download_repo(spec, &staging) {
        Ok(path) => path,
        Err(error) => {
            let _ = fs::remove_dir_all(&staging);
            return Err(error);
        }
    };

    let manifest_path = find_manifest(&extracted)?;
    let module_root = manifest_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid module layout"))?
        .to_path_buf();

    let manifest = load_manifest(&manifest_path)?;

    if !supports_host_language(&manifest, &host_language) {
        let _ = fs::remove_dir_all(&staging);
        bail!(
            "Module '{}' does not target host language '{}'.",
            manifest.module.id,
            host_language
        );
    }

    let id = manifest.module.id.clone();
    let vendor_rel = format!("{VENDOR_ROOT}/{id}");
    let vendor_abs = project_root.join(&vendor_rel);

    fs::create_dir_all(vendor_abs.parent().unwrap())?;
    copy_dir(&module_root, &vendor_abs)?;
    let _ = fs::remove_dir_all(&staging);

    println!(
        "{}",
        style(format!("✔ Vendored into {}", vendor_rel))
            .green()
            .bold()
    );

    run_builds(&vendor_abs, &manifest, &host_language)?;

    if host_language == "typescript" {
        link_typescript_dependency(&project_root, &vendor_rel, &manifest)?;
    }

    upsert_project_module(
        &project_root,
        InstalledModule {
            id: id.clone(),
            source: spec.source_label(),
            version: manifest.module.version.clone(),
            path: vendor_rel,
        },
    )?;

    Ok(InstallResult {
        import_hint: import_hint(&manifest, &host_language),
        id,
        path: vendor_abs,
    })
}

fn import_hint(manifest: &ModuleManifest, host_language: &str) -> String {
    match host_language {
        "python" => {
            let package = manifest
                .module
                .entry
                .python
                .clone()
                .unwrap_or_else(|| format!("fusion_{}_mod", manifest.module.id.replace('-', "_")));
            format!("from {package} import hello")
        }
        "typescript" => {
            let package = manifest
                .module
                .entry
                .typescript
                .clone()
                .unwrap_or_else(|| format!("fusion-{}-mod", manifest.module.id));
            format!("import {{ hello }} from \"{package}\"")
        }
        _ => String::new(),
    }
}

fn find_manifest(root: &Path) -> Result<PathBuf> {
    let direct = root.join(MANIFEST_FILE);
    if direct.is_file() {
        return Ok(direct);
    }

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path().join(MANIFEST_FILE);
            if path.is_file() {
                return Ok(path);
            }
        }
    }

    bail!(
        "No {} found in the repository. Run `fusion module init` first.",
        MANIFEST_FILE
    )
}

fn run_builds(module_root: &Path, manifest: &ModuleManifest, host_language: &str) -> Result<()> {
    let commands: Vec<String> = match (manifest.module.impl_.language, host_language) {
        (ModuleImplLanguage::Python, "python") => {
            manifest.module.build.python.clone().into_iter().collect()
        }
        (ModuleImplLanguage::TypeScript, "typescript") => manifest
            .module
            .build
            .typescript
            .clone()
            .into_iter()
            .collect(),
        (ModuleImplLanguage::Rust, "python") => {
            let mut cmds = Vec::new();
            if let Some(rust) = &manifest.module.build.rust {
                cmds.push(rust.clone());
            }
            if let Some(python) = &manifest.module.build.python {
                cmds.push(python.clone());
            }
            cmds
        }
        (ModuleImplLanguage::Rust, "typescript") => {
            let mut cmds = Vec::new();
            if let Some(rust) = &manifest.module.build.rust {
                cmds.push(rust.clone());
            }
            if let Some(typescript) = &manifest.module.build.typescript {
                cmds.push(typescript.clone());
            }
            cmds
        }
        _ => Vec::new(),
    };

    for command in commands {
        println!();
        println!("{}", style(format!("$ {command}")).yellow());

        let status = shell_command(&command, module_root)
            .with_context(|| format!("Failed to run build command: {command}"))?;

        if !status.success() {
            bail!("Build command failed: {command}");
        }
    }

    Ok(())
}

fn shell_command(command: &str, cwd: &Path) -> Result<std::process::ExitStatus> {
    #[cfg(windows)]
    {
        Command::new("cmd")
            .args(["/C", command])
            .current_dir(cwd)
            .status()
            .context("Could not spawn shell for module build")
    }

    #[cfg(not(windows))]
    {
        Command::new("sh")
            .args(["-c", command])
            .current_dir(cwd)
            .status()
            .context("Could not spawn shell for module build")
    }
}

/// Add `"fusion-mod-x": "file:.fusion/modules/x"` to the host package.json.
fn link_typescript_dependency(
    project_root: &Path,
    vendor_rel: &str,
    manifest: &ModuleManifest,
) -> Result<()> {
    let package_name = manifest
        .module
        .entry
        .typescript
        .clone()
        .unwrap_or_else(|| format!("fusion-mod-{}", manifest.module.id));

    let package_json = project_root.join("package.json");

    let mut root: Value = if package_json.exists() {
        serde_json::from_str(&fs::read_to_string(&package_json)?)
            .context("Could not parse package.json")?
    } else {
        serde_json::json!({
            "name": "fusion-app",
            "version": "0.0.0",
            "private": true,
        })
    };

    let deps = root
        .as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("package.json must be an object"))?
        .entry("dependencies")
        .or_insert_with(|| serde_json::json!({}));

    deps.as_object_mut()
        .ok_or_else(|| anyhow::anyhow!("package.json dependencies must be an object"))?
        .insert(
            package_name.clone(),
            Value::String(format!("file:{vendor_rel}")),
        );

    fs::write(
        &package_json,
        format!("{}\n", serde_json::to_string_pretty(&root)?),
    )?;

    println!(
        "{}",
        style(format!(
            "✔ Linked {package_name} → file:{vendor_rel} in package.json"
        ))
        .green()
        .bold()
    );

    let status = shell_command("npm install", project_root)?;
    if !status.success() {
        bail!("npm install failed after linking the module");
    }

    Ok(())
}

fn upsert_project_module(project_root: &Path, module: InstalledModule) -> Result<()> {
    let path = get::config_path_from(project_root);
    let content = fs::read_to_string(&path)?;
    let mut config: Config =
        toml::from_str(&content).context("Could not parse fusion-framework.toml")?;

    if let Some(existing) = config.modules.iter_mut().find(|item| item.id == module.id) {
        *existing = module;
    } else {
        config.modules.push(module);
    }

    let serialized =
        toml::to_string_pretty(&config).context("Could not serialize fusion-framework.toml")?;

    fs::write(&path, format!("{}\n", serialized))?;

    println!(
        "{}",
        style(format!("✔ Updated {}", path.display()))
            .green()
            .bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setting::config::{FrameworkConfig, ProjectConfig, ToolConfig};
    use crate::setting::module::scaffold::{scaffold, ModuleInitOptions};
    use crate::setting::{FUSION_FRAMEWORK_VERSION, FUSION_TOOL_VERSION};

    fn temp_project() -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fusion-install-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        fs::create_dir_all(dir.join("src/modules")).unwrap();

        let config = Config {
            project: ProjectConfig {
                name: "demo".into(),
                description: "demo".into(),
            },
            fusionframework: FrameworkConfig {
                language: "python".into(),
                extension: ".py".into(),
                version: FUSION_FRAMEWORK_VERSION.into(),
            },
            tool: ToolConfig {
                version: FUSION_TOOL_VERSION.into(),
            },
            modules: vec![],
        };

        fs::write(
            dir.join("fusion-framework.toml"),
            toml::to_string_pretty(&config).unwrap(),
        )
        .unwrap();

        fs::write(
            dir.join("main.py"),
            "from fusion_framework.app import run\n\nif __name__ == \"__main__\":\n    run(\"settings\")\n",
        )
        .unwrap();

        dir
    }

    #[test]
    fn test_upsert_records_module_without_touching_main() {
        let project = temp_project();
        let module_dir = project.join(".fusion/modules/example");

        scaffold(
            &module_dir,
            &ModuleInitOptions {
                name: "example".into(),
                description: "example".into(),
                language: ModuleImplLanguage::Python,
                target_python: true,
                target_typescript: false,
            },
        )
        .unwrap();

        upsert_project_module(
            &project,
            InstalledModule {
                id: "example".into(),
                source: "github:acme/example".into(),
                version: "0.1.0".into(),
                path: ".fusion/modules/example".into(),
            },
        )
        .unwrap();

        let toml = fs::read_to_string(project.join("fusion-framework.toml")).unwrap();
        assert!(toml.contains("id = \"example\""));

        let main = fs::read_to_string(project.join("main.py")).unwrap();
        assert!(!main.contains("fusion:modules"));
        assert!(!main.contains("_fusion_registry"));

        let _ = fs::remove_dir_all(&project);
    }

    #[test]
    fn test_import_hint_python() {
        let dir = std::env::temp_dir().join(format!(
            "fusion-hint-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));

        scaffold(
            &dir,
            &ModuleInitOptions {
                name: "example".into(),
                description: "example".into(),
                language: ModuleImplLanguage::Python,
                target_python: true,
                target_typescript: false,
            },
        )
        .unwrap();

        let manifest = load_manifest(&dir.join(MANIFEST_FILE)).unwrap();
        assert_eq!(
            import_hint(&manifest, "python"),
            "from fusion_example_mod import hello"
        );

        let _ = fs::remove_dir_all(&dir);
    }
}
