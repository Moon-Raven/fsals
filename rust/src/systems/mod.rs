pub mod retarded1;
pub mod distributed_delay1;
pub mod semi_infinite_rod;
pub mod pde_complex_k_sigma;
pub mod pde_complex_tau_sigma;

use log::{debug, info, warn, error};
use num::complex::Complex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::types::{Comp, Par, System};


/* A global collection of all systems */
lazy_static! {
    pub static ref SYSTEMS: HashMap<&'static str, System> = {
        let mut _systems = HashMap::new();

        // systems.insert("retarded1", retarded1::SYSTEM);
        // systems.insert("distributed_delay1", retarded1::SYSTEM);
        // systems.insert("semi_infinite_rod", retarded1::SYSTEM);
        // systems.insert("pde_complex_k_sigma", retarded1::SYSTEM);
        // systems.insert("pde_complex_k_sigma", retarded1::SYSTEM);

        _systems
    };
}