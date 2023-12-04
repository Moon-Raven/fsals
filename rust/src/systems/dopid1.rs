use crate::types::{Comp, Par, System};

const PROCESS_ORDER: u32 = 1;

fn f_complex(s: Comp, p: Par) -> Comp {
    let vp = p.0;
    let vi = p.1;
    let s_ln = s.ln();

    let term1 = s * s_ln.powu(2) * (s+1.0).powu(PROCESS_ORDER);
    let term2 = (vp*s-vi) * s_ln;
    let term3 = (vi-vp) * (s-1.0);

    term1 + term2 + term3
}


// fn line_denominator(w: f64, _p: Par, angle: f64, _th_min: f64, _th_max: f64) -> f64 {
    // let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    // let s = Comp::new(0.0, w);
    // let s_ln = s.ln();

    // let term1 = -s * s_ln * c1;
    // let term2 = s * (c1 - c2);
    // let term3 = c2 * s_ln;
    // let term4 = c2 - c1;

    // (term1 + term2 + term3 + term4).norm()
//     0.0
// }


// pub fn region_fraction_precalculated_numerator<'a>(
//     numerator: &'a [f64],
//     w_logspace: &'a [f64],
//     _origin: Par,
//     _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
// {
//     let fraction_iter = numerator
//         .iter()
//         .zip(w_logspace.iter()).map(move |(num, w)| {
//             let s = Comp::new(0.0, *w);
//             let s_ln = s.ln();
//             let s_minus_1 = s - 1.0;

//             let term1 = (-s * s_ln + s_minus_1).norm();
//             let term2 = (s_ln - s_minus_1).norm();
//             let denom = (term1.powi(2) + term2.powi(2)).sqrt();

//             num / denom
//     });

//     Box::new(fraction_iter)
// }


// pub fn region_fraction<'a>(
//     w_linspace: &'a [f64],
//     origin: Par,
//     _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
// {
//     let fraction_iter = w_linspace
//         .iter()
//         .map(move |w| {
//             let s = Comp::new(0.0, *w);

//             let kp = origin.0;
//             let ki = origin.1;
//             let s_ln = s.ln();
//             let kp_s = kp * s;

//             let term1 = s_ln.powi(2) * s * (s+1.0);
//             let bracket = s_ln * (-kp_s + ki) + (-ki*s + kp_s + ki - kp);
//             let term2 = (-s*TAU).exp() * bracket;
//             let num = (term1 - term2).norm();

//             let term1 = (-s * s_ln + s - 1.0).norm();
//             let term2 = (s_ln - s + 1.0).norm();
//             let denom = (term1.powi(2) + term2.powi(2)).sqrt();

//             num / denom
//     });

//     Box::new(fraction_iter)
// }


pub const SYSTEM: System = System {
    name: "dopid1",
    f_complex,
    parameters: (r"v_p", r"v_i"),
    region_fraction_precalculated_numerator: Option::None,
    region_fraction: Option::None,
    line_denominator: Option::None,
    region_denominator: Option::None,
};