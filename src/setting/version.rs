use crate::setting::get;
use crate::setting::FUSION_TOOL_VERSION;
use anyhow::Result;
use console::style;
use std::path::Path;

pub fn check_version_on_system(project_root: &Path) -> Result<()> {
    let fw_version = get::version_from(project_root)?;

    println!();
    println!("{}", style("Fusion Framework Configuration").cyan().bold());
    println!();
    println!("  Framework version: {}", style(fw_version).yellow());
    println!(
        "  Tool version:      {}",
        style(FUSION_TOOL_VERSION).yellow()
    );
    println!();

    Ok(())
}
