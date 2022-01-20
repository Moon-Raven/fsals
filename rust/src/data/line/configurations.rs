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
use crate::utils::geometry::Delta;

use crate::nu::ContourConfiguration;
use crate::types::{Limits, Par, System};




pub struct LineConfiguration {
    pub name: &'static str,
    pub system: System,
    pub limits: Limits,
    pub origins: Vec<Par>,
    pub ray_count: usize,
    pub contour_conf: ContourConfiguration,
    pub delta: Delta,
    pub safeguard: f64,
    pub w_steps_linear: usize,
    pub log_space_minw: f64,
    pub log_space_maxw: f64,
    pub log_space_steps: usize,
}


impl LineConfiguration {
    pub fn get_log_space(&self) -> Vec<f64> {
        iter_num_tools::log_space(self.log_space_minw..=self.log_space_maxw, self.log_space_steps)
            .collect()
    }
}


lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, LineConfiguration> = {
        let mut configs = HashMap::new();

        configs.insert("retarded1", LineConfiguration {
            name: "retarded1",
            system: retarded1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 2.6, p2_min: 0.0, p2_max: 3.3 },
            ray_count: 320,
            safeguard: 0.95,
            origins: vec![
                (1e-2, 1e-2),
                (1.75, 1.20),
                (0.88, 2.73),
                (0.20, 3.10),
                (0.71, 3.22),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 10_000usize,
                },
            delta: Delta::Abs(1e-3),
            w_steps_linear: 1_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 1000,
        });

        configs.insert("distributed_delay1", LineConfiguration {
            name: "distributed_delay1",
            system: distributed_delay1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.5, p2_min: 0.0, p2_max: 1.3 },
            ray_count: 160,
            safeguard: 0.95, // 0.75 in python
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
            delta: Delta::Abs(1e-4),
            w_steps_linear: 1_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 1000,
        });

        // configs.insert("semi_infinite_rod", LineConfiguration {
        //     name: "semi_infinite_rod",
        //     system: semi_infinite_rod::SYSTEM,
        //     limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 7e4 },
        //     ray_count: 160,
        //     safeguard: 0.80,
        //     origins: vec![
        //         // Region 1
        //         (1.0, 0.5e4),
        //         (50.0, 0.5e4),
        //         (90.0, 0.5e4),
        //         (1.0, 4e4),
        //         // Region 2
        //         (50.0, 4e4),
        //         (20.0, 6e4),
        //         (90.0, 3e4),
        //         ],
        //     contour_conf: ContourConfiguration {
        //         w_min: 1e-3,
        //         w_max: 1e5,
        //         steps: 1_000usize,
        //         },
        //     delta: Delta::Abs(0.8),
        //     w_steps_linear: 5_000,
        // });

        // configs.insert("pde_complex_k_sigma", LineConfiguration {
        //     name: "pde_complex_k_sigma",
        //     system: pde_complex_k_sigma::SYSTEM,
        //     limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
        //     ray_count: 160,
        //     safeguard: 0.90,
        //     origins: vec![
        //         // Region 1
        //         (0.5, 0.5),
        //         (4.0, 4.0),
        //         (2.0, 10.0),
        //         (10.0, 2.0),
        //         // Region 2
        //         (6.0, 6.0),
        //         ],
        //     contour_conf: ContourConfiguration {
        //         w_min: 1e-3,
        //         w_max: 1e5,
        //         steps: 1_000usize,
        //         },
        //     delta: Delta::Abs(1e-2),
        //     w_steps_linear: 100, //1000
        // });

        configs
    };
}
