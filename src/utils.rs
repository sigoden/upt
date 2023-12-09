use std::process::{Command, Stdio};

pub fn which(name: &str) -> bool {
    Command::new(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

pub fn find_tool(tools: &[&str]) -> Option<String> {
    match tools.len() {
        0 => None,
        1 => {
            let tool = &tools[0];
            if which(tool) {
                Some(tool.to_string())
            } else {
                None
            }
        }
        _ => {
            let handles: Vec<_> = tools
                .iter()
                .map(|tool| {
                    let tool = tool.to_string();
                    std::thread::spawn(move || if which(&tool) { Some(tool) } else { None })
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

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "android")))]
pub fn detect_os() -> Option<String> {
    let release = std::fs::read_to_string("/etc/os-release").ok()?;
    let id = release.lines().find(|l| l.starts_with("ID="))?;
    let id = id[3..].trim_matches('"');
    Some(id.to_string())
}
