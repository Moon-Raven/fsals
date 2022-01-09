mod configurations;

use log::{debug, info};
use iter_num_tools::lin_space;
use serde::Serialize;
use std::f64::consts::PI;
use std::f64;
use std::iter::Cloned;

use crate::Args;
use crate::types::{Comp, Par, System, Limits};
use crate::nu;
use crate::utils::{geometry, optimization};
use configurations::{Delta, LineConfiguration, CONFIGURATONS};
use cgmath::Vector2;
use rayon::prelude::*;


#[derive(Serialize)]
pub struct Ray {
    origin: Par,
    angle: f64,
    length: f64,
}


#[derive(Serialize)]
pub struct RayFan {
    origin: Par,
    rays: Vec<Ray>,
    nu: i32,
}


fn spawn_angles(limits: &Limits, count: usize) -> Vec<f64> {
    let angles_lin = lin_space(-PI..PI, count);
    let p1_span = limits.p1_max - limits.p1_min;
    let p2_span = limits.p2_max - limits.p2_min;
    let ratio = p1_span / p2_span;

    /* Find out which angles are in the first and fourth quadrant */
    let is_right = angles_lin.into_iter().map(|angle| angle >= -PI/2.0 && angle <= PI/2.0);

    /* Prepare modifiers based on angle quadrants */
    let modifiers = is_right.map(|b| match b {true => 0.0, false => PI});

    let angles_lin = lin_space(-PI..PI, count);
    let zipped = angles_lin.zip(modifiers);

    let angles_scaled = zipped.map(move |(angle, modifier)|
        f64::atan(f64::tan(angle)/ ratio) + modifier);

    angles_scaled.collect()
}


fn delta_rel2abs(conf: &LineConfiguration, delta_rel: f64, angle: f64) -> f64 {
    let p1span = conf.limits.p1_max - conf.limits.p1_min;
    let p2span = conf.limits.p2_max - conf.limits.p2_min;
    let p1delta = p1span * delta_rel;
    let p2delta = p2span * delta_rel;

    let delta1 = match f64::cos(angle) != 0.0 {
        true => f64::abs(p1delta / f64::cos(angle)),
        false => f64::INFINITY,
    };
    let delta2 = match f64::cos(angle) != 0.0 {
        true => f64::abs(p2delta / f64::cos(angle)),
        false => f64::INFINITY,
    };
    let delta_abs = f64::min(delta1, delta2);

    delta_abs
}


fn check_jump_validity<F1, F2> (
    f: F1,
    line_denominator: F2,
    theta0: f64,
    delta_theta: f64,
) -> bool
where
F1: Fn(Comp, f64) -> Comp,
F2: Fn(f64, f64, f64) -> f64
{
    let theta_min = theta0;
    let theta_max = theta0 + delta_theta;
    let numerator = |w: f64| Comp::norm(f(Comp::new(0.0, w), theta0));
    let denominator = |w: f64| line_denominator(w, theta_min, theta_max);
    let fraction = |w: f64| numerator(w) / denominator(w);
    let min = optimization::find_minimum(fraction);
    let jump_valid = delta_theta < min;
    jump_valid
}


fn find_max_delta_theta<F1, F2>(
    f: F1,
    line_denominator: F2,
    theta0: f64,
    delta: f64,
    limit: f64
) -> f64
where
F1: Fn(Comp, f64) -> Comp,
F2: Fn(f64, f64, f64) -> f64
{
    let min_step = delta;
    let condition = |delta_theta: f64| {
        check_jump_validity(&f, &line_denominator, theta0, delta_theta)
    };
    optimization::get_maximum_condition(condition, min_step, limit)
}


