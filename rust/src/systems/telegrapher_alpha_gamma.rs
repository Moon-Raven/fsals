use crate::types::{Comp, Par, System};

const BETA: f64 = 2.0 / 3.0;
const A: f64 = 4.160167646103808;
const B: f64 = 3.5;
const SAFEGUARD: f64 = 1e-30;
const X: f64 = 0.8;
const K: f64 = 30.0;
const KX_HALF: f64 = K * X / 2.0;
const TOLERANCE: f64 = 1e-8;

fn f_complex(s: Comp, p: Par) -> Comp {

    let alpha = p.0;
    let gamma = p.1;

    let numerator = (s.powf(alpha+BETA) + A*s.powf(alpha) + B) * (s.powf(gamma) + 1.0);
    let denominator = s.powf(BETA) + A;
    let psi = numerator / denominator;
    let e = Comp::exp(-X*Comp::sqrt(psi));

    1.0 + K*e
}


fn t1_rho_2_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (alpha_min, alpha_max) = (eta_min.0, eta_max.0);
    let s = Comp::new(0.0, w);
    let helper = s.powf(BETA) + A;

    if (alpha_min - alpha_max).abs() > TOLERANCE {
        let delta = helper.arg();
        let rho_2_small = delta.cos() * B;

        let point1 = s.powf(alpha_min) * helper + B;
        let point2 = s.powf(alpha_max) * helper + B;
        let side_point1 = point1.norm();
        let side_point2 = point2.norm();
        let side_mutual = (point1 - point2).norm();

        let sh = (side_point1 + side_point2 + side_mutual) / 2.0; // Heron's formula
        let area = (sh * (sh - side_point1) * (sh - side_point2) * (sh - side_mutual)).sqrt();
        let rho_2_big = 2.0 * area / side_mutual;
        let result = if w <= 1.0 { rho_2_small} else {rho_2_big};
        return result;
    }
    else {
        let result = (s.powf(alpha_min) * helper + B).norm();
        return result;
    }
}


fn t1_rho_3_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (gamma_min, _gamma_max) = (eta_min.1, eta_max.1);
    let s = Comp::new(0.0, w);

    let result_small = 1.0 + 0.0*w; // Not sure why
    let result_big = (s.powf(gamma_min)+1.0).norm();
    let result = if w<=4.0 { result_small } else { result_big }; // 4 is arbitrary

    result
}


fn t1_rho_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let s = Comp::new(0.0, w);
    let helper = Comp::powf(s, BETA) + A;

    let rho1 = (helper.powf(-1.0)).norm();
    let rho2 = t1_rho_2_bound(w, eta_min, eta_max);
    let rho3 = t1_rho_3_bound(w, eta_min, eta_max);

    rho1 * rho2 * rho3
}


fn t1_phi_2_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (_alpha_min, alpha_max) = (eta_min.0, eta_max.0);
    let s = Comp::new(0.0, w);

    let result_big = ((s.powf(alpha_max)) * ((s.powf(BETA)) + A) + B).arg();
    let j1 = Comp::new(0.0, 1.0);
    let result_w1 = (j1.powf(alpha_max) * (j1.powf(BETA) + A) + B).arg();
    let result_small = 0.0*w + result_w1; // Not sure why
    let result = if w<=1.0 { result_small } else { result_big };

    result
}


fn t1_phi_3_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (_gamma_min, gamma_max) = (eta_min.1, eta_max.1);
    let s = Comp::new(0.0, w);
    let result_big = (s.powf(gamma_max) + 1.0).arg();
    let j1 = Comp::new(0.0, 1.0);
    let result_w1 = (j1.powf(gamma_max) + 1.0).arg();
    let result_small = 0.0*w + result_w1; // Not sure why

    let result = if w<=1.0 { result_small } else { result_big };

    return result
}


fn t1_phi_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let s = Comp::new(0.0, w);
    let helper = s.powf(BETA) + A;

    let phi1 = -helper.arg();
    let phi2 = t1_phi_2_bound(w, eta_min, eta_max);
    let phi3 = t1_phi_3_bound(w, eta_min, eta_max);

    return phi1 + phi2 + phi3
}


