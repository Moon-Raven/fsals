use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    s.powi(2) + s*k + 1.0 - (-tau*(s+k)).exp()
}

pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let p1min = origin.0 - eps;
    let p1max = origin.0 + eps;
    let p2min = origin.1 - eps;
    let p2max = origin.1 + eps;

    let exp_term = (-p1min * p2min).exp();
    let p2max_powi = p2max.powi(2);
    let p1max_times_exp_term = p1max * exp_term;

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let gradient_p1 = exp_term * f64::sqrt(w.powi(2) + p2max_powi);
            let gradient_p2 = w + p1max_times_exp_term;
            num / (f64::sqrt(gradient_p1.powi(2) + gradient_p2.powi(2)))
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let p1min = origin.0 - eps;
    let p1max = origin.0 + eps;
    let p2min = origin.1 - eps;
    let p2max = origin.1 + eps;

    let exp_term = (-p1min * p2min).exp();
    let p2max_powi = p2max.powi(2);
    let p1max_times_exp_term = p1max * exp_term;

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let s = Comp::new(0.0, *w);
            let tau = origin.0;
            let k = origin.1;
            let num = (s.powi(2) + s*k + 1.0 - (-tau*(s+k)).exp()).norm();
            let gradient_p1 = exp_term * f64::sqrt(w.powi(2) + p2max_powi);
            let gradient_p2 = w + p1max_times_exp_term;
            num / (f64::sqrt(gradient_p1.powi(2) + gradient_p2.powi(2)))
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "distributed_delay1",
    f_complex,
    parameters: (r"\tau", r"k"),
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
    line_denominator: Option::None,
    region_denominator: Option::None,
};