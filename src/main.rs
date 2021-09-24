#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

use crate::error::Error;
use crate::simulator::Simulator;

mod simulator;
mod simulator_manager;
mod error;

const USAGE: &'static str = "Usage: meta [command]";
const ADD_USAGE: &'static str = "Usage: meta add <name> <version> <path>";
const RM_USAGE: &'static str = "Usage: meta rm <name> <version>";
const RUN_USAGE: &'static str = "Usage: meta run <name> <version>";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let subcommand = matches.subcommand();

    if let Err(err) = match subcommand {
        ("add", args) => add(args).await,
        ("rm", args) => rm(args).await,
        ("ls", args) => ls(args).await,
        ("run", args) => run(args).await,
        _ => {
            eprintln!("{}", USAGE);
            std::process::exit(1);
        }
    } {
        eprintln!("{}", err.message());
        std::process::exit(1);
    }

    Ok(())
}

async fn add(args: Option<&ArgMatches<'_>>) -> Result<(), Error> {
    let args = args.expect(ADD_USAGE);
    let name = args.value_of("name").expect(ADD_USAGE);
    let version = args.value_of("version").expect(ADD_USAGE);
    let path = args.value_of("path").expect(ADD_USAGE);

    let simulator = Simulator::new(name.into(), version.into());

    simulator_manager::add_simulator(&simulator, path).await
}

async fn rm(args: Option<&ArgMatches<'_>>) -> Result<(), Error> {
    let args = args.expect(RM_USAGE);
    let name = args.value_of("name").expect(RM_USAGE);
    let version = args.value_of("version").expect(RM_USAGE);

    let _simulator = Simulator::new(name.into(), version.into());

    simulator_manager::remove_simulator(&_simulator).await
}

async fn ls(args: Option<&ArgMatches<'_>>) -> Result<(), Error> {
    match args.and_then(|arg_matches| arg_matches.value_of("filter")) {
        Some(filter) => simulator_manager::list_simulators_with_filter(filter).await,
        None => simulator_manager::list_simulators().await
    }
}

async fn run(args: Option<&ArgMatches<'_>>) -> Result<(), Error> {
    let args = args.expect(RUN_USAGE);
    let name = args.value_of("name").expect(RUN_USAGE);
    let version = args.value_of("version").expect(RUN_USAGE);

    let simulator = Simulator::new(name.into(), version.into());

    simulator_manager::run_simulator(simulator).await
}
