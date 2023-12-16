/*
 * The dopid1 system is used only for NU purposes.
 * It is not possible to run the data/figure algorithms on dopid2,
 * since the system violates assumption #5.
 */
use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let vp = p.0;
    let nu = p.1;
    let s_ln = s.ln();

    let term1 = s * s_ln.powu(2) * (s+1.0).powf(nu);
    let term2 = (vp*s-1.0) * s_ln;
    let term3 = (1.0-vp) * (s-1.0);

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


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let _vp = origin.0;
    let nu = origin.1;
    let nu_max = nu + eps;

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();

            let denom1 = s * s_ln - s + 1.0;
            let denom2 = s * s_ln.powi(2) * (s+1.0).ln() * (s+1.0).powf(nu_max);
            let denom = (denom1.norm().powi(2) + denom2.norm().powi(2)).sqrt();

            num / denom
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let vp = origin.0;
    let nu = origin.1;
    let nu_max = nu + eps;

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();
            
            let term1 = s * s_ln.powu(2) * (s+1.0).powf(nu);
            let term2 = (vp*s-1.0) * s_ln;
            let term3 = (1.0-vp) * (s-1.0);

            let num = (term1 + term2 + term3).norm();

            let denom1 = s * s_ln - s + 1.0;
            let denom2 = s * s_ln.powu(2) * (s+1.0).ln() * (s+1.0).powf(nu_max);
            let denom = (denom1.norm().powi(2) + denom2.norm().powi(2)).sqrt();

            num / denom
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "dopid2",
    f_complex,
    parameters: (r"v_p", r"\nu"),
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
    line_denominator: Option::None,
    region_denominator: Option::None,
};