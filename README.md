# Rekonfig

A lightweight, extensible tool for automating and replicating Arch Linux system configurations. Rekonfig helps you maintain consistent environments across multiple machines by detecting and applying system settings, packages, and themes.

## Features

- 🔍 **System Detection**
  - Automatically detects current shell, desktop environment, and terminal
  - Identifies installed packages and categorizes them
  - Captures locale settings and system preferences

- 📦 **Package Management**
  - Manages both system (pacman) and AUR packages
  - Intelligent package categorization
  - Validates package existence before installation

- 🎨 **Theme Management**
  - KDE Plasma theme configuration
  - Kvantum theme support
  - Global theme application

- ⚙️ **System Configuration**
  - Shell configuration
  - Locale and timezone settings
  - Keyboard layout preferences

## Prerequisites

- Arch Linux or compatible distribution
- Rust toolchain (for building)
- Base development tools:
  ```bash
  sudo pacman -S base-devel git rust
  ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rekonfig
   cd rekonfig
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Install the binary (optional):
   ```bash
   sudo install -Dm755 target/release/rekonfig /usr/local/bin/rekonfig
   ```

## Usage

### Initialize Configuration

Detect current system settings and create a configuration file:

```bash
rekonfig init [--hard]
```

The `--hard` flag includes all installed packages, including dependencies.

### Apply Configuration

Apply settings from a configuration file:

```bash
rekonfig config.toml
```

Apply specific sections only:

```bash
rekonfig config.toml -s system    # Apply only system settings
rekonfig config.toml -s packages  # Install only packages
rekonfig config.toml -s themes    # Configure only themes
```

### Add Packages

Add a new package to the configuration:

```bash
rekonfig add-package pacman neovim development  # Add a pacman package
rekonfig add-package yay paru-bin system        # Add an AUR package
```

## Configuration File

Rekonfig uses TOML for configuration. Here's an example `config.toml`:

```toml
[system]
shell = "zsh"
desktop_environment = "plasma"
terminal = "konsole"
terminal_font = "Fira Code"
icons = "Papirus-Dark"
theme = "Breeze-Dark"
splash_screen = "Breeze"
login_screen = "SDDM"

[locale]
language = "en_US.UTF-8"
timezone = "UTC"
keyboard_layout = "us"

[packages]
system = ["base", "linux", "nano"]
development = ["git", "neovim", "docker"]
graphics = ["gimp", "inkscape"]

[themes]
kvantum = "Sweet"
global = "Breeze-Dark"

[widgets]
- "Event Calendar"
- "Notes"
```

## Project Structure

```
.
├── Cargo.toml              # Project manifest
├── src/
│   ├── main.rs            # Entry point and CLI
│   ├── commands/          # Command implementations
│   │   ├── init.rs        # System detection
│   │   ├── apply.rs       # Configuration application
│   │   └── add_package.rs # Package management
│   ├── config/            # Configuration structures
│   │   └── mod.rs         # TOML configuration
│   ├── system/            # System interaction
│   │   └── mod.rs         # System detection functions
│   └── utils/             # Utility functions
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Troubleshooting

### Common Issues

1. **Package Installation Fails**
   - Check internet connection
   - Verify package exists in repositories
   - Run `sudo pacman -Syu` to update system

2. **Theme Application Fails**
   - Ensure KDE/Kvantum is installed
   - Verify theme package names
   - Check theme file permissions

3. **System Detection Issues**
   - Verify required tools are installed
   - Check environment variables
   - Run with elevated privileges if needed

## Support

For issues and feature requests, please create an issue in the GitHub repository.
