use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau1 = p.0;
    let tau2 = p.1;

    let part0 = s.powi(2) + 1.0;
    let part1 = (s + 2.0) * (-s*tau1).exp();
    let part2 = f64::sqrt(5.0) * (-s*tau2).exp();

    part0 + part1 + part2
}


fn line_denominator(_w: f64, _p: Par, _angle: f64, _th_min: f64, _th_max: f64) -> f64 {
    // let safeguard = 1e-10;

    // 2.0*w.powi(2) + w + safeguard
    0.0
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    _origin: Par,
    _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let denom = w * ((w + 2.0).powi(2) + 5.0).sqrt();
            num / denom
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let denom = w * ((w + 2.0).powi(2) + 5.0).sqrt();
            num / denom
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "retarded2",
    f_complex,
    parameters: (r"\tau_1", r"\tau_2"),
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
    line_denominator: Option::None,
    region_denominator: Option::None,
};