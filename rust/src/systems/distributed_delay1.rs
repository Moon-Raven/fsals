use crate::types::{Comp, Par, System};


fn f_complex(s: Comp, p: Par) -> Comp {
    let tau = p.0;
    let k = p.1;

    s*s + s*k + 1.0 - (-tau*(s+k)).exp()
}


pub const SYSTEM: System = System {
    name: "distributed_delay1",
    f_complex,
    parameters: (r"\tau", r"k"),
    line_denominator: Option::None,
    region_denominator: Option::None,
};
