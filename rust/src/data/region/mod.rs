mod configurations;

use iter_num_tools::lin_space;
use std::f64::consts::PI;
use log::{debug, info};
use serde::Serialize;
use std::collections::VecDeque;

use crate::nu;
use crate::types::{Comp, Limits, Par, System};
use crate::utils::{geometry, optimization, storage};
use crate::Args;
use configurations::{Delta, RegionConfiguration, CONFIGURATONS};
use std::fs;


#[derive(Serialize, Debug)]
pub struct RegionResult {
    pub regions: Vec<Region>,
    pub limits: &'static Limits,
    pub parameters: (&'static str, &'static str),
}


#[derive(Serialize, Debug)]
pub struct PRegion {
    pub origin: Par,
    pub radius: f64,
}

impl PRegion {
    fn spawn_edge_points(&self, point_count: usize) -> Vec<Par> {
        let angles = lin_space(0.0..2.0*PI, point_count);
        let angles2 = angles.clone();
        let p1_offsets = angles.into_iter().map(|angle| self.radius * f64::cos(angle));
        let p2_offsets = angles2.into_iter().map(|angle| self.radius * f64::sin(angle));
        let it = p1_offsets.zip(p2_offsets);
        it.map(|(p1_offset, p2_offset)| (self.origin.0 + p1_offset, self.origin.1 + p2_offset))
            .collect()
    }

    fn is_point_inside(&self, p: Par) -> bool {
        const SAFEGUARD: f64 = 0.99;
        let x = f64::abs(self.origin.0 - p.0);
        let y = f64::abs(self.origin.1 - p.1);
        let distance = f64::sqrt(x.powi(2) + y.powi(2));
        distance < self.radius * SAFEGUARD
    }
}


#[derive(Serialize, Debug)]
pub struct Region {
    pub pregions: Vec<PRegion>,
    pub nu: i32,
}


fn check_jump_validity(conf: &RegionConfiguration, origin: Par, eps: f64) -> bool {
    let numerator = |w| (conf.system.f_complex)(Comp::new(0.0, w), origin).norm();
    let denominator = |w| (conf.system.region_denominator.unwrap())(w, origin, eps);
    let fraction = |w| numerator(w) / denominator(w);
    let min = optimization::find_minimum(fraction);

    eps < min
}


fn get_limiting_eps(p: Par, limits: &Limits) -> f64 {
    let p1min_distance = f64::abs(p.0 - limits.p1_min);
    let p1max_distance = f64::abs(p.0 - limits.p1_max);
    let p2min_distance = f64::abs(p.1 - limits.p2_min);
    let p2max_distance = f64::abs(p.1 - limits.p2_max);
    let p1_limiting_eps = f64::min(p1min_distance, p1max_distance);
    let p2_limiting_eps = f64::min(p2min_distance, p2max_distance);

    f64::min(p1_limiting_eps, p2_limiting_eps)
}


fn delta_rel2abs(delta: f64, limits: &Limits) -> f64 {
    let p1span = limits.p1_max - limits.p1_min;
    let p2span = limits.p2_max - limits.p2_min;
    let p1delta = p1span * delta;
    let p2delta = p2span * delta;

    f64::min(p1delta, p2delta)
}


fn get_pregion(
    conf: &RegionConfiguration,
    origin: Par,
    enforce_limits: bool,
    delta_abs: f64) -> PRegion
{
    let condition = |eps| check_jump_validity(conf, origin, eps);
    let limit = match enforce_limits {
        true => get_limiting_eps(origin, &conf.limits),
        false => f64::INFINITY,
    };
    info!("Finding pregion for origin {:?}; delta={}, limit={}", origin, delta_abs, limit);
    let radius = optimization::get_maximum_condition(condition, delta_abs, limit);

    PRegion { origin, radius }
}


pub fn absolutize_delta(delta: &Delta, limits: &Limits) -> f64 {
    match delta {
        Delta::Abs(abs) => *abs,
        Delta::Rel(rel) => delta_rel2abs(*rel, limits),
    }
}


pub fn get_region(conf: &RegionConfiguration, origin: Par) -> Region {
    const VEC_PREALLOCATION_SIZE: usize = 10_000;
    let delta = absolutize_delta(&conf.delta, &conf.limits);
    let nu = nu::calculate_nu_single(&conf.contour_conf, conf.system.f_complex, origin);
    let mut pending_points: VecDeque<Par> = VecDeque::with_capacity(VEC_PREALLOCATION_SIZE);
    pending_points.push_back(origin);
    let mut pregions: Vec<PRegion> = Vec::with_capacity(VEC_PREALLOCATION_SIZE);

    info!("Searching for region around {:?} with nu {}", origin, nu);

    while let Some(p) = pending_points.pop_front() {

        if pregions.iter().map(|preg| preg.is_point_inside(p)).any(|b| b) {
            continue;
        }

        let pregion = get_pregion(conf, p, conf.enforce_limits, delta);
        if pregion.radius > delta { // Test with > and >=
            let new_points = pregion.spawn_edge_points(conf.spawn_count);
            let mut valid_points: VecDeque<Par> = new_points
                .into_iter()
                .filter(|p| {pregions.iter().map(|preg| !preg.is_point_inside(*p)).all(|b| b)})
                .filter(|p| geometry::is_point_in_limits(*p, &conf.limits))
                .collect();

            pending_points.append(&mut valid_points);
        }

        pregions.push(pregion);
    }

    info!("Returning region around {:?} with {:?} pregions", origin, pregions.len());
    Region { pregions, nu }
}


pub fn run_region(args: &Args) {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS
        .get(config_name.as_str())
        .expect("Unknown system");

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
    let algorithm = algorithm_option
        .as_ref()
        .expect("data requires algorithm to be set");

    let filename = storage::get_filepath(command, &algorithm.to_string(), extension, config_name);

    storage::store_results(results, &filename);
}