fn t1_bound(w: f64, eta_min: (f64, f64), eta_max: (f64, f64)) -> f64 {
    let min_rho = t1_rho_bound(w, eta_min, eta_max);
    let max_phi = t1_phi_bound(w, eta_min, eta_max);

    (-X * min_rho.sqrt() * (max_phi/2.0).cos()).exp()
}


fn t2_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (alpha_min, _alpha_max) = (eta_min.0, eta_max.0);
    let (gamma_min, _gamma_max) = (eta_min.0, eta_max.0);
    let s = Comp::new(0.0, w);
    let helper = s.powf(BETA) + A;

    let p0 = helper.powf(-1.0).norm();

    let delta = helper.arg();
    let result_small_w = B * delta.cos();
    let result_big_w = (s.powf(alpha_min) * helper + B).norm();
    let p1 = if w<=1.0 { result_small_w } else { result_big_w };

    let p2_small = 1.0 + 0.0*w;
    let p2_big = (s.powf(gamma_min)+1.0).norm();
    let p2 =  if w<=4.0 { p2_small } else { p2_big }; // 4 is arbitrary

    let min_psi = p0 * p1 * p2;

    min_psi.powf(-0.5)
}


fn t3_alpha_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (alpha_min, alpha_max) = (eta_min.0, eta_max.0);
    let (gamma_min, gamma_max) = (eta_min.0, eta_max.0);
    let alpha_worst_max = if w<=1.0 { alpha_min } else { alpha_max };
    let gamma_worst_max = if w<=1.0 { gamma_min } else { gamma_max };
    let s = Comp::new(0.0, w);

    return s.ln().norm() * w.powf(alpha_worst_max) * (w.powf(gamma_worst_max) + 1.0);
}


fn t3_gamma_bound(w: f64, eta_min: Par, eta_max: Par) -> f64 {
    let (alpha_min, alpha_max) = (eta_min.0, eta_max.0);
    let (gamma_min, gamma_max) = (eta_min.0, eta_max.0);
    let alpha_worst_max = if w<=1.0 { alpha_min } else { alpha_max };
    let gamma_worst_max = if w<=1.0 { gamma_min } else { gamma_max };
    let s = Comp::new(0.0, w);
    let helper = s.powf(BETA) + A;

    let p1 = (s.ln() / helper).norm();
    let p2 = w.powf(gamma_worst_max);
    let p3 = w.powf(alpha_worst_max) * helper.norm() + B;
    return p1 * p2 * p3;
}


