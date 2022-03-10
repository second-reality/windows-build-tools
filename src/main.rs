extern crate clap;

use clap::Command;
mod commands;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let m = Command::new("wbt")
        .about("Use windows build tools")
        .subcommand_required(true)
        .subcommand(Command::new("list-packages"))
        .get_matches();

    let (command, _) = m.subcommand().expect("supposed to be required");

    match command {
        "list-packages" => commands::list_packages::list_packages(),
        _ => panic!("subcommand {command} not expected"),
    }
}
