use std::path::Path;
use std::{env, process};
use upt::{detect_os, detect_vendor, init_vendor, run_command, UptError, Vendor};

fn main() {
    match run() {
        Ok(c) => {
            process::exit(c);
        }
        Err(e) => {
            eprintln!("{}", e);
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
    let cmd = match create_cmd(&vendor, &args, &os) {
        Ok(v) => v,
        Err(UptError::DisplayHelp(t)) => {
            println!("{t}");
            return Ok(0);
        }
        Err(e) => return Err(e.into()),
    };
    if let Ok(v) = std::env::var("UPT_DRY_RUN") {
        if v == "true" || v == "1" {
            println!("{}", cmd);
            return Ok(0);
        }
    }
    run_command(cmd.as_str(), &os)
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