fn get_stability_segment_1_d<F1, F2>(
    f_1_d: F1,
    line_denom_1_d: F2,
    theta0: f64,
    delta: f64,
    limit: f64,
    conf: &LineConfiguration,
) -> f64
where
    F1: Fn(Comp, f64) -> Comp,
    F2: Fn(f64, f64, f64) -> f64
{
    let mut theta = theta0;
    let mut delta_theta = f64::INFINITY;

    while delta_theta > delta {
        /* Find maximum offset allowed by Rouche's theorem */
        delta_theta = find_max_delta_theta(&f_1_d, &line_denom_1_d, theta, delta, limit);

        /* Reduce change for numerical errors caused by global optimization */
        delta_theta = delta_theta * conf.safeguard;

        /* Update maximum total offset thus far */
        theta += delta_theta;
        if theta > limit {
            theta = limit;
            break;
        }
    }

    theta
}


fn get_max_theta(limits: &Limits, origin: Par, angle: f64, delta: f64) -> f64 {
    let direction = Vector2::new(f64::cos(angle), f64::sin(angle));

    let condition = |theta: f64| -> bool {
        let origin_point = Vector2::new(origin.0, origin.1);
        let offset = theta * direction;
        let p = origin_point + offset;
        geometry::is_point_in_limits((p.x, p.y), limits)
    };
    let min_step = delta;
    let limit = f64::INFINITY;
    optimization::get_maximum_condition(condition, min_step, limit)
}


fn get_stability_segment(conf: &LineConfiguration, angle: f64, origin: Par) -> f64 {
    info!("Getting stability segment for origin {:?}, angle {:?}", origin, angle);
    if !geometry::is_point_in_limits(origin, &conf.limits){
        panic!("Origin not in given limits");
    }

    let delta = match conf.delta {
        Delta::Abs(abs) => abs,
        Delta::Rel(rel) => delta_rel2abs(conf, rel, angle),
    };

    // Create a closure which converts the 2d function to 1d function
    let directional_vec = (f64::cos(angle), f64::sin(angle));
    let f_1_d = |s: Comp, theta: f64 | -> Comp {
        let p0 = origin.0 + theta*directional_vec.0;
        let p1 = origin.1 + theta*directional_vec.1;
        let p = (p0, p1);
        (conf.system.f_complex)(s, p)
    };

    // Create a closure which converts the 2d denom into 1d denom
    let line_denom_2_d = conf.system.line_denominator
        .expect("System must have line denom impl");

    let line_denom_1_d  = |w: f64, th_min: f64, th_max: f64| {
        line_denom_2_d(w, origin, angle, th_min, th_max)
    };

    let limit = get_max_theta(&conf.limits, origin, angle, delta);
    let theta0 = 0.0;

    get_stability_segment_1_d(f_1_d, line_denom_1_d, theta0, delta, limit, conf)
}


fn get_rayfan(conf: &LineConfiguration, origin: Par) -> RayFan {
    info!("Calculating line algo for rayfan {:?}", origin);
    let angles = spawn_angles(&conf.limits, conf.ray_count);

    let nu = nu::calculate_nu_single(&conf.contour_conf, conf.system.f_complex, origin);

    let stability_segments = angles
        .clone()
        .into_par_iter()
        .map(|angle| get_stability_segment(conf, angle, origin));

    let rays = angles
        .into_par_iter()
        .zip(stability_segments)
        .map(|(angle, stability_segment)|{
            Ray {origin: origin, angle: angle, length: stability_segment}}
    );

    RayFan { nu, rays: rays.collect(), origin: origin }
}


#[derive(Serialize)]
pub struct LineResult {
    pub rayfans: Vec<RayFan>,
    pub limits: &'static Limits,
    pub parameters: (&'static str, &'static str),
}


pub fn run_line(args: &Args) -> LineResult {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS.get(config_name.as_str()).expect("Unknown system");

    let rayfans = config
        .origins
        .clone()
        .into_par_iter()
        .map(|origin| get_rayfan(config, origin));

    let results = LineResult {
        rayfans: rayfans.collect(),
        limits: &config.limits,
        parameters: config.system.parameters,
    };

    results
}