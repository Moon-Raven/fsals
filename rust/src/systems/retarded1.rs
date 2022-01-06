use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau1 = p.0;
    let tau2 = p.1;

    s*s + 2.0*s*(-s*tau1).exp() + (-s*tau2).exp()
}


pub const SYSTEM: System = System {
    name: "retarded1",
    f_complex,
    parameters: (r"\tau_1", r"\tau_2")
};