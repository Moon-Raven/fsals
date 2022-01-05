use num::complex::Complex64;
use serde::Serialize;


pub type Par = (f64, f64);
pub type Comp = num::complex::Complex64;

#[derive(Debug, Serialize)]
pub struct Limits {
    pub p1_min: f64,
    pub p1_max: f64,
    pub p2_min: f64,
    pub p2_max: f64,
}


pub struct System {
    pub name: &'static str,
    pub f_complex: fn(Comp, Par) -> Comp,
    pub parameters: (&'static str, &'static str),
}