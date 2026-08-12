use crate::setting::module::{install_module, parse_github_spec};
use anyhow::{bail, Result};
use console::style;

pub fn add(github: Option<String>) -> Result<()> {
    let Some(spec_input) = github else {
        bail!("Pass a module with --github owner/repo (optionally owner/repo@ref)");
    };

    let spec = parse_github_spec(&spec_input)?;
    let result = install_module(&spec)?;

    println!();
    println!(
        "{}",
        style(format!("✔ Module '{}' installed!", result.id))
            .green()
            .bold()
    );
    println!();
    println!("  Location: {}", style(result.path.display()).yellow());
    println!();
    println!("Import it in your Fusion app:");
    println!("  {}", style(&result.import_hint).cyan());
    println!();

    Ok(())
}
