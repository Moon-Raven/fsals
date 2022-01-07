mod configurations;

use log::{debug, info};
use iter_num_tools::lin_space;
use std::f64::consts::PI;
use std::f64;

use crate::Args;
use crate::types::{Comp, Par, System, Limits};
use configurations::{LineConfiguration,CONFIGURATONS};


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


fn spawn_angles(limits: &Limits, count: usize) -> impl Iterator<Item=f64> {
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

    angles_scaled
}


pub fn get_rayfan(conf: &LineConfiguration, origin: &Par) {
    let angles = spawn_angles(&conf.limits, conf.ray_count);
    //   For each angle in config.angles (don't forget scaling)
    //     Calculate delta abs / delta relative
    //     Create a closure which converts the 2d function to 1d function
    //     Create a closure which converts the 2d denom into 1d denom
    //     Find limiting value of theta

    //     While delta_theta > delta
    //       find max delta theta:
    //         Create a closure which checks if a delta theta is valid
    //         Find maximum value which satisfies the condition (why is minimal in python?)
    //       if contions are met, terminate
}


pub fn run_line(args: &Args) -> bool {
    let config_name_option = &args.system;
    let config_name = config_name_option
        .as_ref()
        .expect("data requires system to be specified");

    let config = CONFIGURATONS.get(config_name.as_str()).expect("Unknown system");
    let results = config.origins.iter().map(|origin| get_rayfan(config, origin));

    // let results: Vec<()> = results.collect();
    true
}
