use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::systems::distributed_delay1;
use crate::systems::finite_rod;
use crate::systems::pde_complex_beta_sigma;
use crate::systems::pde_complex_k_sigma;
use crate::systems::pde_complex_tau_sigma;
use crate::systems::retarded1;
use crate::systems::retarded2;
use crate::systems::semi_infinite_rod;
use crate::systems::telegrapher_alpha_gamma;
use crate::systems::telegrapher_x_k;
use crate::systems::telegrapher_standard;
use crate::systems::test_system;
use crate::systems::ln_system1;
use crate::systems::dopid2;
use crate::systems::dopid3;

use crate::nu::ContourConfiguration;
use crate::types::{Limits, Par, System};


pub enum Delta {
    Abs(f64),
    Rel(f64),
}


pub struct RegionConfiguration {
    pub name: &'static str,                  // Config name
    pub system: System,                      // System to be used
    pub limits: Limits,                      // Parametric search domain
    pub origins: Vec<Par>,                   // Origins upon which to run fsals
    pub contour_conf: ContourConfiguration,  // Contour used for evaluating NU
    pub delta: Delta,                        // Termination criteria (relative or absolute)
    pub safeguard: f64,                      // Safeguard against numerical optimization errors
    pub spawn_count: usize,                  // Sample point count for newly obtained regions
    pub enforce_limits: bool,                // Enforce searching domain limits?
    pub lin_steps: usize,                    // Domain granularity for linear minimization
    pub log_space_minw: f64,                 // Lower w for logarithmic minimization
    pub log_space_maxw: f64,                 // Upper w for logarithmic minimization
    pub log_space_steps: usize,              // Domain granularity for logarithmic minimization
    pub max_iter: Option<u32>,               // Maximal number of allowed iterations (depth)
    pub check_obsoletion: bool,              // Check if points are obsolete before evaluating them
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
            safeguard: 0.98,
            check_obsoletion: true,
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
            max_iter: Option::None,
        });

        configs.insert("retarded2", RegionConfiguration {
            name: "retarded2",
            system: retarded2::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 5.0, p2_min: 0.0, p2_max: 5.0 },
            safeguard: 0.80,
            check_obsoletion: true,
            origins: vec![
                (1.5, 1.5),
                (0.1, 0.1),
                (0.1, 3.0),
                (3.55, 0.5),
                (4.0, 4.0),
                (0.55, 2.8),
                (2.95, 4.95),
                (4.5, 0.5),
                (0.5, 4.0),
                (0.44, 4.95),
                (2.2, 4.95),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 10_000,
                },
            delta: Delta::Abs(0.0001),
            spawn_count: 16,
            enforce_limits: false,
            lin_steps: 150_000,
            log_space_minw: 0.65,
            log_space_maxw: 1e4,
            log_space_steps: 30_000,
            max_iter: Option::None,
        });


        configs.insert("distributed_delay1", RegionConfiguration {
            name: "distributed_delay1",
            system: distributed_delay1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.5, p2_min: 0.0, p2_max: 0.5 },
            safeguard: 0.95,
            check_obsoletion: true,
            origins: vec![
                (1.0, 0.2),
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
            delta: Delta::Abs(5e-4),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-4,
            log_space_maxw: 1e1,
            log_space_steps: 10_000,
            lin_steps: 10_000,
            max_iter: Option::None,
        });

        configs.insert("pde_complex_k_sigma", RegionConfiguration {
            name: "pde_complex_k_sigma",
            system: pde_complex_k_sigma::SYSTEM,
            limits: Limits { p1_min: 0.1, p1_max: 20.0, p2_min: 0.1, p2_max: 20.0 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (4.0, 4.0),
                (6.0, 6.0),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(1e-2),
            spawn_count: 32, // Changing to higher value might fix issue in upper right corner
            enforce_limits: false,
            lin_steps: 1_000,      // 10_000 in python
            log_space_minw: 1e-3,   // 1e-3 in python
            log_space_maxw: 1e5,    // 1e7 in python
            log_space_steps: 1_000, // 10_000 in python
            max_iter: Option::None,
        });

        configs.insert("pde_complex_beta_sigma", RegionConfiguration {
            name: "pde_complex_beta_sigma",
            system: pde_complex_beta_sigma::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 2.0, p2_min: 0.0, p2_max: 2.0 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (1.0, 1.0),
                (1.80, 1.50),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 1_000,      // 10_000 in python
            log_space_minw: 1e-3,   // 1e-3 in python
            log_space_maxw: 1e5,    // 1e7 in python
            log_space_steps: 1_000, // 10_000 in python
            max_iter: Option::None,
        });

        configs.insert("pde_complex_tau_sigma", RegionConfiguration {
            name: "pde_complex_tau_sigma",
            system: pde_complex_tau_sigma::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                ( 1.0,  1.0),
                (10.0, 10.0),
                (18.0,  2.0),
                (19.0,  0.15),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            // delta: Delta::Abs(1e-3),
            delta: Delta::Abs(1e-1),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 1_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("telegrapher_x_k", RegionConfiguration {
            name: "telegrapher_x_k",
            system: telegrapher_x_k::SYSTEM,
            limits: Limits { p1_min: 0.1, p1_max: 4.0, p2_min: 4.1, p2_max: 8.0 },
            safeguard: 0.95,
            check_obsoletion: true,
            origins: vec![
                (0.4, 7.0),
                (2.5, 6.0),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(1e-4),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 1_000,
            log_space_minw: 1e-2,
            log_space_maxw: 1e5,
            log_space_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("telegrapher_alpha_gamma", RegionConfiguration {
            name: "telegrapher_alpha_gamma",
            system: telegrapher_alpha_gamma::SYSTEM,
            limits: Limits { p1_min: 0.2, p1_max: 0.9, p2_min: 0.2, p2_max: 0.9 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (0.40, 0.40),
                (0.70, 0.70),
                (0.86, 0.86),
                (0.89, 0.89),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e7,
                steps: 10_000,
                },
            delta: Delta::Abs(0.00005),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-3,
            log_space_maxw: 1e7,
            log_space_steps: 1_000,
            lin_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("telegrapher_alpha_gamma_single_region", RegionConfiguration {
            name: "telegrapher_alpha_gamma_single_region",
            system: telegrapher_alpha_gamma::SYSTEM,
            limits: Limits { p1_min: 0.2, p1_max: 0.9, p2_min: 0.2, p2_max: 0.9 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (0.70, 0.70),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e7,
                steps: 10_000,
                },
            delta: Delta::Abs(0.00005),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-3,
            log_space_maxw: 1e7,
            log_space_steps: 1_000,
            lin_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("semi_infinite_rod", RegionConfiguration {
            name: "semi_infinite_rod",
            system: semi_infinite_rod::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 7e4 },
            safeguard: 0.90,
            check_obsoletion: true,
            origins: vec![
                (20.0, 1e4),
                (60.0, 5e4),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(0.1),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-2,
            log_space_maxw: 1e-1,
            log_space_steps: 1_000,
            lin_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("finite_rod", RegionConfiguration {
            name: "finite_rod",
            system: finite_rod::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 7e4 },
            safeguard: 0.9,
            check_obsoletion: true,
            origins: vec![
                (25.0, 15_000.0),
                (60.0, 50_000.0),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e3,
                steps: 10_000,
                },
            delta: Delta::Abs(0.1),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-3,
            log_space_maxw: 1e4,
            log_space_steps: 1_000,
            lin_steps: 1_000,
            max_iter: Option::None,
        });

        configs.insert("telegrapher_standard", RegionConfiguration {
            name: "telegrapher_standard",
            system: telegrapher_standard::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 0.2, p2_min: 1.0, p2_max: 1.5 },
            safeguard: 0.9,
            check_obsoletion: true,
            origins: vec![
                (0.1, 1.2),
                (0.125, 1.41),
                (0.175, 1.48),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e4,
                steps: 10_000,
                },
            delta: Delta::Abs(1e-5),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e1,
            log_space_maxw: 1e4,
            log_space_steps: 1_000,
            lin_steps: 2_000,
            max_iter: Option::None,
        });

        configs.insert("pde_complex_instructional", RegionConfiguration {
            name: "pde_complex_instructional",
            system: pde_complex_tau_sigma::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
            safeguard: 1.0,
            check_obsoletion: false,
            origins: vec![
                ( 1.0,  1.0),
                (10.0, 10.0),
                (18.0,  2.0),
                (19.0,  0.15),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 1_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 1_000,
            max_iter: Option::Some(12),
        });

        configs.insert("test_configuration", RegionConfiguration {
            name: "test_configuration",
            system: test_system::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 1.0, p2_min: 0.0, p2_max: 1.0 },
            safeguard: 0.9,
            check_obsoletion: false,
            origins: vec![
                (0.1, 0.1),
                (0.9, 0.9),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e5,
                steps: 1_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            lin_steps: 1_000,
            log_space_minw: 1e-3,
            log_space_maxw: 1e5,
            log_space_steps: 10_000,
            max_iter: Option::None,
        });

        configs.insert("telegrapher_alpha_gamma_instructional", RegionConfiguration {
            name: "telegrapher_alpha_gamma_instructional",
            system: telegrapher_alpha_gamma::SYSTEM,
            limits: Limits { p1_min: 0.2, p1_max: 0.9, p2_min: 0.2, p2_max: 0.9 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (0.40, 0.40),
                (0.70, 0.70),
                (0.86, 0.86),
                (0.89, 0.89),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e7,
                steps: 10_000,
                },
            delta: Delta::Abs(0.00005),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-3,
            log_space_maxw: 1e7,
            log_space_steps: 1_000,
            lin_steps: 1_000,
            max_iter: Option::Some(12),
        });
        
        configs.insert("ln_system1", RegionConfiguration {
            name: "ln_system1",
            system: ln_system1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (0.9, 0.9),
                (4.0, 5.5),
                (13.0, 12.0),
                (18.5, 18.0),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e3,
                steps: 10_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-4,
            log_space_maxw: 1e3,
            log_space_steps: 10_000,
            lin_steps: 10_000,
            max_iter: Option::None,
        });

        configs.insert("ln_system1_negative", RegionConfiguration {
            name: "ln_system1_negative",
            system: ln_system1::SYSTEM,
            limits: Limits { p1_min: -20.0, p1_max: 20.0, p2_min: -20.0, p2_max: 20.0 },
            // limits: Limits { p1_min: -20.0, p1_max: -6.0, p2_min: -2.0, p2_max: 2.0 },
            safeguard: 0.85,
            check_obsoletion: true,
            origins: vec![
                // // Region 1
                (-0.5, 1.1),
                // // Region 2
                (-5.0, 10.0),
                // // Region 3
                (13.0, 12.0),
                // // Region 4
                (18.5, 18.0),
                // // Region 5
                (3.0, -6.0),
                // // Region 6
                (-16.0, -14.0),
                // // Region 7
                (-16.0, 8.0),
                // Region 8
                (-6.0, -10.0),
                // Region 9
                (18.0, -4.0),
                // Region 10
                (-19.8, 1.0),
                // Fake region
                // (-15.0, -1.0)
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e3,
                steps: 10_000,
                },
            delta: Delta::Abs(0.02),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-13,
            log_space_maxw: 1e2,
            log_space_steps: 200_000,
            lin_steps: 3_000_000,
            max_iter: Option::None,
        });

        configs.insert("dopid2", RegionConfiguration {
            name: "dopid2",
            system: dopid2::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 5.0 },
            safeguard: 1.0,
            check_obsoletion: true,
            origins: vec![
                (20.0, 0.1),
                // (50.0, 0.2),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e3,
                steps: 10_000,
                },
            delta: Delta::Abs(1e-3),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-3,
            log_space_maxw: 1e8,
            log_space_steps: 10_000,
            lin_steps: 10_000,
            max_iter: Option::None,
        });

        configs.insert("dopid3", RegionConfiguration {
            name: "dopid3",
            system: dopid3::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 10.0, p2_min: 0.0, p2_max: 10.0 },
            safeguard: 0.95,
            check_obsoletion: true,
            origins: vec![
                (1.0, 1.0),
                (3.0, 3.0),
                (5.5, 5.5),
                (6.8, 6.8),
                (8.2, 8.2),
                (9.3, 9.3),
                ],
            contour_conf: ContourConfiguration {
                w_min: 1e-3,
                w_max: 1e3,
                steps: 10_000,
                },
            delta: Delta::Abs(5.0 * 1e-4),
            spawn_count: 32,
            enforce_limits: false,
            log_space_minw: 1e-1,
            log_space_maxw: 1e2,
            log_space_steps: 10_000,
            lin_steps: 10_000,
            max_iter: Option::None,
        });

        configs
    };
}
