mod configurations;

use log::{debug, info};
use iter_num_tools::lin_space;
use std::f64::consts::PI;
use std::f64;
use std::iter::Cloned;

use crate::Args;
use crate::types::{Comp, Par, System, Limits};
use crate::nu;
use crate::utils::{geometry, optimization};
use configurations::{Delta, LineConfiguration, CONFIGURATONS};
use cgmath::Vector2;


struct Ray {
    origin: Par,
    angle: f64,
    length: f64,
}


struct RayFan {
    origin: Par,
    rays: Vec<Ray>,
    nu: i32,
}


type F1d = fn(s: Comp, theta: f64) -> Comp;
type LineDenomFunc1d = fn(w: f64, theta_min: f64, theta_max: f64) -> Comp;


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


fn get_stability_segment_1D(
    f_1D: F1d,
    line_denom_1D: LineDenomFunc1d,
    theta0: f64,
    delta: f64,
    limit: f64,
    conf: &LineConfiguration,
) -> f64
{

    0.0
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
    if !geometry::is_point_in_limits(origin, &conf.limits){
        panic!("Origin not in given limits");
    }

    let delta = match conf.delta {
        Delta::Abs(abs) => abs,
        Delta::Rel(rel) => delta_rel2abs(conf, rel, angle),
    };

    // Create a closure which converts the 2d function to 1d function
    let directional_vec = (f64::cos(angle), f64::sin(angle));
    let f_1D = |s: Comp, theta: f64 | -> Comp {
        let p0 = origin.0 + theta*directional_vec.0;
        let p1 = origin.1 + theta*directional_vec.1;
        let p = (p0, p1);
        (conf.system.f_complex)(s, p)
    };

    //     Create a closure which converts the 2d denom into 1d denom
    let line_denom_2D = conf.system.line_denominator.expect("System must have line denom impl");
    let line_denom_1D  = |w: f64, th_min: f64, th_max: f64| {
        line_denom_2D(w, origin, angle, th_min, th_max)
    };

    let limit = get_max_theta(&conf.limits, origin, angle, delta);

    //     While delta_theta > delta
    //       find max delta theta:
    //         Create a closure which checks if a delta theta is valid
    //         Find maximum value which satisfies the condition (why is minimal in python?)
    //       if contions are met, terminate
    9.0
}


fn get_rayfan(conf: &LineConfiguration, origin: Par) -> RayFan {
    let angles = spawn_angles(&conf.limits, conf.ray_count);

    let nu = nu::calculate_nu_single(&conf.contour_conf, conf.system.f_complex, origin);

    let stability_segments = angles.clone().into_iter().map({
        |angle| get_stability_segment(conf, angle, origin)});
    let rays = angles.into_iter().zip(stability_segments).map(|(angle, stability_segment)| {
        Ray {origin: origin, angle: angle, length: stability_segment}
    });

    RayFan { nu, rays: rays.collect(), origin: origin }
}


pub fn run_line(args: &Args) -> bool {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS.get(config_name.as_str()).expect("Unknown system");
    let results = config.origins.clone().into_iter().map(|origin| get_rayfan(config, origin));

    // let results: Vec<()> = results.collect();
    true
}
