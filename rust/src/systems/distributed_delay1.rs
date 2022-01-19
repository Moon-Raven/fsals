use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    s.powi(2) + s*k + 1.0 - (-tau*(s+k)).exp()
}


fn line_denominator(w: f64, p: Par, angle: f64, _th_min: f64, th_max: f64) -> f64 {
    let tau0 = p.0;
    let k0 = p.1;
    let c1 = f64::cos(angle);
    let c2 = f64::sin(angle);
    let term1 = f64::abs(c2 * w);
    let term2 = f64::sqrt((tau0*c2 + k0*c1 + th_max).powi(2) + (c1*w).powi(2));
    return term1 + term2
}


fn region_denominator(w: f64, origin: Par, eps: f64) -> f64 {
    let p1min = origin.0 - eps;
    let p1max = origin.0 + eps;
    let p2min = origin.1 - eps;
    let p2max = origin.1 + eps;

    let exp_term = (-p1min * p2min).exp();

    let gradient_p1 = exp_term * f64::sqrt(w.powi(2) + p2max.powi(2));
    let gradient_p2 = w + p1max * exp_term;

    f64::sqrt(gradient_p1.powi(2) + gradient_p2.powi(2))
}


pub const SYSTEM: System = System {
    name: "distributed_delay1",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::Some(region_denominator),
};
