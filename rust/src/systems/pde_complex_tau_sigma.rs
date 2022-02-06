use crate::types::{Comp, Par, System};

const K: f64 = 1.0;

fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let sigma = p.1;

    s + (-(sigma * s).sqrt()).exp() * (-tau * s).exp()
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let safeguard: f64 = 1e-30;
    let sigma0 = p.1;
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));

    let sigma_min = f64::max(f64::min(sigma0 + c2*th_min,  sigma0 + c2*th_max), safeguard);
    let y1 = K * (-(w*sigma_min/2.0).sqrt()).exp();
    let y2 = 0.5*(w/sigma_min).sqrt()*(c2).abs() + w*(c1).abs();

    y1 * y2
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let sigma = origin.1;
    let safeguard = 1e-30;
    let sigma_min = f64::max(sigma - eps, safeguard);
    let coeff1 = -(sigma_min/2.0).sqrt();
    let coeff2 = 1.0 / (4.0 * sigma_min);
    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let x = (coeff1 * w.sqrt()).exp();
            let denominator = K * x * (w * (w + coeff2)).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let sigma = origin.1;
    let safeguard = 1e-30;
    let sigma_min = f64::max(sigma - eps, safeguard);
    let coeff1 = -(sigma_min/2.0).sqrt();
    let coeff2 = 1.0 / (4.0 * sigma_min);

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let w_sqrt = w.sqrt();
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let x = (coeff1 * w_sqrt).exp();
            let denominator = K * x * w_sqrt * (w + coeff2).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "pde_complex_tau_sigma",
    f_complex,
    parameters: (r"\tau", r"\sigma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};