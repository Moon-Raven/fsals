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
    let root = if sigma_abmn == 0.0 {
        f64::INFINITY
    }
    else {
        f64::sqrt(w/sigma_abmn)
    };

    let x1 = c1.abs() + c2.abs() * k_abmx * root;

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


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let safeguard = 1e-20;
    let k = origin.0;
    let sigma = origin.1;
    let kmax = k + eps;
    let sigma_min = f64::max(sigma - eps, safeguard);
    let coeff1 = -(2.0*sigma_min).sqrt()/2.0;
    let prefix_g_sigma = 0.5 * kmax / sigma_min.sqrt();
    let coeff2 = -(sigma_min/2.0).sqrt();

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let w_sqrt = w.sqrt();
            let gradient_k = (w_sqrt * coeff1).exp();
            let gradient_sigma = prefix_g_sigma * w_sqrt * (coeff2 * w_sqrt).exp();
            num / (f64::sqrt(gradient_k.powi(2) + gradient_sigma.powi(2)))
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let safeguard = 1e-20;
    let k = origin.0;
    let sigma = origin.1;
    let kmax = k + eps;
    let sigma_min = f64::max(sigma - eps, safeguard);
    let coeff1 = -(2.0*sigma_min).sqrt()/2.0;
    let prefix_g_sigma = 0.5 * kmax / sigma_min.sqrt();
    let coeff2 = -(sigma_min/2.0).sqrt();

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let s = Comp::new(0.0, *w);
            let num = f_complex(s, origin).norm();
            let w_sqrt = w.sqrt();
            let gradient_k = (w_sqrt * coeff1).exp();
            let gradient_sigma = prefix_g_sigma * w_sqrt * (coeff2 * w_sqrt).exp();
            num / (f64::sqrt(gradient_k.powi(2) + gradient_sigma.powi(2)))
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "pde_complex_k_sigma",
    f_complex,
    parameters: (r"k", r"\sigma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::Some(region_denominator),
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};