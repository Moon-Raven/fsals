use crate::types::{Comp, Par, System};


const LAMBDA: f64 = 237.0;
const SIGMA: f64 = 98.8 * 1e-6;
const X: f64 = 0.15;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let root = Comp::sqrt(s / SIGMA);
    let term1 = LAMBDA * root;
    let term2 = k * Comp::exp(-X * root) * Comp::exp(-s*tau);

    term1 + term2
}


pub const SYSTEM: System = System {
    name: "semi_infinite_rod",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::None,
    region_denominator: Option::None,
};