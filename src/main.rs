use std::env;
use std::path::Path;
use std::process::{self, Command};
use upt::{detect_os, detect_vendor, init_vendor, UptError, Vendor};

fn main() {
    match run() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let env_args = env::args().collect::<Vec<String>>();
    let bin = Path::new(&env_args[0])
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let vendor = init_vendor(bin)?;
    let mut args = vec![bin.to_string()];
    args.extend(env_args.iter().skip(1).cloned());
    let os = detect_os().unwrap_or_default();
    let cmd = create_cmd(&vendor, &args, &os)?;
    if let Ok(v) = std::env::var("UPT_DRY_RUN") {
        if v == "true" || v == "1" {
            println!("{}", cmd);
            return Ok(());
        }
    }
    run_cmd(cmd.as_str(), &os)
}

fn create_cmd(vendor: &Vendor, args: &[String], os: &str) -> Result<String, UptError> {
    let tool = match std::env::var("UPT_TOOL") {
        Ok(v) => init_vendor(&v)?,
        Err(_) => detect_vendor(os)?,
    };
    let task = vendor.parse(args, tool.name())?;
    let cmd = tool.eval(&task)?;
    Ok(cmd)
}

#[cfg(not(target_os = "windows"))]
fn run_cmd(cmd: &str, _os: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("sh").arg("-c").arg(cmd).spawn()?;
    child.wait()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_cmd(cmd: &str, os: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = if os == "windows/msys2" {
        Command::new("sh").arg("-c").arg(cmd).spawn()?
    } else {
        Command::new("cmd").args(["/C", cmd]).spawn()?
    };
    child.wait()?;
    Ok(())
}
