use anyhow::{anyhow, Result};
use log::{error, info};
use std::path::Path;
use std::process::Command;

pub fn run(config_file: &Path, section: Option<&str>) -> Result<()> {
    let config = crate::config::Config::from_file(config_file.to_str().unwrap())?;

    match section {
        Some("system") => apply_system_config(&config)?,
        Some("packages") => apply_packages(&config)?,
        Some("themes") => apply_themes(&config)?,
        Some(s) => return Err(anyhow!("Unknown section: {}", s)),
        None => {
            apply_system_config(&config)?;
            apply_packages(&config)?;
            apply_themes(&config)?;
        }
    }

    Ok(())
}

fn apply_system_config(config: &crate::config::Config) -> Result<()> {
    info!("Applying system configuration...");

    // Set shell if different
    if !config.system.shell.is_empty() {
        let current_shell = std::env::var("SHELL").unwrap_or_default();
        if !current_shell.ends_with(&config.system.shell) {
            Command::new("chsh")
                .args(["-s", &format!("/bin/{}", config.system.shell)])
                .status()?;
        }
    }

    // Set locale
    if !config.locale.language.is_empty() {
        // Uncomment locale in /etc/locale.gen
        let locale_gen = std::fs::read_to_string("/etc/locale.gen")?;
        let new_locale_gen = locale_gen
            .lines()
            .map(|line| {
                if line.contains(&config.locale.language) && line.starts_with('#') {
                    &line[1..]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write("/etc/locale.gen", new_locale_gen)?;

        // Generate locale
        Command::new("locale-gen").status()?;
    }

    // Set timezone
    if !config.locale.timezone.is_empty() {
        Command::new("timedatectl")
            .args(["set-timezone", &config.locale.timezone])
            .status()?;
    }

    // Set keyboard layout
    if !config.locale.keyboard_layout.is_empty() {
        Command::new("setxkbmap")
            .arg(&config.locale.keyboard_layout)
            .status()?;
    }

    Ok(())
}

fn apply_packages(config: &crate::config::Config) -> Result<()> {
    info!("Installing packages...");

    // Collect all packages
    let mut system_packages = Vec::new();
    let mut aur_packages = Vec::new();

    // Helper function to check if a package is from AUR
    let is_aur_package = |pkg: &str| -> bool {
        let output = Command::new("pacman")
            .args(["-Si", pkg])
            .output()
            .map_err(|_| false)
            .unwrap_or(false);
        !output
    };

    // Categorize packages
    for pkg in &config.packages.system {
        if is_aur_package(pkg) {
            aur_packages.push(pkg);
        } else {
            system_packages.push(pkg);
        }
    }
    for pkg in &config.packages.development {
        if is_aur_package(pkg) {
            aur_packages.push(pkg);
        } else {
            system_packages.push(pkg);
        }
    }
    for pkg in &config.packages.graphics {
        if is_aur_package(pkg) {
            aur_packages.push(pkg);
        } else {
            system_packages.push(pkg);
        }
    }
    for pkg in &config.packages.other {
        if is_aur_package(pkg) {
            aur_packages.push(pkg);
        } else {
            system_packages.push(pkg);
        }
    }

    // Install system packages
    if !system_packages.is_empty() {
        info!("Installing system packages...");
        let status = Command::new("pacman")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm")
            .args(system_packages)
            .status()?;

        if !status.success() {
            error!("Failed to install some system packages");
        }
    }

    // Install AUR packages
    if !aur_packages.is_empty() {
        info!("Installing AUR packages...");
        let status = Command::new("yay")
            .arg("-S")
            .arg("--needed")
            .arg("--noconfirm")
            .args(aur_packages)
            .status()?;

        if !status.success() {
            error!("Failed to install some AUR packages");
        }
    }

    Ok(())
}

fn apply_themes(config: &crate::config::Config) -> Result<()> {
    info!("Applying themes...");

    // Apply KDE global theme
    if !config.themes.global.is_empty() {
        Command::new("lookandfeeltool")
            .args(["-a", &config.themes.global])
            .status()?;
    }

    // Apply Kvantum theme
    if let Some(kvantum) = &config.themes.kvantum {
        Command::new("kvantummanager")
            .args(["--set", kvantum])
            .status()?;
    }

    Ok(())
}
