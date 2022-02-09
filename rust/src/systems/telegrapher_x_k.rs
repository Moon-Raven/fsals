use crate::types::{Comp, Par, System};

const ALPHA: f64 = 5.0 / 6.0;
const BETA: f64 = 2.0 / 3.0;
const GAMMA: f64 = 2.0 / 3.0;
const A: f64 = 4.160167646103808;
const B: f64 = 3.5;
const SAFEGUARD: f64 = 1e-30;


fn f_complex(s: Comp, p: Par) -> Comp {
    let x = p.0;
    let k = p.1;

    let numerator = (s.powf(ALPHA+BETA) + A*s.powf(ALPHA) + B) * (s.powf(GAMMA) + 1.0);
    let denominator = s.powf(BETA) + A;
    let psi = numerator / denominator;
    let e = Comp::exp(-x*Comp::sqrt(psi));

    1.0 + k*e
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    let (x0, k0) = (p.0, p.1);
    let x_min = f64::max(f64::min(x0 + c1*th_min, x0 + c1*th_max), SAFEGUARD);

    let s = Comp::new(0.0, w);
    let s_alpha = Comp::powf(s, ALPHA);
    let s_beta = Comp::powf(s, BETA);
    let num1 = s_alpha * s_beta + A * s_alpha + B;
    let num2 = Comp::powf(s, GAMMA) + 1.0;
    let numerator = num1 * num2;
    let denominator = s_beta + A;
    let psi = numerator / denominator;
    let r = psi.sqrt();
    let t1 = (-x_min * r.re).exp();
    let t2 = (r * c1 * k0).norm() + c2.abs() + (c1 * c2 * r).norm() * th_max;
    t1 * t2
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let x_min = origin.0 - eps;
    let k_max = origin.1 + eps;
    let kmax_powi = k_max.powi(2);
    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let s = Comp::new(0.0, *w);
            let s_alpha = Comp::powf(s, ALPHA);
            let s_beta = Comp::powf(s, BETA);
            let num1 = s_alpha * s_beta + A * s_alpha + B;
            let num2 = Comp::powf(s, GAMMA) + 1.0;
            let numerator = num1 * num2;
            let denominator = s_beta + A;
            let psi = numerator / denominator;
            let r = psi.sqrt();
            let t = (-x_min * r.re).exp();
            let denominator = t * (1.0 + kmax_powi * psi.norm()).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let x_min = origin.0 - eps;
    let k_max = origin.1 + eps;
    let kmax_powi = k_max.powi(2);
    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let s = Comp::new(0.0, *w);
            let s_alpha = Comp::powf(s, ALPHA);
            let s_beta = Comp::powf(s, BETA);
            let num1 = s_alpha * s_beta + A * s_alpha + B;
            let num2 = Comp::powf(s, GAMMA) + 1.0;
            let numerator = num1 * num2;
            let denominator = s_beta + A;
            let psi = numerator / denominator;
            let r = psi.sqrt();
            let t = (-x_min * r.re).exp();
            let denominator = t * (1.0 + kmax_powi * psi.norm()).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "telegrapher_x_k",
    f_complex,
    parameters: (r"x", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};