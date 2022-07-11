#![allow(dead_code)]
#![allow(unused_imports)]

mod nu;
mod data;
mod systems;
mod types;
mod utils;

use clap::{Parser, Subcommand};
use std::fmt;
use std::fmt::Display;
use std::{str::FromStr};
use log::{info, LevelFilter};
use std::time::Instant;
use std::panic;
use std::process;


#[derive(Parser)]
#[clap(author = "Vukan Turkulov <vukant@gmail.com>")]
#[clap(about = "Framework for stability analysis of linear systems")]
#[clap(name = "fsals")]
pub struct Args {
    /// Name of the system which the program should analyze
    #[clap(short, long)]
    system: Option<String>,

    /// Algorithm which should be run on the given system
    #[clap(short, long)]
    algorithm: Option<Algorithm>,

    /// Logging Level
    #[clap(short, long, default_value_t = LevelFilter::Info)]
    loglevel: LevelFilter,

    /// Parallelize code execution
    #[clap(short, long)]
    parallel: bool,

    /// Save verbose data as output
    #[clap(short, long)]
    verbose_data: bool,

    /// Main command that should be run
    #[clap(subcommand)]
    command: Command,
}


#[derive(Subcommand, Debug)]
pub enum Command {
    /// Determine number of unstable poles for the given system using Cauchy's argument principle
    Nu,
    /// Run the specified algorithm
    Data,
    /// Run a custom snippet of Rust code
    Custom,
}


#[derive(Debug)]
pub enum Algorithm {
    /// Line fsals algorithm
    Line,
    /// Region fsals algorithm
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


fn print_args_verbose(args: &Args) {
    info!("Input parameters:");
    info!("  {:<12} {:?}", "Command:", args.command);
    info!("  {:<12} {}", "Algorithm:" , match &args.algorithm {
        Some(algo) => algo.to_string(),
        None =>  String::from("unspecified"),
    });
    info!("  {:<12} {}", "System:", match &args.system {
        Some(name) => String::from(name),
        None => String::from("unspecified"),
    });
    info!("  {:<12} {}", "Parallel:", args.parallel);
    info!("  {:<12} {}", "Verbose data:", args.verbose_data);
    info!("  {:<12} {}", "LogLevel:", args.loglevel);
}


fn main() {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let start = Instant::now();
    let args = Args::parse();
    simple_logger::SimpleLogger::new()
        .with_utc_timestamps()
        .with_level(args.loglevel)
        .init()
        .unwrap();

    info!("Starting rust program");
    print_args_verbose(&args);

    match args.command {
        Command::Nu => nu::run(&args),
        Command::Data => data::run(&args),
        Command::Custom => info!("Dummy placeholder for running custom commands"),
    };

    let end = Instant::now();
    let elapsed = end - start;
    info!("Rust program completed in {:?}", elapsed);
}