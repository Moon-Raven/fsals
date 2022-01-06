pub mod retarded1;

use log::{debug, info, warn, error};
use num::complex::Complex;
use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::types::{Comp, Par, System};


/* A global collection of all systems */
lazy_static! {
    pub static ref SYSTEMS: HashMap<&'static str, System> = {
        let mut systems = HashMap::new();

        systems.insert("retarded1", retarded1::RETARDED1);

        systems
    };
}