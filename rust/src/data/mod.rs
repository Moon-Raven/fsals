mod line;

use log::{debug, info};

use crate::Args;
use crate::Algorithm;
use crate::types::{Comp, Par, System, Limits};


pub fn run(args: &Args) {
    /* Check if necessary optional parameters were provided (a paradox, huh?) */
    let algorithm_option = &args.algorithm;
    let algorithm_name = algorithm_option.as_ref().expect("data requires algorithm to be set");

    let results = match algorithm_name {
         Algorithm::Line => line::run_line(args),
         Algorithm::Region => panic!("region algorithm not yet implemented"),
    };


    // /* Store results in file */
    // let filename = get_nu_path(config_name);
    // store_results(results, &filename);
}