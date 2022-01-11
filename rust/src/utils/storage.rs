use log::{debug, info};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;


pub fn store_results<I>(results: I, filename: &String)
where I: Serialize
{
    let results = serde_json::to_string(&results).unwrap();
    info!("Storing results into {}", filename);
    fs::write(filename, results).expect("Unable to store results");
}


pub fn get_filepath(
    command: &str,
    subcommand: &str,
    extension: &str,
    config: &str)
-> String {
    let root = "../";
    let output = "output";

    let mut path: PathBuf = [root, output, command, subcommand, config]
        .iter()
        .collect();
    path.set_extension(extension);
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create path");
    path.into_os_string().into_string().unwrap()

}