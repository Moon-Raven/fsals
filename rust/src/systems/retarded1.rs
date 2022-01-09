use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau1 = p.0;
    let tau2 = p.1;

    s*s + 2.0*s*(-s*tau1).exp() + (-s*tau2).exp()
}


fn line_denominator(w: f64, _p: Par, _angle: f64, _th_min: f64, _th_max: f64) -> f64 {
    let safeguard = 1e-10;

    2.0*w.powi(2) + w + safeguard
}


pub const SYSTEM: System = System {
    name: "retarded1",
    f_complex,
    parameters: (r"\tau_1", r"\tau_2"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
};