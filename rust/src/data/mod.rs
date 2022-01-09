mod line;

use log::{debug, info};

use crate::Args;
use crate::Algorithm;
use crate::types::{Comp, Par, System, Limits};
use crate::utils::storage;


pub fn run(args: &Args) {
    /* Check if necessary optional parameters were provided (a paradox, huh?) */
    let algorithm_option = &args.algorithm;
    let algorithm = algorithm_option.as_ref().expect("data requires algorithm to be set");

    let results = match algorithm {
         Algorithm::Line => line::run_line(args),
         Algorithm::Region => panic!("region algorithm not yet implemented"),
    };


    // /* Store results in file */
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");
    let command = "data";
    let extension = "data";

    let filename = storage::get_filepath(
        command,
        &algorithm.to_string(),
        extension,
        config_name);

    storage::store_results(results, &filename);
}