fn line_denominator(w: f64, p: Par, angle: f64, th_min: f64, th_max: f64) -> f64 {
    let (alpha, gamma) = (p.0, p.1);
    let (c1, c2) = (f64::cos(angle), f64::sin(angle));

    let alpha_start = alpha + th_min * c1;
    let alpha_end = alpha + th_max * c1;
    let gamma_start = gamma + th_min * c2;
    let gamma_end = gamma + th_max * c2;
    let alpha_min = f64::min(alpha_start, alpha_end);
    let gamma_min = f64::min(gamma_start, gamma_end);
    let alpha_max = f64::max(alpha_start, alpha_end);
    let gamma_max = f64::max(gamma_start, gamma_end);
    let j1 = Comp::new(0.0, 1.0);

    let s = Comp::new(0.0, w);
    let s_ln = s.ln();
    let helper = Comp::powf(s, BETA) + A;
    let helper_arg = helper.arg();
    let helper_arg_cos = helper_arg.cos();
    let inverse_helper_norm = helper.powf(-1.0).norm();
    let alpha_worst_max = if w <= 1.0 { alpha_min } else { alpha_max };
    let gamma_worst_max = if w <= 1.0 { gamma_min } else { gamma_max };
    let s_powf_alpha_min = s.powf(alpha_min);
    let s_powf_gamma_min = s.powf(gamma_min);
    let s_powf_alpha_max = s.powf(alpha_max);

    let t0 = KX_HALF;

    let max_t1 = {
        let min_rho = {
            let rho1 = inverse_helper_norm;
            let rho2 = {
                if (alpha_min - alpha_max).abs() > TOLERANCE {
                    let rho_2_small = helper_arg_cos * B;

                    let point1 = s_powf_alpha_min * helper + B;
                    let point2 = s_powf_alpha_max * helper + B;
                    let side_point1 = point1.norm();
                    let side_point2 = point2.norm();
                    let side_mutual = (point1 - point2).norm();

                    let sh = (side_point1 + side_point2 + side_mutual) / 2.0; // Heron's formula
                    let area = (sh * (sh - side_point1) * (sh - side_point2) * (sh - side_mutual)).sqrt();
                    let rho_2_big = 2.0 * area / side_mutual;
                    if w <= 1.0 { rho_2_small } else { rho_2_big }
                }
                else {
                    (s_powf_alpha_min * helper + B).norm()
                }

            };
            let rho3 = {
                let result_small = 1.0 + 0.0*w; // Not sure why
                let result_big = (s_powf_gamma_min+1.0).norm();
                if w <= 4.0 { result_small } else { result_big } // 4 is arbitrary
            };

            rho1 * rho2 * rho3
        };
        let max_phi = {
            let phi1 = -helper_arg;
            let phi2 = {
                let result_big = (s_powf_alpha_max * ((s.powf(BETA)) + A) + B).arg();
                let result_w1 = (j1.powf(alpha_max) * (j1.powf(BETA) + A) + B).arg();
                let result_small = result_w1;
                if w <= 1.0 { result_small } else { result_big }
            };
            let phi3 = {
                let result_big = (s.powf(gamma_max) + 1.0).arg();
                let result_w1 = (j1.powf(gamma_max) + 1.0).arg();
                let result_small = 0.0*w + result_w1; // Not sure why

                if w <= 1.0 { result_small } else { result_big }
            };

            phi1 + phi2 + phi3
        };

        (-X * min_rho.sqrt() * (max_phi/2.0).cos()).exp()
    };

    let max_t2 = {
        let p0 = inverse_helper_norm;

        let result_small_w = B * helper_arg_cos;
        let result_big_w = (s_powf_alpha_min * helper + B).norm();
        let p1 = if w<=1.0 { result_small_w } else { result_big_w };

        let p2_small = 1.0 + 0.0*w;
        let p2_big = (s_powf_gamma_min+1.0).norm();
        let p2 =  if w <= 4.0 { p2_small } else { p2_big }; // 4 is arbitrary

        let min_psi = p0 * p1 * p2;

        min_psi.powf(-0.5)
    };

    let max_t3_alpha = {
        s_ln.norm() * w.powf(alpha_worst_max) * (w.powf(gamma_worst_max) + 1.0)
    };

    let max_t3_gamma = {
        let p1 = (s_ln / helper).norm();
        let p2 = w.powf(gamma_worst_max);
        let p3 = w.powf(alpha_worst_max) * helper.norm() + B;
        p1 * p2 * p3
    };

    let derivative_alpha = t0 * max_t1 * max_t2 * max_t3_alpha;
    let derivative_gamma = t0 * max_t1 * max_t2 * max_t3_gamma;

    let derivative_theta = derivative_alpha * c1.abs() + derivative_gamma * c2.abs();

    return derivative_theta
}


