use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{self, Command};
use upt::{UptError, Vendor};

fn main() {
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, remainer) = env_args.split_first().unwrap();
    let bin = Path::new(bin).file_stem().unwrap().to_str().unwrap();
    let vendor = exit_if_err(Vendor::lookup(bin));
    let cmd = match create_cmd(&vendor, remainer) {
        Ok(v) => v,
        Err(e) => {
            dump_upt_error(&vendor, e);
            process::exit(1);
        }
    };
    run_cmd(cmd.as_str())
}

fn create_cmd(vendor: &Vendor, args: &[String]) -> Result<String, UptError> {
    let task = vendor.parse(args)?;
    let vendor = Vendor::detect()?;
    let cmd = vendor.eval(&task)?;
    Ok(cmd)
}

#[cfg(not(target_os = "windows"))]
fn run_cmd(cmd: &str) {
    let mut child = exit_if_err(Command::new("sh").arg("-c").arg(cmd).spawn());
    exit_if_err(child.wait());
}

#[cfg(target_os = "windows")]
fn run_cmd(cmd: &str) {
    let mut child = exit_if_err(Command::new("cmd").args(["/C", cmd]).spawn());
    exit_if_err(child.wait());
}

fn exit_if_err<T, E: Error>(result: Result<T, E>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn dump_upt_error(vendor: &Vendor, err: UptError) {
    use UptError::*;
    match err {
        NotRecognize => {
            eprintln!("{}\n{}", err, vendor.help());
        }
        _ => eprintln!("{}", err),
    }
}
