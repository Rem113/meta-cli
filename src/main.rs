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
    let args: Vec<_> = std::env::args().collect();

    let command = match args.iter().skip(1).next() {
        Some(command) => command,
        None => {
            eprintln!("{}", USAGE);
            // TODO: Extract code to constant
            std::process::exit(1);
        }
    };

    let error = match &command[..] {
        "add" => add(args).await,
        "rm" => rm(args).await,
        "ls" => ls(args).await,
        "run" => run(args).await,
        _ => {
            eprintln!("{}", USAGE);
            std::process::exit(1);
        }
    };

    if let Some(error) = error {
        eprintln!("{}", error.message());
    };

    Ok(())
}

async fn add(args: Vec<String>) -> Option<Error> {
    let name = args.get(2)
        .expect(ADD_USAGE);

    let version = args.get(3)
        .expect(ADD_USAGE);

    let path = args.get(4)
        .expect(ADD_USAGE);

    let simulator = Simulator::new(name.into(), version.into());

    simulator_manager::add_simulator(&simulator, path).await
}

async fn rm(args: Vec<String>) -> Option<Error> {
    let name = args.get(2)
        .expect(RM_USAGE);

    let version = args.get(3)
        .expect(RM_USAGE);

    let _simulator = Simulator::new(name.into(), version.into());

    simulator_manager::remove_simulator(&_simulator).await
}

async fn ls(args: Vec<String>) -> Option<Error> {
    match args.get(2) {
        Some(filter) => simulator_manager::list_simulators_with_filter(filter).await,
        None => simulator_manager::list_simulators().await
    }
}

async fn run(args: Vec<String>) -> Option<Error> {
    let name = args.get(2)
        .expect(RUN_USAGE);

    let version = args.get(3)
        .expect(RUN_USAGE);

    let simulator = Simulator::new(name.into(), version.into());

    simulator_manager::run_simulator(simulator).await
}
