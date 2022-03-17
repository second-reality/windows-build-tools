extern crate clap;

use clap::Command;
mod commands;

const LIST_PACKAGES: &str = "list-packages";
const LIST_TOOLCHAINS: &str = "list-toolchains";
const GET_TOOLCHAIN: &str = "get-toolchain";

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let m = Command::new("wbt")
        .about("Use windows build tools")
        .subcommand_required(true)
        .subcommand(Command::new(LIST_PACKAGES))
        .subcommand(Command::new(LIST_TOOLCHAINS))
        .subcommand(Command::new(GET_TOOLCHAIN))
        .get_matches();

    let (command, _) = m.subcommand().expect("supposed to be required");

    match command {
        LIST_PACKAGES => commands::list_packages::run(),
        LIST_TOOLCHAINS => commands::list_toolchains::run(),
        GET_TOOLCHAIN => commands::get_toolchain::run(),
        _ => panic!("subcommand {command} not expected"),
    }
}
