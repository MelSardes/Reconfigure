#!/bin/bash

# Package Addition Script
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

# Print usage
usage() {
    echo "Usage: $0 <package-name> [section]"
    echo "Sections:"
    echo "  system.base        - Base system utilities"
    echo "  system.development - Development tools"
    echo "  system.desktop     - Desktop environment packages"
    echo "  aur.development    - AUR development tools"
    echo "  aur.themes        - AUR themes and customization"
    echo "  fonts             - Font packages"
    exit 1
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

# Add package to configuration
add_package() {
    local package="$1"
    local section="$2"
    local temp_file=$(mktemp)
    
    # Determine package type and validate
    case "$section" in
        system.*)
            if ! validate_pacman_package "$package"; then
                log "ERROR" "Package '$package' not found in pacman repositories"
                exit 1
            fi
            ;;
        aur.*)
            if ! validate_aur_package "$package"; then
                log "ERROR" "Package '$package' not found in AUR"
                exit 1
            fi
            ;;
        fonts)
            if ! validate_pacman_package "$package" && ! validate_aur_package "$package"; then
                log "ERROR" "Font package '$package' not found"
                exit 1
            fi
            ;;
        *)
            log "ERROR" "Invalid section: $section"
            usage
            ;;
    esac
    
    # Check if package already exists in the section
    if yq -e ".$section[] | select(. == \"$package\")" "$CONFIG_FILE" &>/dev/null; then
        log "WARNING" "Package '$package' already exists in section '$section'"
        exit 0
    fi
    
    # Add package to the configuration
    if ! yq -i ".${section} += [\"$package\"]" "$CONFIG_FILE"; then
        log "ERROR" "Failed to add package '$package' to section '$section'"
        exit 1
    fi
    log "INFO" "Added package '$package' to section '$section'"
}

# Main function
main() {
    if [ $# -lt 1 ] || [ $# -gt 2 ]; then
        usage
    fi
    
    local package="$1"
    local section="${2:-system.base}"  # Default to system.base if no section provided
    
    add_package "$package" "$section"
}

# Run main function
main "$@"
