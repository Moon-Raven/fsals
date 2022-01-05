use lazy_static::lazy_static;
use crate::systems::retarded1;
use super::Configuration;
use crate::types::Limits;

pub const RETARDED1: Configuration = Configuration {
     name: "retarded1",
     system: retarded1::RETARDED1,
     w_min: 1e-3,
     w_max: 1e5,
     steps: 10usize,
    //  steps: 10_000usize,
     limits: Limits { p1_min: 0.0, p1_max: 2.6, p2_min: 0.0, p2_max: 3.3 },
};