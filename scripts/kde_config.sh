#!/bin/bash

# KDE Theme Configuration Script
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_FILE="$PROJECT_ROOT/reconfig.yml"
LOG_FILE="$PROJECT_ROOT/logs/setup.log"
THEME_DIR="$HOME/.local/share/plasma/desktoptheme"
KVANTUM_DIR="$HOME/.config/Kvantum"

# Logger function
log() {
    local level="$1"
    local message="$2"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$LOG_FILE"
}

# Download and install global theme
install_global_theme() {
    local name="$1"
    local url="$2"
    
    log "INFO" "Installing global theme: $name"
    
    # Create temporary directory for theme download
    local temp_dir=$(mktemp -d)
    
    # Download theme
    if ! curl -L "$url" -o "$temp_dir/$name.tar.gz"; then
        log "ERROR" "Failed to download theme: $name"
        rm -rf "$temp_dir"
        return 1
    fi
    
    # Extract theme
    mkdir -p "$THEME_DIR"
    if ! tar -xzf "$temp_dir/$name.tar.gz" -C "$THEME_DIR"; then
        log "ERROR" "Failed to extract theme: $name"
        rm -rf "$temp_dir"
        return 1
    }
    
    # Apply theme
    if ! lookandfeeltool -a "$name"; then
        log "WARNING" "Failed to apply global theme: $name"
    fi
    
    rm -rf "$temp_dir"
}

# Install and configure Kvantum theme
install_kvantum_theme() {
    local name="$1"
    local url="$2"
    
    log "INFO" "Installing Kvantum theme: $name"
    
    # Create temporary directory for theme download
    local temp_dir=$(mktemp -d)
    
    # Download theme
    if ! curl -L "$url" -o "$temp_dir/$name.tar.gz"; then
        log "ERROR" "Failed to download Kvantum theme: $name"
        rm -rf "$temp_dir"
        return 1
    }
    
    # Extract theme
    mkdir -p "$KVANTUM_DIR"
    if ! tar -xzf "$temp_dir/$name.tar.gz" -C "$KVANTUM_DIR"; then
        log "ERROR" "Failed to extract Kvantum theme: $name"
        rm -rf "$temp_dir"
        return 1
    }
    
    # Apply theme
    if command -v kvantummanager &> /dev/null; then
        kvantummanager --set "$name"
    else
        log "WARNING" "kvantummanager not found, skipping theme application"
    fi
    
    rm -rf "$temp_dir"
}

# Main function
main() {
    # Install global themes
    while IFS= read -r theme; do
        name=$(echo "$theme" | yq '.name')
        url=$(echo "$theme" | yq '.url')
        install_global_theme "$name" "$url"
    done < <(yq -o=json '.themes.global[]' "$CONFIG_FILE")
    
    # Install Kvantum themes
    while IFS= read -r theme; do
        name=$(echo "$theme" | yq '.name')
        url=$(echo "$theme" | yq '.url')
        install_kvantum_theme "$name" "$url"
    done < <(yq -o=json '.themes.kvantum[]' "$CONFIG_FILE")
    
    log "INFO" "Theme configuration completed"
}

# Run main function
main "$@"
