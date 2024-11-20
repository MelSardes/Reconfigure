use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub system: SystemConfig,
    pub locale: LocaleConfig,
    pub packages: PackageConfig,
    pub themes: ThemeConfig,
    #[serde(default)]
    pub widgets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    pub shell: String,
    pub desktop_environment: String,
    pub terminal: String,
    pub terminal_font: String,
    pub icons: String,
    pub theme: String,
    pub splash_screen: String,
    pub login_screen: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleConfig {
    pub language: String,
    pub timezone: String,
    pub keyboard_layout: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageConfig {
    #[serde(default)]
    pub system: Vec<String>,
    #[serde(default)]
    pub development: Vec<String>,
    #[serde(default)]
    pub graphics: Vec<String>,
    #[serde(default)]
    pub other: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub kvantum: Option<String>,
    pub global: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            system: SystemConfig {
                shell: String::new(),
                desktop_environment: String::new(),
                terminal: String::new(),
                terminal_font: String::new(),
                icons: String::new(),
                theme: String::new(),
                splash_screen: String::new(),
                login_screen: String::new(),
            },
            locale: LocaleConfig {
                language: String::from("en_US.UTF-8"),
                timezone: String::from("UTC"),
                keyboard_layout: String::from("us"),
            },
            packages: PackageConfig {
                system: Vec::new(),
                development: Vec::new(),
                graphics: Vec::new(),
                other: Vec::new(),
            },
            themes: ThemeConfig {
                kvantum: None,
                global: String::from("Breeze"),
            },
            widgets: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
