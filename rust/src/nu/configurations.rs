use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::systems::retarded1;
use crate::systems::distributed_delay1;
use crate::systems::semi_infinite_rod;
use crate::systems::pde_complex_k_sigma;
use crate::systems::pde_complex_tau_sigma;
use crate::systems::pde_complex_beta_sigma;

use super::NuConfiguration;
use crate::types::Limits;


/* Global collection of all nu configurations */
lazy_static! {
    pub static ref CONFIGURATONS: HashMap<&'static str, NuConfiguration> = {
        let mut configs = HashMap::new();

        configs.insert("retarded1",
          NuConfiguration {
               name: "retarded1",
               system: retarded1::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 2.6, p2_min: 0.0, p2_max: 3.3 },
               grid_step: 40usize,
          });

        configs.insert("distributed_delay1",
          NuConfiguration {
               name: "distributed_delay1",
               system: distributed_delay1::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 0.4 },
               grid_step: 40usize,
          });

        configs.insert("semi_infinite_rod",
          NuConfiguration {
               name: "semi_infinite_rod",
               system: semi_infinite_rod::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 100.0, p2_min: 0.0, p2_max: 70_000.0 },
               grid_step: 40usize,
          });

        configs.insert("pde_complex_k_sigma",
          NuConfiguration {
               name: "pde_complex_k_sigma",
               system: pde_complex_k_sigma::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
               grid_step: 40usize,
          });

        configs.insert("pde_complex_tau_sigma",
          NuConfiguration {
               name: "pde_complex_tau_sigma",
               system: pde_complex_tau_sigma::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 20.0, p2_min: 0.0, p2_max: 20.0 },
               grid_step: 40usize,
          });

        configs.insert("pde_complex_beta_sigma",
          NuConfiguration {
               name: "pde_complex_beta_sigma",
               system: pde_complex_beta_sigma::SYSTEM,
               w_min: 1e-3,
               w_max: 1e5,
               steps: 10_000usize,
               limits: Limits { p1_min: 0.0, p1_max: 2.0, p2_min: 0.0, p2_max: 2.0 },
               grid_step: 40usize,
          });

        configs
    };
}