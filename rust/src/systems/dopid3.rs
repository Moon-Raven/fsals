use crate::types::{Comp, Par, System};

const NU: u32 = 2;
const VI: u32 = 1;

fn f_complex(s: Comp, p: Par) -> Comp {
    let vp = p.0;
    let tau = p.1;
    let s_ln = s.ln();

    let term1 = s * s_ln.powu(2) * (s+1.0).powu(VI);
    let term2_1 = (vp*s-(VI as f64)) * s_ln;
    let term2_2 = ((VI as f64)-vp) * (s-1.0);
    let term2 = (-s*tau).exp() * (term2_1 + term2_2);

    term1 + term2
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
    _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let vp = origin.0;
    let _tau = origin.1;

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();

            let denom1 = (s * s_ln - s + 1.0).norm();
            let denom2 = w * ((vp*s - (VI as f64)) * s_ln + (((VI as f64)-vp)) * (s-1.0)).norm();
            let denom = (denom1.powi(2) + denom2.powi(2)).sqrt();

            num / denom
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    _eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let vp = origin.0;
    let tau = origin.1;

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();
            
            let term1 = s * s_ln.powu(2) * (s+1.0).powu(VI);
            let term2_1 = (vp*s-(VI as f64)) * s_ln;
            let term2_2 = ((VI as f64)-vp) * (s-1.0);
            let term2 = (-s*tau).exp() * (term2_1 + term2_2);
            
            let num = (term1 + term2).norm();

            let denom1 = (s * s_ln - s + 1.0).norm();
            let denom2 = w * ((vp*s - (VI as f64)) * s_ln + (((VI as f64)-vp)) * (s-1.0)).norm();
            let denom = (denom1.powi(2) + denom2.powi(2)).sqrt();

            num / denom
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "dopid3",
    f_complex,
    parameters: (r"v_p", r"\tau"),
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
    line_denominator: Option::None,
    region_denominator: Option::None,
};