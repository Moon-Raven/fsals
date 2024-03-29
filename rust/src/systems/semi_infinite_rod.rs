use crate::types::{Comp, Par, System};


const LAMBDA: f64 = 237.0;
const SIGMA: f64 = 98.8 * 1e-6;
const X: f64 = 0.15;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let root = Comp::sqrt(s / SIGMA);
    let term1 = LAMBDA * root;
    let term2 = k * Comp::exp(-X * root - s*tau);

    term1 + term2
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let k_max = origin.1 + eps;
    let k_powi = k_max.powi(2);
    let helper = -X / (2.0 * SIGMA).sqrt();

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let a = (helper * w.sqrt()).exp();
            let denom = a * (1.0 + k_powi * w.powi(2)).sqrt();
            num / denom
    });

    Box::new(fraction_iter)
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let c1 = f64::cos(angle);
    let c2 = f64::sin(angle);

    let k_start = p.1 + th_min * c2;
    let k_end = p.1 + th_max * c2;
    let k_max = f64::max(k_start, k_end);

    let term1 = (-X * f64::sqrt(w/(2.0*SIGMA))).exp();
    let term2 = f64::sqrt(c2.powi(2) + w.powi(2) * c1.powi(2)*k_max.powi(2));

    term1 * term2
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let k_max = origin.1 + eps;
    let k_powi = k_max.powi(2);
    let helper = -X / (2.0 * SIGMA).sqrt();

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let s = Comp::new(0.0, *w);
            let tau = origin.0;
            let k = origin.1;

            let root = Comp::sqrt(s / SIGMA);
            let term1 = LAMBDA * root;
            let term2 = k * Comp::exp(-X * root - s*tau);

            let num = (term1 + term2).norm();

            let a = (helper * w.sqrt()).exp();
            let denom = a * (1.0 + k_powi * w.powi(2)).sqrt();
            num / denom
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "semi_infinite_rod",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};