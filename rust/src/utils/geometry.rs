use crate::types::{Par, Limits};


pub fn is_point_in_limits(point: Par, limits: &Limits) -> bool {
    if point.0 < limits.p1_min || point.0 > limits.p1_max {
        false
    } else if point.1 < limits.p2_min || point.1 > limits.p2_max {
        false
    } else {
        true
    }
}