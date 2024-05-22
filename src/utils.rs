use std::process::{Command, Output};

use which::which;

pub fn find_tool(pairs: &[(&str, &str)]) -> Option<String> {
    match pairs.len() {
        0 => None,
        1 => {
            let (tool, bin_name) = &pairs[0];
            if which(bin_name).is_ok() {
                Some(tool.to_string())
            } else {
                None
            }
        }
        _ => {
            let handles: Vec<_> = pairs
                .iter()
                .map(|(tool, bin_name)| {
                    let tool = tool.to_string();
                    let bin_name = bin_name.to_string();
                    std::thread::spawn(move || {
                        if which(&bin_name).is_ok() {
                            Some(tool)
                        } else {
                            None
                        }
                    })
                })
                .collect();
            for handle in handles {
                if let Ok(Some(tool)) = handle.join() {
                    return Some(tool);
                }
            }
            None
        }
    }
}

#[cfg(target_os = "windows")]
pub fn detect_os() -> Option<String> {
    if std::env::var("MSYSTEM").is_ok() {
        let os = "windows/msys2";
        if let Ok(output) = run_command_with_output("which pacman", os) {
            if output.status.success() {
                return Some(os.to_string());
            }
        }
    }
    Some("windows".to_string())
}

#[cfg(target_os = "macos")]
pub fn detect_os() -> Option<String> {
    Some("macos".to_string())
}

#[cfg(target_os = "android")]
pub fn detect_os() -> Option<String> {
    Some("android".to_string())
}

#[cfg(target_os = "haiku")]
pub fn detect_os() -> Option<String> {
    Some("haiku".to_string())
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "android",
    target_os = "haiku"
)))]
pub fn detect_os() -> Option<String> {
    let release = std::fs::read_to_string("/etc/os-release").ok()?;
    let id = release.lines().find(|l| l.starts_with("ID="))?;
    let id = id[3..].trim_matches('"');
    Some(id.to_string())
}

pub fn run_command(cmd: &str, os: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let exit_status = if os == "windows" {
        Command::new("cmd").args(["/C", cmd]).status()?
    } else {
        Command::new("sh").arg("-c").arg(cmd).status()?
    };
    Ok(exit_status.code().unwrap_or_default())
}

pub fn run_command_with_output(cmd: &str, os: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let output = if os == "windows" {
        Command::new("cmd").args(["/C", cmd]).output()?
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()?
    };
    Ok(output)
}
