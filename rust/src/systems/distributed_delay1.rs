use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    s*s + s*k + 1.0 - (-tau*(s+k)).exp()
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


pub const SYSTEM: System = System {
    name: "distributed_delay1",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
};
