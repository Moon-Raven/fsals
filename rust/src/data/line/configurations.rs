use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::systems::retarded1;
use crate::systems::distributed_delay1;
use crate::systems::semi_infinite_rod;
use crate::systems::finite_rod;
use crate::systems::pde_complex_k_sigma;
use crate::systems::pde_complex_tau_sigma;
use crate::systems::pde_complex_beta_sigma;
use crate::systems::telegrapher_x_k;
use crate::systems::telegrapher_alpha_gamma;

use crate::types::{System, Limits, Par};


pub struct LineConfiguration {
    pub name: &'static str,
    pub system: System,
    pub limits: Limits,
    pub origins: Vec<Par>,
    pub ray_count: usize,
}


lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, LineConfiguration> = {
        let mut configs = HashMap::new();

        configs.insert("retarded1", LineConfiguration {
            name: "retarded1",
            system: retarded1::SYSTEM,
            limits: Limits { p1_min: 0.0, p1_max: 2.6, p2_min: 0.0, p2_max: 3.3 },
            ray_count: 8,
            origins: vec![
                (1e-2, 1e-2),
                // (1.75, 1.20),
                // (0.88, 2.73),
                // (0.20, 3.10),
                // (0.71, 3.22),
                ],
            });


        configs
    };
}