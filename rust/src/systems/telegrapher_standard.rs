use crate::types::{Comp, Par, System};

const R: f64 = 172.24 * 1e-3;
const C: f64 = 51.57 * 1e-12;
const L: f64 = 612.9 * 1e-9;
const G: f64 = 1e-9;
const X0: f64 = 20_000.0;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let sqrt_term = Comp::sqrt((C*s + G) * (L*s + R));
    let term1 = Comp::exp(-s * tau);
    let term2 = Comp::exp(-X0 * sqrt_term);

    1.0 + k * term1 * term2
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    let (_tau0, k0) = (p.0, p.1);
    let k_max = f64::max(k0 + c2*th_min, k0 + c2*th_max);

    let s = Comp::new(0.0, w);
    let sqrt_term = Comp::sqrt((C*s + G) * (L*s + R));
    let exp_term = Comp::exp(-X0 * sqrt_term);
    let t1 = Comp::norm(exp_term);
    let t2 = f64::sqrt(c2.powi(2) + c1.powi(2) * w.powi(2) * k_max.powi(2));
    t1 * t2
}


// pub fn region_fraction_precalculated_numerator<'a>(
//     numerator: &'a [f64],
//     w_logspace: &'a [f64],
//     origin: Par,
//     eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
// {
//     let x_min = origin.0 - eps;
//     let k_max = origin.1 + eps;
//     let kmax_powi = k_max.powi(2);
//     let fraction_iter = numerator
//         .iter()
//         .zip(w_logspace.iter()).map(move |(num, w)| {
//             let s = Comp::new(0.0, *w);
//             let s_alpha = Comp::powf(s, ALPHA);
//             let s_beta = Comp::powf(s, BETA);
//             let num1 = s_alpha * s_beta + A * s_alpha + B;
//             let num2 = Comp::powf(s, GAMMA) + 1.0;
//             let numerator = num1 * num2;
//             let denominator = s_beta + A;
//             let psi = numerator / denominator;
//             let r = psi.sqrt();
//             let t = (-x_min * r.re).exp();
//             let denominator = t * (1.0 + kmax_powi * psi.norm()).sqrt();
//             num / denominator
//     });

//     Box::new(fraction_iter)
// }


// pub fn region_fraction<'a>(
//     w_linspace: &'a [f64],
//     origin: Par,
//     eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
// {
//     let x_min = origin.0 - eps;
//     let k_max = origin.1 + eps;
//     let kmax_powi = k_max.powi(2);
//     let fraction_iter = w_linspace
//         .iter()
//         .map(move |w| {
//             let num = f_complex(Comp::new(0.0, *w), origin).norm();
//             let s = Comp::new(0.0, *w);
//             let s_alpha = Comp::powf(s, ALPHA);
//             let s_beta = Comp::powf(s, BETA);
//             let num1 = s_alpha * s_beta + A * s_alpha + B;
//             let num2 = Comp::powf(s, GAMMA) + 1.0;
//             let numerator = num1 * num2;
//             let denominator = s_beta + A;
//             let psi = numerator / denominator;
//             let r = psi.sqrt();
//             let t = (-x_min * r.re).exp();
//             let denominator = t * (1.0 + kmax_powi * psi.norm()).sqrt();
//             num / denominator
//     });

//     Box::new(fraction_iter)
// }


pub const SYSTEM: System = System {
    name: "telegrapher_standard",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::None,
    region_fraction: Option::None,
};
