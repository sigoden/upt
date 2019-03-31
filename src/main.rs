use std::env;
use std::error::Error;
use std::path::Path;
use std::process::{self, Command};
use upt::{UptError, Vendor, detect_os_vendor};

fn main() {
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, remind_args) = env_args.split_first().unwrap();;
    let bin = Path::new(bin).file_stem().unwrap().to_str().unwrap();
    let vendor = exit_if_err(Vendor::lookup(bin));
    let cmd = match create_cmd(&vendor, remind_args) {
        Ok(v) => v,
        Err(e) => {
            dump_upt_error(&vendor, e);
            process::exit(1);
        }
    };
    let mut child = exit_if_err(Command::new("sh").arg("-c").arg(cmd).spawn());
    exit_if_err(child.wait());
}

fn create_cmd(vendor: &Vendor, args: &[String]) -> Result<String, UptError> {
    let task = vendor.parse(args)?;
    let vendor = detect_os_vendor()?;
    let cmd = vendor.eval(&task)?;
    Ok(cmd)
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
        NoSubcommand | NotRecongize => {
            eprintln!("{}\n{}", err, vendor.help());
        }
        _ => eprintln!("{}", err),
    }
}
