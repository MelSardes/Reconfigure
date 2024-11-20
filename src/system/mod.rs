use std::env;
use std::process::Command;

pub fn detect_shell() -> Option<String> {
    env::var("SHELL")
        .ok()
        .and_then(|s| s.split('/').last().map(String::from))
}

pub fn detect_desktop_env() -> Option<String> {
    if let Ok(desktop) = env::var("XDG_CURRENT_DESKTOP") {
        Some(desktop.to_lowercase())
    } else if let Ok(session) = env::var("DESKTOP_SESSION") {
        Some(session.to_lowercase())
    } else {
        // Try to detect by running processes
        let output = Command::new("ps")
            .args(["aux"])
            .output()
            .ok()?;
        
        let processes = String::from_utf8_lossy(&output.stdout).to_lowercase();
        
        if processes.contains("plasma") {
            Some("plasma".to_string())
        } else if processes.contains("gnome-shell") {
            Some("gnome".to_string())
        } else if processes.contains("xfce") {
            Some("xfce".to_string())
        } else {
            None
        }
    }
}

pub fn detect_terminal() -> Option<String> {
    // Try to get from environment variable
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return Some(term.to_lowercase());
    }
    
    // Try to get from process hierarchy
    let ppid = std::process::id();
    let output = Command::new("ps")
        .args(["-p", &ppid.to_string(), "-o", "comm="])
        .output()
        .ok()?;
    
    let terminal = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
    
    if terminal.contains("konsole") 
        || terminal.contains("gnome-terminal")
        || terminal.contains("xfce4-terminal")
        || terminal.contains("alacritty")
        || terminal.contains("kitty") {
        Some(terminal)
    } else {
        None
    }
}

pub fn get_kde_widgets() -> Option<Vec<String>> {
    // This is a placeholder - actual implementation would need to read KDE config files
    None
}

pub fn get_system_locale() -> Option<String> {
    Command::new("locale")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout).ok()
        })
        .and_then(|s| {
            s.lines()
                .find(|line| line.starts_with("LANG="))
                .map(|line| line.trim_start_matches("LANG=").to_string())
        })
}

pub fn get_system_timezone() -> Option<String> {
    std::fs::read_link("/etc/localtime")
        .ok()
        .and_then(|path| {
            path.to_str()
                .and_then(|s| s.split("/zoneinfo/").nth(1))
                .map(String::from)
        })
}

pub fn get_keyboard_layout() -> Option<String> {
    Command::new("setxkbmap")
        .arg("-query")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout).ok()
        })
        .and_then(|s| {
            s.lines()
                .find(|line| line.contains("layout:"))
                .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
        })
}
