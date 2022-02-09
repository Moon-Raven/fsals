use crate::types::{Comp, Par, System};


const LAMBDA: f64 = 237.0;
const SIGMA: f64 = 98.8 * 1e-6;
const X: f64 = 0.15;
const L: f64 = 0.20;
const LMX: f64 = L - X;


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    let root = Comp::sqrt(s / SIGMA);

    let term11 = LAMBDA * root;
    let term12 = Comp::cosh(L*root);
    let term1 = term11 * term12;

    let term21 = k * Comp::exp(-s*tau);
    let term22 = Comp::sinh((LMX)*root);
    let term2 = term21 * term22;

    term1 + term2
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    let k = p.1;
    let k_start = k + th_min * c1;
    let k_end = k + th_max * c1;
    let k_max = f64::max(k_start, k_end);


    let t1 = (LMX * (Comp::new(0.0, w) / SIGMA).sqrt()).sinh().norm();
    let t2 = (c2.powi(2) + (c1 * k_max * w).powi(2)).sqrt();

    t1 * t2
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let k = origin.1;
    let k_max = k + eps;
    let k_max_powi = k_max.powi(2);
    let lmxoss: f64 = LMX / SIGMA.sqrt();
    let coeff1: Comp = lmxoss * Comp::new(1.0, 1.0) / f64::sqrt(2.0);

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let t1 = (coeff1 * w.sqrt()).sinh().norm();
            let t2 = (k_max_powi * w.powi(2) + 1.0).sqrt();
            let denom = t1 * t2;
            num / denom
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let lmxoss: f64 = LMX / SIGMA.sqrt();
    let coeff1: Comp = lmxoss * Comp::new(1.0, 1.0) / f64::sqrt(2.0);

    let tau = origin.0;
    let k = origin.1;
    let k_max = k + eps;
    let k_max_powi = k_max.powi(2);
    let z = Comp::new(1.0, 1.0) / (2.0 * SIGMA).sqrt();

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let w_sqrt = w.sqrt();
            let z_w_sqrt = w_sqrt * z;
            let term1 = LAMBDA * z_w_sqrt * (L * z_w_sqrt).cosh();
            let term2 = k * Comp::exp(-Comp::new(0.0, *w) * tau) * (LMX * z_w_sqrt).sinh();
            let num = (term1 + term2).norm();

            let t1 = (coeff1 * w_sqrt).sinh().norm();
            let t2 = (k_max_powi * w.powi(2) + 1.0).sqrt();
            let denom = t1 * t2;
            num / denom
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "finite_rod",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};