mod configurations;

use log::{debug, info, warn, error};
use crate::Args;
use crate::types::{Comp, Par, System, Limits};
use crate::systems;
use std::collections::HashMap;
use lazy_static::lazy_static;
use iter_num_tools::{log_space, lin_space, grid_space};
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::path::{Path,PathBuf};


pub struct Configuration {
    name: &'static str,
    system: System,
    w_min: f64,
    w_max: f64,
    steps: usize,
    pub limits: Limits,
}


#[derive(Debug, serde::Serialize, Deserialize)]
pub struct NuResult {
    p: Par,
    nu: i32,
}


lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, Configuration> = {
        let mut configs = HashMap::new();

        configs.insert("retarded1", configurations::RETARDED1);

        configs
    };
}

fn calculate_nu_single(
    w_min: f64,
    w_max: f64,
    steps: usize,
    f: fn(Comp, Par) -> Comp,
    p: Par
) -> i32
{
    let freq = log_space(w_min..=w_max, steps);
    let imag_positive = freq.map(|w| Comp::new(0.0, w));
    let freq = log_space(w_min..=w_max, steps);
    let imag_negative = freq.map(|w| Comp::new(0.0, -w)).rev();
    let angles = lin_space(-PI/2.0..=PI/2.0, steps).rev();
    let semicircle = angles.map(|theta| Comp::from_polar(w_max, theta));

    let contour = imag_positive;
    let contour = contour.chain(semicircle);
    let contour = contour.chain(imag_negative);

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
            } else { integral += 2.0*PI - diff.abs();
            }
        }
    }

    let windings = -integral / (2.0*PI);
    debug!("Windings (real): {}", windings);
    let windings = windings.round();
    debug!("Windings (rounded): {}", windings);
    windings as i32
}


fn calculate_nu(conf: &Configuration) -> Vec <NuResult> {
    let grid_min = [conf.limits.p1_min, conf.limits.p2_min];
    let grid_max = [conf.limits.p1_max, conf.limits.p2_max];
    let grid = grid_space(grid_min..=grid_max, [5, 5]);
    let results = grid.map(|p| {
        let p = (p[0], p[1]);
        NuResult {
            p: p,
            nu: calculate_nu_single(conf.w_min, conf.w_max, conf.steps, conf.system.f_complex, p)}
        }
    );
    results.collect()
}


pub fn store_results<I>(results: I, filename: &String)
where I: IntoIterator<Item=NuResult> + Serialize
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
    let results = calculate_nu(&config);

    /* Store results in file */
    let filename = get_nu_path(config_name);
    store_results(results, &filename);
}