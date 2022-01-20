use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::systems::distributed_delay1;
use crate::systems::finite_rod;
use crate::systems::pde_complex_beta_sigma;
use crate::systems::pde_complex_k_sigma;
use crate::systems::pde_complex_tau_sigma;
use crate::systems::retarded1;
use crate::systems::semi_infinite_rod;
use crate::systems::telegrapher_alpha_gamma;
use crate::systems::telegrapher_x_k;

use crate::nu::ContourConfiguration;
use crate::types::{Limits, Par, System};


pub enum Delta {
    Abs(f64),
    Rel(f64),
}


pub struct RegionConfiguration {
    pub name: &'static str,
    pub system: System,
    pub limits: Limits,
    pub origins: Vec<Par>,
    pub contour_conf: ContourConfiguration,
    pub delta: Delta,
    pub safeguard: f64,
    pub spawn_count: usize,
    pub enforce_limits: bool,
    pub lin_steps: usize,
    pub log_space_minw: f64,
    pub log_space_maxw: f64,
    pub log_space_steps: usize,
}


impl RegionConfiguration {
    pub fn get_log_space(&self) -> Vec<f64> {
        iter_num_tools::log_space(self.log_space_minw..=self.log_space_maxw, self.log_space_steps)
            .collect()
    }
}


lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, RegionConfiguration> = {
        let mut configs = HashMap::new();

        configs.insert("retarded1", RegionConfiguration {
            name: "retarded1",
            system: retarded1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 2.6, p2_min: 0.0, p2_max: 3.3 },
            safeguard: 0.7,
            origins: vec![
                (0.25, 1.00),
                (1.75, 1.20),
                (0.88, 2.73),
                (0.20, 3.10),
                (0.71, 3.22),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 10_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 10_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 10_000,
        });

        configs.insert("distributed_delay1", RegionConfiguration {
            name: "distributed_delay1",
            system: distributed_delay1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.5, p2_min: 0.0, p2_max: 1.3 },
            safeguard: 0.75, // 0.75 in python
            origins: vec![
                (0.05, 1.0),
                (4.9, 0.1),
                (8.0, 0.04),
                (11.3, 0.08),
                (14.5, 0.04),
                (17.5, 0.08),
                (12.0, 0.013),
                (18.0, 0.018),
                (19.9, 0.047),
                (16.19, 0.007),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(0.001), // 1e-3 in python
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 10_000,      // 10_000 in python
            log_space_minw: 1e-3,   // 1e-3 in python
            log_space_maxw: 1e5,    // 1e7 in python
            log_space_steps: 10_000, // 10_000 in python
        });

        // configs.insert("semi_infinite_rod", RegionConfiguration {
        //     name: "semi_infinite_rod",
        //     system: semi_infinite_rod::SYSTEM,
        //     limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 7e4 },
        //     safeguard: 0.99,
        //     origins: vec![
        //         (20.0, 1e4),
        //         (60.0, 5e4),
        //         ],
        //     contour_conf: ContourConfiguration {
        //         w_min: 1e-3,
        //         w_max: 1e5,
        //         steps: 1_000,
        //         },
        //     delta: Delta::Rel(0.01),
        //     spawn_count: 32,
        //     enforce_limits: false,
        // });

        configs
    };
}
