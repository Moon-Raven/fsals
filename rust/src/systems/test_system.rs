use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let a = p.0;
    let b = p.1;

    s.powi(2) + a*s + (a.powi(2) + b - 1.0)
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    let (a0, _b0) = (p.0, p.1);
    let a_max = f64::max(a0 + c1*th_min, a0 + c1*th_max);
    let a_max_powi = a_max.powi(2);

    (w.powi(2) + 4.0 * a_max_powi).sqrt() * c1.abs() + c2.abs()
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let a_max = origin.0 + eps;
    let amax_powi = a_max.powi(2);
    let amax_powi_times_4 = amax_powi * 4.0;
    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let derivative_a = (w.powi(2) + amax_powi_times_4).sqrt();
            let denominator = (derivative_a.powi(2) + 1.0).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let a_max = origin.0 + eps;
    let amax_powi = a_max.powi(2);
    let amax_powi_times_4 = amax_powi * 4.0;

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let derivative_a = (w.powi(2) + amax_powi_times_4).sqrt();
            let denominator = (derivative_a.powi(2) + 1.0).sqrt();
            num / denominator
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "test_system",
    f_complex,
    parameters: ("a", "b"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};
