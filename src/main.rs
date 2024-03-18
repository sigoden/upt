use std::env;
use std::path::Path;
use std::process::{self, Command};
use upt::{detect_vendor, init_vendor, UptError, Vendor};

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
    let cmd = create_cmd(&vendor, &args)?;
    if let Ok(v) = std::env::var("UPT_DRY_RUN") {
        if v == "true" || v == "1" {
            println!("{}", cmd);
            return Ok(());
        }
    }
    run_cmd(cmd.as_str())
}

fn create_cmd(vendor: &Vendor, args: &[String]) -> Result<String, UptError> {
    let tool = match std::env::var("UPT_TOOL") {
        Ok(v) => init_vendor(&v)?,
        Err(_) => detect_vendor()?,
    };
    let task = vendor.parse(args, tool.name())?;
    let cmd = tool.eval(&task)?;
    Ok(cmd)
}

#[cfg(not(target_os = "windows"))]
fn run_cmd(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("sh").arg("-c").arg(cmd).spawn()?;
    child.wait()?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_cmd(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new("cmd").args(["/C", cmd]).spawn()?;
    child.wait()?;
    Ok(())
}
