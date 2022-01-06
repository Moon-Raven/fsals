use crate::types::{Comp, Par, System};


const LAMBDA: f64 = 237.0;
const SIGMA: f64 = 98.8 * 1e-6;
const X: f64 = 0.15;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let term1 = LAMBDA * (s / SIGMA).sqrt();
    let term2 = k * (-X * (s/SIGMA).sqrt()).exp() * (-s*tau).exp();

    term1 + term2
}


pub const SYSTEM: System = System {
    name: "semi_infinite_rod",
    f_complex,
    parameters: (r"\tau", r"k")
};