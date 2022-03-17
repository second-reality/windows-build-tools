extern crate clap;

use clap::{Arg, Command};
mod commands;

const LIST_PACKAGES: &str = "list-packages";
const LIST_TOOLCHAINS: &str = "list-toolchains";
const GET_TOOLCHAIN: &str = "get-toolchain";

const ARG_TOOLCHAIN_VERSION: &str = "toolchain-version";
const ARG_INSTALL_DIR: &str = "install-dir";

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let m = Command::new("wbt")
        .about("Use windows build tools")
        .subcommand_required(true)
        .subcommand(Command::new(LIST_PACKAGES))
        .subcommand(Command::new(LIST_TOOLCHAINS))
        .subcommand(
            Command::new(GET_TOOLCHAIN)
                .arg(Arg::new(ARG_TOOLCHAIN_VERSION).required(true))
                .arg(Arg::new(ARG_INSTALL_DIR).required(true)),
        )
        .get_matches();

    let (command, args) = m.subcommand().expect("supposed to be required");

    match command {
        LIST_PACKAGES => commands::list_packages::run(),
        LIST_TOOLCHAINS => commands::list_toolchains::run(),
        GET_TOOLCHAIN => commands::get_toolchain::run(
            args.value_of_t_or_exit(ARG_TOOLCHAIN_VERSION),
            args.value_of_t_or_exit(ARG_INSTALL_DIR),
        ),
        _ => panic!("subcommand {command} not expected"),
    }
}