pub fn region_fraction_precalculated_numerator<'a>(
    numerator: &'a [f64],
    w_logspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let t0 = KX_HALF;
    let (alpha, gamma) = (origin.0, origin.1);
    let alpha_min = alpha - eps;
    let gamma_min = gamma - eps;
    let alpha_max = alpha + eps;
    let gamma_max = gamma + eps;
    let j1 = Comp::new(0.0, 1.0);

    let fraction_iter = numerator
        .iter()
        .zip(w_logspace.iter()).map(move |(num, w)| {
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();
            let helper = Comp::powf(s, BETA) + A;
            let helper_arg = helper.arg();
            let helper_arg_cos = helper_arg.cos();
            let inverse_helper_norm = helper.powf(-1.0).norm();
            let alpha_worst_max = if *w <= 1.0 { alpha_min } else { alpha_max };
            let gamma_worst_max = if *w <= 1.0 { gamma_min } else { gamma_max };
            let s_powf_alpha_min = s.powf(alpha_min);
            let s_powf_gamma_min = s.powf(gamma_min);
            let s_powf_alpha_max = s.powf(alpha_max);

            let max_t1 = {
                let min_rho = {
                    let rho1 = inverse_helper_norm;
                    let rho2 = {
                        if (alpha_min - alpha_max).abs() > TOLERANCE {
                            let result = if *w <= 1.0 {
                                helper_arg_cos * B
                            } else {
                                let point1 = s_powf_alpha_min * helper + B;
                                let point2 = s_powf_alpha_max * helper + B;
                                let side_point1 = point1.norm();
                                let side_point2 = point2.norm();
                                let side_mutual = (point1 - point2).norm();

                                let sh = (side_point1 + side_point2 + side_mutual) / 2.0; // Heron's formula
                                let area = (sh * (sh - side_point1) * (sh - side_point2) * (sh - side_mutual)).sqrt();
                                2.0 * area / side_mutual
                            };
                            result
                        }
                        else {
                            let result = (s_powf_alpha_min * helper + B).norm();
                            result
                        }

                    };
                    let rho3 = {
                        if *w <= 4.0 { 1.0 } else { (s_powf_gamma_min+1.0).norm()}
                    };

                    rho1 * rho2 * rho3
                };
                let max_phi = {
                    let phi1 = -helper_arg;
                    let phi2 = {
                        if *w <= 1.0 {
                            (j1.powf(alpha_max) * (j1.powf(BETA) + A) + B).arg()
                        } else {
                            (s_powf_alpha_max * ((s.powf(BETA)) + A) + B).arg()
                        }
                    };
                    let phi3 = {
                        if *w <= 1.0 {
                            (j1.powf(gamma_max) + 1.0).arg()
                        } else {
                            (s.powf(gamma_max) + 1.0).arg()
                        }
                    };

                    phi1 + phi2 + phi3
                };

                (-X * min_rho.sqrt() * (max_phi/2.0).cos()).exp()
            };

            let max_t2 = {
                let p0 = inverse_helper_norm;
                let p1 = if *w<=1.0 { B * helper_arg_cos } else {
                    (s_powf_alpha_min * helper + B).norm()
                };

                let p2 =  if *w <= 4.0 {
                    1.0
                } else {
                    (s_powf_gamma_min+1.0).norm()
                }; // 4 is arbitrary

                let min_psi = p0 * p1 * p2;

                min_psi.powf(-0.5)
            };

            let max_t3_alpha = {
                s_ln.norm() * w.powf(alpha_worst_max) * (w.powf(gamma_worst_max) + 1.0)
            };

            let max_t3_gamma = {
                let p1 = (s_ln / helper).norm();
                let p2 = w.powf(gamma_worst_max);
                let p3 = w.powf(alpha_worst_max) * helper.norm() + B;
                p1 * p2 * p3
            };

            let gradient_alpha = t0 * max_t1 * max_t2 * max_t3_alpha;
            let gradient_gamma = t0 * max_t1 * max_t2 * max_t3_gamma;

            let denominator = (gradient_alpha.powi(2) + gradient_gamma.powi(2)).sqrt();
            let result = num / denominator;

            result
    });

    Box::new(fraction_iter)
}


