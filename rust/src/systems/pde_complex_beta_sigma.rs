use crate::types::{Comp, Par, System};

const K: f64 = 3.0;
const L: u32 = 3;

fn f_complex(s: Comp, p: Par) -> Comp {
    let beta = p.0;
    let sigma = p.1;

    let original = s.powf(beta) + K*Comp::exp(-Comp::sqrt(sigma*s));
    let augment = ((s.powf((L as f64-beta)/L as f64)) + 1.0).powi(L as i32);

    original * augment
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let safeguard = 1e-30;
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));
    let (beta, sigma) = (p.0, p.1);

    let beta_min = f64::min(beta + c1*th_min,  beta + c1*th_max);
    let sigma_min = f64::max(f64::min(sigma+c2*th_min, sigma+c2*th_max), safeguard);

    let beta_max = f64::max(beta + c1*th_min,  beta + c1*th_max);

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
        sum1 += a[i as usize] as f64 * (1.0-i as f64/L as f64) * w.powf(exponent);
    }

    for i in 0..(L+1) {
        let exponent = i as f64 * (L as f64-beta_worst_min) / L as f64;
        sum2 += a[i as usize] as f64 * (i as f64/L as f64) * w.powf(exponent);
    }
    let derivative_beta = ln_pref * (sum1 + K * exp_part * sum2);

    let mut sum_sigma = 0.0;
    for i in 0..(L+1) {
        let exponent = (L as f64 -beta_worst_min) * (i as f64 / L as f64);
        sum_sigma += a[i as usize] as f64 * w.powf(exponent);
    }

    let derivative_sigma = K/2.0 * exp_part * f64::sqrt(w/sigma_min) * sum_sigma;

    let result = c1.abs() * derivative_beta + c2.abs() * derivative_sigma;

    result
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let safeguard = 1e-30;
    let (beta, sigma) = (origin.0, origin.1);

    let beta_min = beta - eps;
    let beta_max = beta + eps;
    let sigma_min = f64::max(sigma-eps, safeguard);

    // Helper variables
    let a = [1.0, 3.0, 3.0, 1.0]; // Assuming L = 3
    let coeff1 = -f64::sqrt(2.0 * sigma_min)/2.0;

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let beta_worst_max = if *w <= 1.0 {beta_min} else {beta_max};
            let beta_worst_min = if *w <= 1.0 {beta_max} else {beta_min};

            let ln_pref = (Comp::new(0.0, *w)).ln().norm();
            let exp_part = (coeff1 * f64::sqrt(*w)).exp();

            // Check below
            let mut sum1 = 0.0;
            let mut sum2 = 0.0;

            for i in 0..(L+1) {
                let exponent = ((i*L) as f64 + beta_worst_max * (L-i) as f64) / L as f64;
                sum1 += a[i as usize] as f64 * (1.0-i as f64/L as f64) * w.powf(exponent);
            }

            for i in 0..(L+1) {
                let exponent = i as f64 * (L as f64-beta_worst_min) / L as f64;
                sum2 += a[i as usize] as f64 * (i as f64/L as f64) * w.powf(exponent);
            }
            let gradient_beta = ln_pref * (sum1 + K * exp_part * sum2);

            let mut sum_sigma = 0.0;
            for i in 0..(L+1) {
                let exponent = (L as f64 -beta_worst_min) * (i as f64 / L as f64);
                sum_sigma += a[i as usize] as f64 * w.powf(exponent);
            }

            let gradient_sigma = K/2.0 * exp_part * f64::sqrt(w/sigma_min) * sum_sigma;
            let result = num / (f64::sqrt(gradient_beta.powi(2) + gradient_sigma.powi(2)));
            result
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let safeguard = 1e-30;
    let (beta, sigma) = (origin.0, origin.1);

    let beta_min = beta - eps;
    let beta_max = beta + eps;
    let sigma_min = f64::max(sigma-eps, safeguard);

    // Helper variables
    let a = [1.0, 3.0, 3.0, 1.0]; // Assuming L = 3
    let coeff1 = -f64::sqrt(2.0 * sigma_min)/2.0;
    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let beta_worst_max = if *w <= 1.0 {beta_min} else {beta_max};
            let beta_worst_min = if *w <= 1.0 {beta_max} else {beta_min};

            let ln_pref = (Comp::new(0.0, *w)).ln().norm();
            let exp_part = (coeff1 * f64::sqrt(*w)).exp();

            // Check below
            let mut sum1 = 0.0;
            let mut sum2 = 0.0;

            for i in 0..(L+1) {
                let exponent = ((i*L) as f64 + beta_worst_max * (L-i) as f64) / L as f64;
                sum1 += a[i as usize] as f64 * (1.0-i as f64/L as f64) * w.powf(exponent);
            }

            for i in 0..(L+1) {
                let exponent = i as f64 * (L as f64-beta_worst_min) / L as f64;
                sum2 += a[i as usize] as f64 * (i as f64/L as f64) * w.powf(exponent);
            }
            let gradient_beta = ln_pref * (sum1 + K * exp_part * sum2);

            let mut sum_sigma = 0.0;
            for i in 0..(L+1) {
                let exponent = (L as f64 -beta_worst_min) * (i as f64 / L as f64);
                sum_sigma += a[i as usize] as f64 * w.powf(exponent);
            }

            let gradient_sigma = K/2.0 * exp_part * f64::sqrt(w/sigma_min) * sum_sigma;

            let result = num / (f64::sqrt(gradient_beta.powi(2) + gradient_sigma.powi(2)));
            result
    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "pde_complex_beta_sigma",
    f_complex,
    parameters: (r"\beta", r"\sigma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};


#[cfg(test)]
mod tests {
    use super::*;
    use iter_num_tools;
    use crate::types;

    fn assert_floats_eq(x: f64, y:f64, assertion_eps: f64) {
        let diff = f64::abs(x - y);
        assert!(diff <= assertion_eps);
    }


    #[test]
    fn array() {
        let log_space_minw = 1e-3;
        let log_space_maxw = 1e5;
        let log_space_steps = 100;
        let p = (1.00, 1.00);
        // let angle = std::f64::consts::PI;
        let angle = 3.691371367968007;
        // let (th_min, th_max) = (1.1, 0.3);
        let (th_min, th_max) = (0.87890625, 2.0506640625);

        let w = iter_num_tools::log_space(log_space_minw..=log_space_maxw, log_space_steps);
        let numerator: Vec<f64> = w.map(|w| f_complex(Comp::new(0.0, w), p).norm()).collect();
        let w = iter_num_tools::log_space(log_space_minw..=log_space_maxw, log_space_steps);
        let denom: Vec<f64> = w.map(|w| line_denominator(w, p, angle, th_min, th_max)).collect();
        let fraction: Vec<f64> = numerator.iter().zip(denom.iter()).map(|(n, d)| n/d).collect();

        println!("\n*** Logspaced W ***");
        let w = iter_num_tools::log_space(log_space_minw..=log_space_maxw, log_space_steps);
        for x in w {
            println!("{:100.20}", x);
        }

        println!("\n*** Numerator ***");
        for x in numerator.iter() {
            println!("{:100.20}", x);
        }

        println!("\n*** Denominator ***");
        for x in denom.iter() {
            println!("{:100.20}", x);
        }

        println!("\n*** Fraction ***");
        for x in fraction.iter() {
            println!("{:100.20}", x);
        }
    }

    #[test]
    fn single() {
        let w = 1.2;
        let p = (1.00, 1.00);
        let angle = 3.691371367968007;
        let (th_min, th_max) = (0.1, 0.3);
        let denom = line_denominator(w, p, angle, th_min, th_max);
        println!("denom({}) = {}", w, denom);
    }
}