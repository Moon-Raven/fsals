use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let k = p.0;
    let sigma = p.1;

    s + k * (-(sigma*s).sqrt()).exp()
}


pub const SYSTEM: System = System {
    name: "pde_complex_k_sigma",
    f_complex,
    parameters: (r"k", r"\sigma"),
    line_denominator: Option::None,
    region_denominator: Option::None,
};