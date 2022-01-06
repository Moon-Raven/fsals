use crate::types::{Comp, Par, System};


const LAMBDA: f64 = 237.0;
const SIGMA: f64 = 98.8 * 1e-6;
const X: f64 = 0.15;
const L: f64 = 0.20;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let root = Comp::sqrt(s / SIGMA);

    let term11 = LAMBDA * root;
    let term12 = Comp::cosh(L*root);
    let term1 = term11 * term12;

    let term21 = k * Comp::exp(-s*tau);
    let term22 = Comp::sinh((L-X)*root);
    let term2 = term21 * term22;

    term1 + term2
}


pub const SYSTEM: System = System {
    name: "finite_rod",
    f_complex,
    parameters: (r"\tau", r"k")
};