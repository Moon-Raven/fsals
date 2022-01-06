use lazy_static::lazy_static;
use crate::systems::retarded1;
use crate::systems::distributed_delay1;
use super::NuConfiguration;
use crate::types::Limits;
use std::collections::HashMap;


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

        configs
    };
}