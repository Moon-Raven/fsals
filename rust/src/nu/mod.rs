mod configurations;

use log::{debug, info};
use std::collections::HashMap;
use lazy_static::lazy_static;
use iter_num_tools::{log_space, lin_space, grid_space};
use std::f64::consts::PI;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use rayon::prelude::*;

use crate::Args;
use crate::types::{Comp, Par, System, Limits};
pub use configurations::{ContourConfiguration,NuConfiguration,CONFIGURATONS};


#[derive(Debug, serde::Serialize)]
pub struct NuPointResult {
    p: Par,
    nu: i32,
}


#[derive(Debug, serde::Serialize)]
pub struct NuResult {
    point_results: Vec<NuPointResult>,
    limits: &'static Limits,
    parameters: (&'static str, &'static str),
}


fn get_bromwhich_contour(conf: &ContourConfiguration) -> impl Iterator<Item=Comp>  + '_ {
    const SAFETY_OFFSET: f64 = 1e-3; // Nudge the contour a bit to the right

    let freq = log_space(conf.w_min..=conf.w_max, conf.steps);
    let imag_positive = freq.map(|w| Comp::new(SAFETY_OFFSET, w));
    let freq = log_space(conf.w_min..=conf.w_max, conf.steps);
    let imag_negative = freq.map(|w| Comp::new(SAFETY_OFFSET, -w)).rev();
    let angles = lin_space(-PI/2.0..=PI/2.0, conf.steps).rev();
    let semicircle = angles.map(move |theta| Comp::from_polar(conf.w_max, theta));

    let contour = imag_positive;
    let contour = contour.chain(semicircle);
    let contour = contour.chain(imag_negative);

    contour
}


pub fn calculate_nu_single(
    contour_conf: &ContourConfiguration,
    f: fn(Comp, Par) -> Comp,
    p: Par
) -> i32
{
    let contour = get_bromwhich_contour(contour_conf);
    let image = contour.map(|s| f(s, p));
    let angles: Vec<f64> = image.map(|s| s.arg()).collect();

    let mut integral: f64 = 0.0;

    for i in 0..angles.len()-1 {
        let diff = angles[i+1] - angles[i];
        if diff > 0.0 {
            if diff < PI {
                integral += diff;
            }
            else {
                integral -= 2.0*PI - diff;
            }
        }
        else {
            if diff.abs() < PI {
                integral -= diff.abs();
            } else {
                integral += 2.0*PI - diff.abs();
            }
        }
    }

    let windings = -integral / (2.0*PI);

    if windings.is_nan() {
        panic!("NaN value detected");
    }

    debug!("Windings (real): {}", windings);
    let windings = windings.round();
    debug!("Windings (rounded): {}", windings);

    windings as i32
}


fn calculate_nu(conf: &'static NuConfiguration, _parallel: bool) -> NuResult {
    let grid_min = [conf.limits.p1_min, conf.limits.p2_min];
    let grid_max = [conf.limits.p1_max, conf.limits.p2_max];
    let steps = [conf.grid_step, conf.grid_step];
    let grid: Vec<[f64; 2]> = grid_space(grid_min..=grid_max, steps).collect();

    let grid = grid.into_par_iter();
    let results = grid.map(|p| {
        let p = (p[0], p[1]);
        NuPointResult {
            p: p,
            nu: calculate_nu_single(&conf.contour_conf, conf.system.f_complex, p)}
        }
    );
    let ret_val = NuResult {
        point_results: results.collect(),
        limits: &conf.limits,
        parameters: conf.system.parameters,
    };

    ret_val
}


pub fn store_results<I>(results: I, filename: &String)
where I: Serialize
{
    let results = serde_json::to_string(&results).unwrap();
    info!("Storing results into {}", filename);
    fs::write(filename, results).expect("Unable to store nu results");
}


fn get_nu_path(config_name: &String) -> String {
    let root = "../";
    let output = "output";
    let nu = "nu";
    let temp = "temp_data";
    let extension = "nudata";

    let mut path: PathBuf = [root, output, nu, temp, config_name].iter().collect();
    path.set_extension(extension);
    fs::create_dir_all(path.parent().unwrap()).expect("Unable to create path");
    path.into_os_string().into_string().unwrap()
}


pub fn run(args: &Args) {
    /* Calculate nu */
    let config_name_option = &args.system;
    let config_name = config_name_option.as_ref().expect("nu requires system to be specified");
    let config = CONFIGURATONS.get(config_name.as_str()).expect("Unknown system");
    let results = calculate_nu(&config, args.parallel);

    /* Store results in file */
    let filename = get_nu_path(config_name);
    store_results(results, &filename);
}