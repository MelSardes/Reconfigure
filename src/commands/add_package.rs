use anyhow::{anyhow, Result};
use log::{error, info};
use std::process::Command;

pub fn run(package_manager: &str, package_name: &str, category: Option<&str>) -> Result<()> {
    // Validate package manager
    match package_manager {
        "pacman" | "yay" => (),
        _ => return Err(anyhow!("Unsupported package manager: {}", package_manager)),
    }

    // Validate package exists
    if !package_exists(package_manager, package_name)? {
        return Err(anyhow!("Package not found: {}", package_name));
    }

    // Load existing configuration
    let mut config = crate::config::Config::from_file("config.toml")
        .map_err(|_| anyhow!("Could not load config.toml"))?;

    // Determine category
    let category = category.unwrap_or("other");
    
    // Add package to appropriate category
    match category {
        "system" => config.packages.system.push(package_name.to_string()),
        "development" => config.packages.development.push(package_name.to_string()),
        "graphics" => config.packages.graphics.push(package_name.to_string()),
        _ => config.packages.other.push(package_name.to_string()),
    }

    // Save configuration
    config.save_to_file("config.toml")?;
    info!("Added package {} to category {}", package_name, category);

    Ok(())
}

fn package_exists(package_manager: &str, package_name: &str) -> Result<bool> {
    let output = Command::new(package_manager)
        .args(["-Ss", package_name])
        .output()?;

    if !output.status.success() {
        error!("Failed to search for package: {}", package_name);
        return Ok(false);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.lines().any(|line| {
        line.split_whitespace()
            .next()
            .map(|s| s.ends_with(package_name))
            .unwrap_or(false)
    }))
}
