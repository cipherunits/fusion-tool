use crate::setting::get;
use crate::setting::FUSION_TOOL_VERSION;
use console::style;

pub fn check_version_on_system() {
    let fw_version = get::get_version();

    println!();
    println!("{}", style("Fusion Framework Configuration").cyan().bold());
    println!();
    println!("  Framework version: {}", style(fw_version).yellow());
    println!(
        "  Tool version:      {}",
        style(FUSION_TOOL_VERSION).yellow()
    );
    println!();
}
