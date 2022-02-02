use std::iter::{Map, Zip};
use core::slice::Iter;
use serde::Serialize;


/* Convenience aliases */
pub type Par = (f64, f64);
pub type Comp = num::complex::Complex64;
pub type LineDenomFunc = fn(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64;
pub type RegionDenomFunc = fn(w: f64, p: Par, eps: f64) -> f64;
// pub type RegionFractionFunc = fn(num: &[f64], w: &[f64], origin: Par, eps: f64)
//     -> Map<Zip<Iter<f64>, Iter<f64>>, Fn(&f64, &f64) -> f64>;

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
    pub line_denominator: Option<LineDenomFunc>,
    pub region_denominator: Option<RegionDenomFunc>,
    // pub region_fraction: Option<RegionFractionFunc>,
}