mod line;
mod region;

use log::{debug, info};

use crate::Args;
use crate::Algorithm;
use crate::types::{Comp, Par, System, Limits};
use crate::utils::storage;


pub fn run(args: &Args) {
    /* Check if necessary optional parameters were provided (a paradox, huh?) */
    let algorithm_option = &args.algorithm;
    let algorithm = algorithm_option.as_ref().expect("data requires algorithm to be set");

    match algorithm {
         Algorithm::Line => line::run_line(args),
         Algorithm::Region => region::run_region(args),
    };
}