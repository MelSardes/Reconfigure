use crate::config::Config;
use crate::system::{detect_desktop_env, detect_shell, detect_terminal};
use anyhow::Result;
use log::{info, warn};
use std::process::Command;

pub fn run(hard: bool) -> Result<()> {
    info!("Detecting system configuration...");
    
    let mut config = Config::new();
    
    // Detect shell
    if let Some(shell) = detect_shell() {
        config.system.shell = shell;
    } else {
        warn!("Could not detect shell");
    }
    
    // Detect desktop environment
    if let Some(de) = detect_desktop_env() {
        config.system.desktop_environment = de;
    } else {
        warn!("Could not detect desktop environment");
    }
    
    // Detect terminal
    if let Some(terminal) = detect_terminal() {
        config.system.terminal = terminal;
    } else {
        warn!("Could not detect terminal");
    }
    
    // Detect installed packages
    let packages = if hard {
        get_all_packages()?
    } else {
        get_user_packages()?
    };
    
    // Categorize packages (this is a simple example)
    for package in packages {
        if package.contains("dev") || package.contains("build") {
            config.packages.development.push(package);
        } else if package.contains("gtk") || package.contains("qt") {
            config.packages.system.push(package);
        } else if package.contains("gimp") || package.contains("inkscape") || package.contains("blender") {
            config.packages.graphics.push(package);
        } else {
            config.packages.other.push(package);
        }
    }
    
    // Save configuration
    config.save_to_file("config.toml")?;
    info!("Configuration saved to config.toml");
    
    Ok(())
}

fn get_user_packages() -> Result<Vec<String>> {
    let output = Command::new("pacman")
        .args(["-Qe"])
        .output()?;
    
    let packages = String::from_utf8(output.stdout)?
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    Ok(packages)
}

fn get_all_packages() -> Result<Vec<String>> {
    let output = Command::new("pacman")
        .args(["-Q"])
        .output()?;
    
    let packages = String::from_utf8(output.stdout)?
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    Ok(packages)
}
