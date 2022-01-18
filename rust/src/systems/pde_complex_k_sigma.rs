use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let k = p.0;
    let sigma = p.1;

    s + k * (-(sigma*s).sqrt()).exp()
}

fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let c1 = f64::cos(angle);
    let c2 = f64::sin(angle);

    let k = p.0;
    let sigma = p.1;

    let e1max = (-f64::sqrt(2.0*w)/2.0).exp();
    let k_abmx = f64::max(f64::abs(k+th_min*c1), f64::abs(k+th_max*c1));
    let sigma_abmn = f64::min((sigma+th_min*c2).abs(), (sigma+th_max*c2).abs());
    let x1 = c1.abs() + c2.abs()*k_abmx * f64::sqrt(w/sigma_abmn);

    return e1max * x1
}


fn region_denominator(w: f64, origin: Par, eps: f64) -> f64 {
    let k = origin.0;
    let sigma = origin.1;

    let kmax = k + eps;
    let sigma_min = sigma - eps;

    let gradient_k = (-f64::sqrt(2.0)/2.0 * f64::sqrt(w*sigma_min)).exp();
    let gradient_sigma = 0.5*kmax*f64::sqrt(w/sigma_min)*(-f64::sqrt(w*sigma_min/2.0)).exp();
    f64::sqrt(gradient_k.powi(2) + gradient_sigma.powi(2))
}

pub const SYSTEM: System = System {
    name: "pde_complex_k_sigma",
    f_complex,
    parameters: (r"k", r"\sigma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::Some(region_denominator),
};