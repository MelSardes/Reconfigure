#!/bin/bash

# Package Installation Script
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_FILE="$PROJECT_ROOT/reconfig.yml"
LOG_FILE="$PROJECT_ROOT/logs/setup.log"

# Logger function
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

# Validate package existence in pacman repositories
validate_pacman_package() {
    local package="$1"
    if ! pacman -Si "$package" &>/dev/null; then
        return 1
    fi
    return 0
}

# Validate package existence in AUR
validate_aur_package() {
    local package="$1"
    if ! yay -Si "$package" &>/dev/null; then
        return 1
    fi
    return 0
}

# Install system packages
install_system_packages() {
    log "INFO" "Installing system packages..."
    
    local packages=()
    while IFS= read -r package; do
        if validate_pacman_package "$package"; then
            packages+=("$package")
        else
            log "WARNING" "Invalid package: $package"
        fi
    done < <(yq -o=json '.system.**.[]' "$CONFIG_FILE")

    if [ ${#packages[@]} -gt 0 ]; then
        log "INFO" "Installing ${#packages[@]} system packages..."
        pacman -Syu --needed --noconfirm "${packages[@]}" || {
            log "ERROR" "Failed to install system packages"
            return 1
        }
    fi
}

# Install AUR packages
install_aur_packages() {
    log "INFO" "Installing AUR packages..."
    
    local packages=()
    while IFS= read -r package; do
        if validate_aur_package "$package"; then
            packages+=("$package")
        else
            log "WARNING" "Invalid AUR package: $package"
        fi
    done < <(yq -o=json '.aur.**.[]' "$CONFIG_FILE")

    if [ ${#packages[@]} -gt 0 ]; then
        log "INFO" "Installing ${#packages[@]} AUR packages..."
        yay -S --needed --noconfirm "${packages[@]}" || {
            log "ERROR" "Failed to install AUR packages"
            return 1
        }
    fi
}

# Install fonts
install_fonts() {
    log "INFO" "Installing fonts..."
    
    local fonts=()
    while IFS= read -r font; do
        if validate_pacman_package "$font" || validate_aur_package "$font"; then
            fonts+=("$font")
        else
            log "WARNING" "Invalid font package: $font"
        fi
    done < <(yq -o=json '.fonts[]' "$CONFIG_FILE")

    if [ ${#fonts[@]} -gt 0 ]; then
        log "INFO" "Installing ${#fonts[@]} font packages..."
        yay -S --needed --noconfirm "${fonts[@]}" || {
            log "ERROR" "Failed to install font packages"
            return 1
        }
    fi
}

# Main function
main() {
    # Update package databases
    log "INFO" "Updating package databases..."
    pacman -Sy || {
        log "ERROR" "Failed to update package databases"
        exit 1
    }

    # Install packages
    install_system_packages
    install_aur_packages
    install_fonts

    log "INFO" "Package installation completed"
}

# Run main function
main "$@"
