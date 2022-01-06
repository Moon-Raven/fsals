use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let sigma = p.1;

    s + (-(sigma * s).sqrt()).exp() * (-tau * s).exp()
}


pub const SYSTEM: System = System {
    name: "pde_complex_tau_sigma",
    f_complex,
    parameters: (r"\tau", r"\sigma")
};