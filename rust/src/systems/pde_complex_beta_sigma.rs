use crate::types::{Comp, Par, System};

const K: f64 = 3.0;
const L: u32 = 3;

fn f_complex(s: Comp, p: Par) -> Comp {
    let beta = p.0;
    let sigma = p.1;

    s.powf(beta) + Comp::exp(-Comp::sqrt(sigma*s))
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let c1 = f64::cos(angle);
    let c2 = f64::sin(angle);
    let beta = p.0;
    let sigma = p.1;

    let beta_min = f64::min(beta+c1*th_min, beta+c1*th_max);
    let sigma_min = f64::min(sigma+c2*th_min, sigma+c2*th_max);

    let beta_max = f64::max(beta+c1*th_min, beta+c1*th_max);

    let beta_worst_max = if w <= 1.0 {beta_min} else {beta_max};
    let beta_worst_min = if w <= 1.0 {beta_max} else {beta_min};

    // Helper variables
    let ln_pref = (Comp::new(0.0, w)).ln().norm();
    let exp_part = (-f64::sqrt(2.0)/2.0 * f64::sqrt(sigma_min * w)).exp();
    let a = [1, 3, 3, 1]; // Assuming L = 3

    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..(L+1) {
        let exponent = ((i*L) as f64 + beta_worst_max * (L-i) as f64) / L as f64;
        sum1 += a[i as usize] as f64 * (1-i/L) as f64 * w.powf(exponent);
    }

    for i in 0..(L+1) {
        let exponent = i as f64 * (L as f64-beta_worst_min) / L as f64;
        sum2 += a[i as usize] as f64 * (i/L) as f64 * w.powf(exponent);
    }
    let derivative_beta = ln_pref * (sum1 + K * exp_part * sum2);

    let mut sum_sigma = 0.0;
    for i in 0..(L+1) {
        let exponent = (L as f64 -beta_worst_min) * (i/L) as f64;
        sum_sigma += a[i as usize] as f64 * w.powf(exponent);
    }

    let derivative_sigma = K/2.0 * exp_part * f64::sqrt(w/sigma_min) * sum_sigma;

    c1.abs() * derivative_beta + c2.abs() * derivative_sigma
}


fn region_denominator(w: f64, origin: Par, eps: f64) -> f64 {
    // let k = origin.0;
    // let sigma = origin.1;

    // let kmax = k + eps;
    // let sigma_min = sigma - eps;

    // let gradient_k = (-f64::sqrt(2.0)/2.0 * f64::sqrt(w*sigma_min)).exp();
    // let gradient_sigma = 0.5*kmax*f64::sqrt(w/sigma_min)*(-f64::sqrt(w*sigma_min/2.0)).exp();
    // f64::sqrt(gradient_k.powi(2) + gradient_sigma.powi(2))

    let beta = origin.0;
    let sigma = origin.1;
    let beta_min = beta - eps;
    let beta_max = beta + eps;
    let sigma_min = sigma - eps;

    let beta_worst_max = if w <= 1.0 {beta_min} else {beta_max};
    let beta_worst_min = if w <= 1.0 {beta_max} else {beta_min};

    // Helper variables
    let ln_pref = Comp::new(0.0, w).ln().norm();
    let exp_part = (-f64::sqrt(2.0)/2.0 * f64::sqrt(sigma_min * w)).exp();

    let a = [1, 3, 3, 1]; // Assuming L = 3

    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    // for i in 0..(L+1) {
    //     let exponent = ((i*L) as f64 + beta_worst_max * (L-i) as f64) / L as f64;
    //     sum1 += a[i as usize] as f64 * (1-i/L) as f64 * w.powf(exponent);
    // }

    // for i in range(L+1):
    //     exponent = (i*L + beta_worst_max*(L-i)) / L
    //     sum1 += a[i] * (1-i/L) * pw(w, exponent)
    // for i in range(L+1):
    //     exponent = i * (L-beta_worst_min) / L
    //     sum2 += a[i] * i/L * pw(w, exponent)
    // derivative_beta = ln_pref * (sum1 + k*exp_part*sum2)

    // sum_sigma = 0
    // for i in range(L+1):
    //     exponent = (L-beta_worst_min) * i/L
    //     sum_sigma += a[i] * pw(w, exponent)
    // derivative_sigma = k/2 * exp_part * sqrt(w/sigma_min) * sum_sigma

    // vectors = np.vstack((derivative_beta, derivative_sigma))
    // result = np.linalg.norm(vectors, ord=p, axis=0)
    // return result
}


pub const SYSTEM: System = System {
    name: "pde_complex_beta_sigma",
    f_complex,
    parameters: (r"\beta", r"\sigma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::Some(region_denominator),
};