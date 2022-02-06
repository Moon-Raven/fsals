mod configurations;

use std::sync::{Arc, Mutex, RwLock};
use iter_num_tools::lin_space;
use std::f64::consts::PI;
use log::{debug, info};
use serde::Serialize;
use std::collections::VecDeque;
use std::{fs, slice};

use crate::nu;
use crate::types::{Comp, Limits, Par, System};
use crate::utils::optimization::MinimizationProblemFast;
use crate::utils::{geometry, optimization, storage};
use crate::Args;
use rayon::{prelude::*, ScopeFifo};
use configurations::{Delta, RegionConfiguration, CONFIGURATONS};
use crate::systems::distributed_delay1;


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
    pub origin: Par,
}


fn check_jump_validity (
    conf: &RegionConfiguration,
    origin: Par,
    eps: f64,
    precalculated_numerator: &[f64],
    log_space: &[f64],
) -> bool
{
    let minimization_problem = MinimizationProblemFast {
        log_space: log_space,
        lin_steps: conf.lin_steps,
        logspace_fraction_iterator: (conf.system.region_fraction_precalculated_numerator.unwrap())(
                precalculated_numerator,
                log_space,
                origin,
                eps),
        linspace_fraction_generator: {
            Box::new(move |w_linspace| conf.system.region_fraction.unwrap()(w_linspace, origin, eps))
        },
    };

    let min = optimization::find_minimum_fraction_fast(minimization_problem);

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
    delta_abs: f64,
    log_space: &[f64],
) -> PRegion
{
    let precalculated_numerator: Vec<f64> = log_space
        .iter()
        .map(|w| (conf.system.f_complex)(Comp::new(0.0, *w), origin).norm())
        .collect();

    let condition = |eps| check_jump_validity(
        conf,
        origin,
        eps,
        &precalculated_numerator,
        log_space);

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


pub fn spawn_valid_points<'a, I>(
    pregion: &'a PRegion,
    conf: &'a RegionConfiguration,
    existing_pregs: &'a I,
) -> impl Iterator<Item=Par> + 'a
where
    for<'b> &'b I: IntoIterator<Item=&'b PRegion>
{
    let new_points = pregion.spawn_edge_points(conf.spawn_count);
    let valid_points = new_points
        .into_iter()
        .filter(move |p| {
            (&existing_pregs).into_iter().map(|preg| !preg.is_point_inside(*p)).all(|b| b)})
        .filter(move |p| geometry::is_point_in_limits(*p, &conf.limits));

    valid_points
}


pub fn get_region_parallel<'a>(
    scope: &ScopeFifo<'a>,
    conf: &'a RegionConfiguration,
    origin: Par,
    pregions: &'a Arc<RwLock<Vec<PRegion>>>,
    delta: f64,
    log_space: &'a [f64],
)
{
    /* Check if the point is obsolete */
    {
        let pregions_unlocked = pregions.read().unwrap();

        /* Check if it has been enough */
        const THRESHOLD: usize = 9999999;
        if pregions_unlocked.len() > THRESHOLD { return }

        /* Check if the point is obsolete */
        if pregions_unlocked.iter().map(|preg| preg.is_point_inside(origin)).any(|b| b) {
            return;
        }
    }

    /* Find new PRegion around the point */
    let pregion = get_pregion(conf, origin, conf.enforce_limits, delta, log_space);

    let new_points = {
        let pregions_unlocked = pregions.read().unwrap();

        /* If necessary, spawn new points on edge of the newly obtained PRegion */
        if pregion.radius > delta {
            let new_points: Vec<Par> = spawn_valid_points(
                &pregion,
                &conf,
                &*pregions_unlocked)
                .collect();
            Some(new_points)
        }
        else {None}
    };

    /* Add obtained pregion to the Vec - this is the only access that requires write privileges */
    {
        let mut pregions_unlocked = pregions.write().unwrap();
        pregions_unlocked.push(pregion);
    }

    if let Some(new_points) = new_points {
        if new_points.len() != 0 {
            for point in new_points {
                scope.spawn_fifo(move |s| get_region_parallel(s, conf, point, pregions, delta, log_space));
            }
        }
    }
}


pub fn get_region(conf: &RegionConfiguration, origin: Par) -> Region {
    const VEC_PREALLOCATION_SIZE: usize = 10_000;

    let delta = absolutize_delta(&conf.delta, &conf.limits);
    let nu = nu::calculate_nu_single(&conf.contour_conf, conf.system.f_complex, origin);
    let mut pregions = Arc::new(RwLock::new(Vec::with_capacity(VEC_PREALLOCATION_SIZE)));
    let w_log_space: Vec<f64> = conf.get_log_space();

    info!("Searching for region around {:?} with nu {}", origin, nu);
    rayon::scope_fifo(|s| {
        get_region_parallel(s, conf, origin, &mut pregions, delta, &w_log_space);
    });
    let pregions = Arc::try_unwrap(pregions).unwrap().into_inner().unwrap();
    info!("Returning region around {:?} with {:?} pregions", origin, pregions.len());

    Region { pregions, nu, origin}
}


pub fn store_results(results: &RegionResult, config: &RegionConfiguration) {
    let system_name = config.system.name;
    let command = "data";
    let extension = "data";

    let algorithm_option = "region";

    let filename = storage::get_filepath(command, algorithm_option, extension, system_name);

    storage::store_results(results, &filename);
}


pub fn args2config(args: &Args) -> &'static RegionConfiguration {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS
        .get(config_name.as_str())
        .expect("Unknown system");

    config
}


pub fn run_region(args: &Args) {
    const MEGABYTE: usize = 1024 * 1024;
    const STACK_SIZE: usize = MEGABYTE * 128;
    rayon::ThreadPoolBuilder::new().stack_size(STACK_SIZE).build_global().unwrap();

    let config = args2config(args);


    let regions = config
        .origins
        .clone()
        .into_par_iter()
        // .into_iter()
        .map(|origin| get_region(config, origin));

    let results = RegionResult {
        regions: regions.collect(),
        limits: &config.limits,
        parameters: config.system.parameters,
    };

    store_results(&results, &config);
}