use std::process::Command;

use anyhow::Context;

pub fn run_node_js(target_dir: String) -> anyhow::Result<()> {
    let target_dir = target_dir.replace("{target}", "node");

    Command::new("node")
        .current_dir(target_dir)
        .args(["index.js"])
        .status()
        .context("Failed to run node.js")?;
    Ok(())
}
