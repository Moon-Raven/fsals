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
use crate::systems::telegrapher_standard;
use crate::systems::test_system;

use crate::types::{Limits, System};


pub struct ContourConfiguration {
    pub w_min: f64,
    pub w_max: f64,
    pub steps: usize,
}


pub struct NuConfiguration {
    pub name: &'static str,
    pub system: System,
    pub limits: Limits,
    pub grid_step: usize,
    pub contour_conf: ContourConfiguration,
}


/* Global collection of all nu configurations */
lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, NuConfiguration> = {
        let mut configs = HashMap::new();

        configs.insert(
            "retarded1",
            NuConfiguration {
                name: "retarded1",
                system: retarded1::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 2.6,
                    p2_min: 0.0,
                    p2_max: 3.3,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "distributed_delay1",
            NuConfiguration {
                name: "distributed_delay1",
                system: distributed_delay1::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 20.0,
                    p2_min: 0.0,
                    p2_max: 0.4,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "semi_infinite_rod",
            NuConfiguration {
                name: "semi_infinite_rod",
                system: semi_infinite_rod::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 100.0,
                    p2_min: 0.0,
                    p2_max: 70_000.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "finite_rod",
            NuConfiguration {
                name: "finite_rod",
                system: finite_rod::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e2,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 100.0,
                    p2_min: 0.0,
                    p2_max: 70_000.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "pde_complex_k_sigma",
            NuConfiguration {
                name: "pde_complex_k_sigma",
                system: pde_complex_k_sigma::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 20.0,
                    p2_min: 0.0,
                    p2_max: 20.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "pde_complex_tau_sigma",
            NuConfiguration {
                name: "pde_complex_tau_sigma",
                system: pde_complex_tau_sigma::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 20.0,
                    p2_min: 0.0,
                    p2_max: 20.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "pde_complex_beta_sigma",
            NuConfiguration {
                name: "pde_complex_beta_sigma",
                system: pde_complex_beta_sigma::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 2.0,
                    p2_min: 0.0,
                    p2_max: 2.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "telegrapher_x_k",
            NuConfiguration {
                name: "telegrapher_x_k",
                system: telegrapher_x_k::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 4.0,
                    p2_min: 4.0,
                    p2_max: 8.0,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "telegrapher_alpha_gamma",
            NuConfiguration {
                name: "telegrapher_alpha_gamma",
                system: telegrapher_alpha_gamma::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.1,
                    p1_max: 0.9,
                    p2_min: 0.1,
                    p2_max: 0.9,
                },
                grid_step: 40usize,
            },
        );

        configs.insert(
            "telegrapher_standard",
            NuConfiguration {
                name: "telegrapher_standard",
                system: telegrapher_standard::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e5,
                    steps: 10_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 0.2,
                    p2_min: 1.0,
                    p2_max: 1.5,
                },
                grid_step: 20usize,
            },
        );

        configs.insert(
            "test_system",
            NuConfiguration {
                name: "test_system",
                system: test_system::SYSTEM,
                contour_conf: ContourConfiguration {
                    w_min: 1e-3,
                    w_max: 1e2,
                    steps: 1_000usize,
                },
                limits: Limits {
                    p1_min: 0.0,
                    p1_max: 1.2,
                    p2_min: 0.0,
                    p2_max: 1.2,
                },
                grid_step: 20usize,
            },
        );

        configs
    };
}