pub fn region_fraction<'a>(
    w_linspace: &'a [f64],
    origin: Par,
    eps: f64) -> Box<dyn Iterator<Item=f64> + 'a>
{
    let t0 = KX_HALF;
    let (alpha, gamma) = (origin.0, origin.1);
    let alpha_min = alpha - eps;
    let gamma_min = gamma - eps;
    let alpha_max = alpha + eps;
    let gamma_max = gamma + eps;
    let j1 = Comp::new(0.0, 1.0);

    let fraction_iter = w_linspace
        .iter()
        .map(move |w| {
            let num = f_complex(Comp::new(0.0, *w), origin).norm();
            let s = Comp::new(0.0, *w);
            let s_ln = s.ln();
            let helper = Comp::powf(s, BETA) + A;
            let helper_arg = helper.arg();
            let helper_arg_cos = helper_arg.cos();
            let inverse_helper_norm = helper.powf(-1.0).norm();
            let alpha_worst_max = if *w <= 1.0 { alpha_min } else { alpha_max };
            let gamma_worst_max = if *w <= 1.0 { gamma_min } else { gamma_max };
            let s_powf_alpha_min = s.powf(alpha_min);
            let s_powf_gamma_min = s.powf(gamma_min);
            let s_powf_alpha_max = s.powf(alpha_max);

            let max_t1 = {
                let min_rho = {
                    let rho1 = inverse_helper_norm;
                    let rho2 = {
                        if (alpha_min - alpha_max).abs() > TOLERANCE {
                            let rho_2_small = helper_arg_cos * B;

                            let point1 = s_powf_alpha_min * helper + B;
                            let point2 = s_powf_alpha_max * helper + B;
                            let side_point1 = point1.norm();
                            let side_point2 = point2.norm();
                            let side_mutual = (point1 - point2).norm();

                            let sh = (side_point1 + side_point2 + side_mutual) / 2.0; // Heron's formula
                            let area = (sh * (sh - side_point1) * (sh - side_point2) * (sh - side_mutual)).sqrt();
                            let rho_2_big = 2.0 * area / side_mutual;
                            if *w <= 1.0 { rho_2_small } else { rho_2_big }
                        }
                        else {
                            (s_powf_alpha_min * helper + B).norm()
                        }

                    };
                    let rho3 = {
                        let result_small = 1.0 + 0.0*w; // Not sure why
                        let result_big = (s_powf_gamma_min+1.0).norm();
                        if *w <= 4.0 { result_small } else { result_big } // 4 is arbitrary
                    };

                    rho1 * rho2 * rho3
                };
                let max_phi = {
                    let phi1 = -helper_arg;
                    let phi2 = {
                        let result_big = (s_powf_alpha_max * ((s.powf(BETA)) + A) + B).arg();
                        let result_w1 = (j1.powf(alpha_max) * (j1.powf(BETA) + A) + B).arg();
                        let result_small = result_w1;
                        if *w <= 1.0 { result_small } else { result_big }
                    };
                    let phi3 = {
                        let result_big = (s.powf(gamma_max) + 1.0).arg();
                        let result_w1 = (j1.powf(gamma_max) + 1.0).arg();
                        let result_small = 0.0*w + result_w1; // Not sure why

                        if *w <= 1.0 { result_small } else { result_big }
                    };

                    phi1 + phi2 + phi3
                };

                (-X * min_rho.sqrt() * (max_phi/2.0).cos()).exp()
            };

            let max_t2 = {
                let p0 = inverse_helper_norm;

                let result_small_w = B * helper_arg_cos;
                let result_big_w = (s_powf_alpha_min * helper + B).norm();
                let p1 = if *w<=1.0 { result_small_w } else { result_big_w };

                let p2_small = 1.0 + 0.0*w;
                let p2_big = (s_powf_gamma_min+1.0).norm();
                let p2 =  if *w <= 4.0 { p2_small } else { p2_big }; // 4 is arbitrary

                let min_psi = p0 * p1 * p2;

                min_psi.powf(-0.5)
            };

            let max_t3_alpha = {
                s_ln.norm() * w.powf(alpha_worst_max) * (w.powf(gamma_worst_max) + 1.0)
            };

            let max_t3_gamma = {
                let p1 = (s_ln / helper).norm();
                let p2 = w.powf(gamma_worst_max);
                let p3 = w.powf(alpha_worst_max) * helper.norm() + B;
                p1 * p2 * p3
            };

            let gradient_alpha = t0 * max_t1 * max_t2 * max_t3_alpha;
            let gradient_gamma = t0 * max_t1 * max_t2 * max_t3_gamma;
            let denominator = (gradient_alpha.powi(2) + gradient_gamma.powi(2)).sqrt();
            let result = num / denominator;

            result

    });

    Box::new(fraction_iter)
}


pub const SYSTEM: System = System {
    name: "telegrapher_alpha_gamma",
    f_complex,
    parameters: (r"\alpha", r"\gamma"),
    line_denominator: Option::Some(line_denominator),
    region_denominator: Option::None,
    region_fraction_precalculated_numerator: Option::Some(region_fraction_precalculated_numerator),
    region_fraction: Option::Some(region_fraction),
};