# Arch Linux Reconfiguration Manager

A modular and extensible solution for automating the configuration of Arch Linux systems. This tool helps you maintain a consistent, reproducible system setup through a centralized YAML configuration file.

## Features

- 📦 **Package Management**
  - System package installation via `pacman`
  - AUR package installation via `yay`
  - Package validation before installation
  - Easy package addition with category management

- 🎨 **Theme Management**
  - KDE Global theme installation and configuration
  - Kvantum theme support
  - Automated theme downloads and application

- 🛠️ **System Configuration**
  - Centralized configuration through `reconfig.yml`
  - Font package installation
  - Custom script execution support
  - Comprehensive logging


## Prerequisites

- Arch Linux
- `sudo` access
- Basic dependencies:
  ```bash
  sudo pacman -S yq curl base-devel git
  ```

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/arch-reconfig-manager
   cd arch-reconfig-manager
   ```

2. Make scripts executable:
   ```bash
   chmod +x scripts/*.sh
   ```

## Configuration

The system is configured through `reconfig.yml`. Here's an example configuration:

```yaml
# System packages (installed via pacman)
system:
  base:
    - base-devel
    - git
    - vim
    - yq
  
  development:
    - docker
    - docker-compose
    - visual-studio-code-bin

# AUR packages
aur:
  development:
    - postman-bin
  
  themes:
    - kvantum-theme-materia

# KDE Themes
themes:
  global:
    - name: "Materia Dark"
      url: "https://store.kde.org/p/1229134"
  
  kvantum:
    - name: "MateriaDark"
      url: "https://store.kde.org/p/1229134"

# Font packages
fonts:
  - noto-fonts
  - ttf-dejavu
  - ttf-liberation
```

## Usage

### Full System Setup

To apply your entire configuration:

```bash
sudo ./scripts/setup.sh
```

This will:
1. Install all configured packages
2. Apply KDE themes (if configured)
3. Install fonts
4. Execute any custom scripts

### Adding Packages

To add a new package:

```bash
./scripts/add_package.sh <package-name> [section]
```

Available sections:
- `system.base` - Base system utilities
- `system.development` - Development tools
- `system.desktop` - Desktop environment packages
- `aur.development` - AUR development tools
- `aur.themes` - AUR themes and customization
- `fonts` - Font packages

Example:
```bash
./scripts/add_package.sh neovim system.development
```

## Directory Structure

```
.
├── reconfig.yml        # Main configuration file
├── scripts/
│   ├── setup.sh        # Main setup script
│   ├── add_package.sh  # Package addition utility
│   ├── kde_config.sh   # KDE theme configuration
│   └── install_packages.sh  # Package installation
└── logs/
    └── setup.log       # Setup and error logs
```

## Error Handling

- All operations are logged to `logs/setup.log`
- Failed operations are clearly marked in the log
- The system will continue with the next item on failure when possible
- Critical failures will stop the process and provide clear error messages

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Troubleshooting

### Common Issues

1. **Package Not Found**
   - Verify the package name exists in pacman or AUR
   - Check your internet connection
   - Update package databases: `sudo pacman -Sy`

2. **Theme Installation Fails**
   - Verify the theme URL is accessible
   - Ensure KDE/Kvantum is properly installed
   - Check the theme format is correct

3. **Permission Issues**
   - Ensure scripts are executable
   - Run setup.sh with sudo
   - Check file permissions in ~/.local

### Logs

Check `logs/setup.log` for detailed error messages and operation history.

## Support

For issues and feature requests, please create an issue in the GitHub repository.
