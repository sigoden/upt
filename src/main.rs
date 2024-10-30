use std::path::Path;
use std::process::Command;
use std::{env, process};
use upt::{detect_os, detect_vendor, init_vendor, UptError, Vendor};

fn main() {
    match run() {
        Ok(c) => {
            process::exit(c);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
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
    let cmd_args = match create_cmd(&vendor, &args, &os) {
        Ok(v) => v,
        Err(UptError::DisplayHelp(t)) => {
            println!("{t}");
            return Ok(0);
        }
        Err(e) => return Err(e.into()),
    };
    if let Ok(v) = std::env::var("UPT_DRY_RUN") {
        if v == "true" || v == "1" {
            println!("{}", cmd_args.join(" "));
            return Ok(0);
        }
    }
    let cmd = &cmd_args[0];
    let cmd = match which::which(cmd) {
        Ok(v) => v,
        Err(_) => return Err(format!("Command '{cmd}' not found").into()),
    };
    let status = Command::new(cmd).args(&cmd_args[1..]).status()?;

    Ok(status.code().unwrap_or_default())
}

fn create_cmd(vendor: &Vendor, args: &[String], os: &str) -> Result<Vec<String>, UptError> {
    let tool = match std::env::var("UPT_TOOL") {
        Ok(v) => init_vendor(&v)?,
        Err(_) => detect_vendor(os)?,
    };
    let task = vendor.parse(args, tool.name())?;
    let cmd = tool.eval(&task)?;
    Ok(cmd)
}
