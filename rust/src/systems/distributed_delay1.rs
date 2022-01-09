use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    s*s + s*k + 1.0 - (-tau*(s+k)).exp()
}


// fn line_denominator(w: f64, _p: Par, _angle: f64, _th_min: f64, _th_max: f64) -> f64 {
//     let safeguard = 1e-10;

//     2.0*w.powi(2) + w + safeguard
// }


pub const SYSTEM: System = System {
    name: "distributed_delay1",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
};
