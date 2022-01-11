mod configurations;

use serde::Serialize;
use log::{debug, info};
use std::collections::VecDeque;

use crate::Args;
use configurations::{Delta, RegionConfiguration, CONFIGURATONS};
use crate::types::{Comp, Par, System, Limits};
use crate::nu;
use crate::utils::storage;
use std::fs;


#[derive(Serialize)]
pub struct RegionResult {
    pub regions: Vec<Region>,
    pub limits: &'static Limits,
    pub parameters: (&'static str, &'static str),
}


#[derive(Serialize)]
pub struct PRegion {
    pub origin: Par,
    pub radius: f64,
}


#[derive(Serialize)]
pub struct Region {
    pub pregions: Vec<PRegion>,
    pub nu: i32,
}


fn get_pregion(conf: &RegionConfiguration, origin: Par) -> PRegion {

}



pub fn get_region(conf: &RegionConfiguration, origin: Par) -> Region {
    let nu = nu::calculate_nu_single(&conf.contour_conf, conf.system.f_complex, origin);
    let pending_points: VecDeque<Par> = VecDeque::new();
    pending_points.push_back(origin);
    let pregions: Vec<PRegion> = vec![];

    while let Some(p) = pending_points.pop_front() {
        let pregion = get_pregion(conf, p);
    //      Delete points which are now obsoleted
    //      If needed, spawn new points on edges of new pregion
    //          Spawn new points
    //          Remove those which are obsolete
    //          Add remaining ones to pending_points

        pregions.push(pregion);
    }

    Region { pregions, nu }
}


pub fn run_region(args: &Args) {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS.get(config_name.as_str()).expect("Unknown system");

    let regions = config
        .origins
        .clone()
        .into_iter()
        .map(|origin| get_region(config, origin));

    let results = RegionResult {
        regions: regions.collect(),
        limits: &config.limits,
        parameters: config.system.parameters,
    };

    /* Store results in file */
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");
    let command = "data";
    let extension = "data";

    let algorithm_option = &args.algorithm;
    let algorithm = algorithm_option.as_ref().expect("data requires algorithm to be set");

    let filename = storage::get_filepath(
        command,
        &algorithm.to_string(),
        extension,
        config_name);

    storage::store_results(results, &filename);
}
