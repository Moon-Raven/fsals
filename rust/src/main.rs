use clap::{Parser, Subcommand};
use std::fmt;
use std::fmt::Display;
use std::{str::FromStr};
use log::info;


#[derive(Parser)]
#[clap(author = "Vukan Turkulov <vukant@gmail.com>")]
#[clap(about = "Framework for stability analysis of linear systems")]
#[clap(name = "fsals")]
struct Args {
    /// Name of the system which the program should analyze
    #[clap(short, long)]
    system: Option<String>,

    /// Algorithm which should be run on the given system
    #[clap(short, long)]
    algorithm: Option<Algorithm>,

    /// Parallelize code execution
    #[clap(short, long)]
    parallel: bool,

    #[clap(subcommand)]
    command: Command,
}


#[derive(Subcommand, Debug)]
enum Command {
    /// Determines the number of unstable poles for the given system
    Nu,
    /// Runs the specified algorithm
    Data,
    /// Runs a custom snippet of Rust code
    Custom,
}


#[derive(Debug)]
enum Algorithm {
    /// Line algorithm
    Line,
    /// Region algorithm
    Region,
}


impl FromStr for Algorithm {
    type Err = String;
    fn from_str(input: &str) -> Result<Algorithm, Self::Err> {
        match input {
            "line"  => Ok(Algorithm::Line),
            "region"  => Ok(Algorithm::Region),
            _      => Err(String::from("unknown algorithm"))
        }
    }
}


impl Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Algorithm::Line => write!(f, "line"),
           Algorithm::Region => write!(f, "region"),
       }
    }
}


fn print_args_verbose(args: Args) {
    info!("Input parameters:");
    info!("  {:<12} {:?}", "Command:", args.command);
    info!("  {:<12} {}", "Algorithm:" , match args.algorithm {
        Some(algo) => algo.to_string(),
        None =>  String::from("unspecified"),
    });
    info!("  {:<12} {}", "System:", match args.system {
        Some(name) => name,
        None => String::from("unspecified"),
    });
    info!("  {:<12} {}", "Parallel:", args.parallel);
}


fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let args = Args::parse();

    info!("Starting rust program; input parameters:");
    print_args_verbose(args);
    info!("Rust program complete");
}