use std::env;
use std::io::{self, Write};
use std::process::{self, Command};
use upt::{detect_os_vender, lookup_vender, UptError};

fn main() {
    let cmd = match solve_cmd() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
    let output = match Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn solve_cmd() -> Result<String, UptError> {
    let env_args = env::args().collect::<Vec<String>>();
    let (bin, remind_args) = env_args.split_first().unwrap();;
    let bin_vender = lookup_vender(bin)?;
    let task = bin_vender.parse(remind_args)?;
    let os_vender = detect_os_vender()?;
    let exe_args = os_vender.eval(task);
    Ok(exe_args.join(""))
}
