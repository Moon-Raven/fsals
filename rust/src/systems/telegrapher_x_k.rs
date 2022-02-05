use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    const ALPHA: f64 = 5.0 / 6.0;
    const BETA: f64 = 2.0 / 3.0;
    const GAMMA: f64 = 2.0 / 3.0;
    let a: f64 = 2.0 * 9.0f64.powf(1.0/3.0); // Can't be const because of powf
    const B: f64 = 3.5;

    let x = p.0;
    let k = p.1;

    let numerator = (s.powf(ALPHA+BETA) + a*s.powf(ALPHA) + B) * (s.powf(GAMMA) + 1.0);
    let denominator = s.powf(BETA) + a;
    let psi = numerator / denominator;
    let e = Comp::exp(-x*Comp::sqrt(psi));

    1.0 + k*e
}


pub const SYSTEM: System = System {
    name: "telegrapher_x_k",
    f_complex,
    parameters: (r"x", r"k"),
    line_denominator: Option::None,
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::None,
    region_fraction: Option::None,
};