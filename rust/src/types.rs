use num::complex::Complex64;
use serde::Serialize;


/* Convenience aliases */
pub type Par = (f64, f64);
pub type Comp = num::complex::Complex64;


/* Rectangular 2D limits */
#[derive(Debug, Serialize)]
pub struct Limits {
    pub p1_min: f64,
    pub p1_max: f64,
    pub p2_min: f64,
    pub p2_max: f64,
}


/* A LTI system represented by its characteristic function in Laplace domain */
pub struct System {
    pub name: &'static str,
    pub f_complex: fn(Comp, Par) -> Comp,
    pub parameters: (&'static str, &'static str),
}