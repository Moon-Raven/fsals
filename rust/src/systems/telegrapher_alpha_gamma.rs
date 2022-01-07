use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    const X: f64 = 0.8;
    const K: f64 = 30.0;
    const BETA: f64 = 2.0 / 3.0;
    let a: f64 = 2.0 * 9.0f64.powf(1.0/3.0); // Can't be const because of powf
    const B: f64 = 3.5;

    let alpha = p.0;
    let gamma = p.1;

    let numerator = (s.powf(alpha+BETA) + a*s.powf(alpha) + B) * (s.powf(gamma) + 1.0);
    let denominator = s.powf(BETA) + a;
    let psi = numerator / denominator;
    let e = Comp::exp(-X*Comp::sqrt(psi));

    1.0 + K*e
}


pub const SYSTEM: System = System {
    name: "telegrapher_alpha_gamma",
    f_complex,
    parameters: (r"\alpha", r"\gamma"),
    line_denominator: Option::None,
    region_denominator: Option::None,
};