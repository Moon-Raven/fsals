use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let beta = p.0;
    let sigma = p.1;

    s.powf(beta) + Comp::exp(-Comp::sqrt(sigma*s))
}


pub const SYSTEM: System = System {
    name: "pde_complex_beta_sigma",
    f_complex,
    parameters: (r"\beta", r"\sigma"),
    line_denominator: Option::None,
    region_denominator: Option::None,
};