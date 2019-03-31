use std::env;
use std::io::{self, Write};
use std::process::{self, Command};
use upt::{detect_os_vendor, UptError, lookup_vendor, Vendor};

fn main() {
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, remind_args) = env_args.split_first().unwrap();;
    let vendor = match lookup_vendor(bin) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }

    };
    let cmd = match create_cmd(&vendor, remind_args) {
        Ok(v) => v,
        Err(e) => {
            dump_err(&vendor, e);
            process::exit(1);
        }
    };
    let output = match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn create_cmd(vendor: &Vendor, args: &[String]) -> Result<String, UptError> {
    let task = vendor.parse(args)?;
    let vendor = detect_os_vendor()?;
    let cmd = vendor.eval(&task);
    Ok(cmd)
}

fn dump_err(vendor: &Vendor, err: UptError) {
    use UptError::*;
    match err {
        NoSubcommand | NotRecongize => {
            eprintln!("{}\n{}", err, vendor.help());
        }
        _ => eprintln!("{}", err),
    }
}
