#!/bin/bash

# Arch Linux Reconfiguration Manager - Main Setup Script
set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_FILE="$PROJECT_ROOT/reconfig.yml"
LOG_FILE="$PROJECT_ROOT/logs/setup.log"

# Ensure log directory exists
mkdir -p "$(dirname "$LOG_FILE")"

# Logger function
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

# Check for root privileges
if [[ $EUID -ne 0 ]]; then
    log "ERROR" "This script must be run as root"
    exit 1
fi

# Check for required dependencies
check_dependencies() {
    local deps=("yq" "pacman" "curl")
    local missing_deps=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing_deps+=("$dep")
        fi
    done

    if [ ${#missing_deps[@]} -ne 0 ]; then
        log "ERROR" "Missing required dependencies: ${missing_deps[*]}"
        log "INFO" "Please install missing dependencies and try again"
        exit 1
    fi
}

# Install AUR helper if not present
install_aur_helper() {
    if ! command -v yay &> /dev/null; then
        log "INFO" "Installing yay AUR helper..."
        local temp_dir=$(mktemp -d)
        git clone https://aur.archlinux.org/yay.git "$temp_dir"
        (cd "$temp_dir" && makepkg -si --noconfirm)
        rm -rf "$temp_dir"
    fi
}

# Main setup process
main() {
    log "INFO" "Starting Arch Linux Reconfiguration Manager setup"

    # Check dependencies
    check_dependencies

    # Install AUR helper
    install_aur_helper

    # Run package installation script
    log "INFO" "Installing packages..."
    bash "$SCRIPT_DIR/install_packages.sh"

    # Configure KDE themes
    if [[ -n "$(yq '.themes' "$CONFIG_FILE")" ]]; then
        log "INFO" "Configuring KDE themes..."
        bash "$SCRIPT_DIR/kde_config.sh"
    fi

    # Run custom scripts
    if [[ -n "$(yq '.scripts' "$CONFIG_FILE")" ]]; then
        log "INFO" "Running custom scripts..."
        while IFS= read -r script; do
            local script_path=$(echo "$script" | yq '.path')
            local description=$(echo "$script" | yq '.description')
            
            if [[ -f "$PROJECT_ROOT/$script_path" ]]; then
                log "INFO" "Running: $description"
                bash "$PROJECT_ROOT/$script_path"
            else
                log "WARNING" "Script not found: $script_path"
            fi
        done < <(yq -o=json '.scripts[]' "$CONFIG_FILE")
    fi

    log "INFO" "Setup completed successfully"
}

# Run main function
main "$@"
