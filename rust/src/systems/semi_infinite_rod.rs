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


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    // let tau = p.0;
    // let k = p.1;
    let c1 = f64::cos(angle);
    let c2 = f64::sin(angle);

    // let tau_start = p.0 + th_min * c1;
    // let tau_end = p.0 + th_max * c1;
    let k_start = p.1 + th_min * c2;
    let k_end = p.1 + th_max * c2;
    // let tau_max = f64::max(tau_start, tau_end);
    let k_max = f64::max(k_start, k_end);

    let term1 = (-X * f64::sqrt(w/(2.0*SIGMA))).exp();
    let term2 = f64::sqrt(c2.powi(2) + w.powi(2) * c1.powi(2)*k_max.powi(2));

    term1 * term2
}


fn region_denominator(w: f64, origin: Par, eps: f64) -> f64 {
    let k_max = origin.1 + eps;

    let exp_term = (-X * f64::sqrt(w/(2.0*SIGMA))).exp();

    let gradient_tau= exp_term;
    let gradient_k = exp_term * k_max * w;

    f64::sqrt(gradient_tau.powi(2) + gradient_k.powi(2))
}


pub const SYSTEM: System = System {
    name: "semi_infinite_rod",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::Some(region_denominator),
};