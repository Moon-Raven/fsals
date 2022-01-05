use num::complex::Complex64;


pub type Par = (f64, f64);
pub type Comp = num::complex::Complex64;

pub struct Limits {
    pub p1_min: f64,
    pub p1_max: f64,
    pub p2_min: f64,
    pub p2_max: f64,
}


pub struct System {
    pub name: &'static str,
    pub f_complex: fn(Comp, Par) -> Comp,
}