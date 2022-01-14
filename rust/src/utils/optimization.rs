use lazy_static::lazy_static;
use log::{debug, info};

use iter_num_tools::{log_space, lin_space};


pub fn get_maximum_condition<F>(condition: F, min_step: f64, limit: f64) -> f64
where F: Fn(f64) -> bool
{
    const CONSECUTIVE_SUCCESSES_THRESHOLD: u32 = 3;
    let mut step = min_step * 1e3;
    let mut consecutive_successes: u32 = 0;
    let mut x: f64 = 0.0; // Variable we are optimizing
    debug!("Searching for min cond with: limit={}, min_step={}", limit, min_step);

    loop {
        let mut x_try = x + step;
        debug!("Trying {} = {} + {}", x_try, x, step);

        if x == x_try {
            panic!("Floating point precition exceeded!");
        }

        if x_try > limit {
            x_try = limit;
            debug!("Attempting maximum allowed x={} by limit", x_try);
        }

        if condition(x_try) {
            // Things went well; accept new value
            x = x_try;

            // If we cannnot increase anymore, terminate
            if x == limit {
                debug!("Limit {} reached; aborting", limit);
                break;
            }

            // If things have been going well for a while, increase steps
            consecutive_successes += 1;
            if consecutive_successes >= CONSECUTIVE_SUCCESSES_THRESHOLD {
                step *= 2.0;
            }
        } else {
            // Things didn't go well; decrease steps
            step /= 2.0;
            consecutive_successes = 0;
        }

        if step < min_step {
            debug!{"Minimum step breached; aborting"}
            break;
        }
    }
    debug!("Found maximum condition fulfilled by {}", x);

    x
}


lazy_static! {
    pub static ref W_LOGSPACED: Vec<f64> = {
        let w_min = 1e-3;
        let w_max = 1e10;
        let steps = 1_000;
        log_space(w_min..=w_max, steps).collect()
    };
}


pub fn find_minimum<F>(f: F) -> f64
where F: Fn(f64) -> f64
{
    let w_size = W_LOGSPACED.len();
    let last_ind: usize = w_size - 1;

    /* Perform search on logspace */
    let minind = W_LOGSPACED
        .iter()
        .map(|w| f(*w))
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Invalid value found"))
        .map(|(index, _)| index)
        .expect("Error while searching for log min");

    let argmin = W_LOGSPACED[minind];
    let min = f(argmin);
    debug!("Found log minimum f({}) = {} at index {}", argmin, min, minind);

    /* Perform search on linspace */
    let w_min =
        if minind == 0 {
            0.0
        } else {
            W_LOGSPACED[minind -1]
        };
    let w_max =
        if minind == last_ind {
            panic!("Minimum seems to be out of bounds")
        } else {
            W_LOGSPACED[minind + 1]
        };
    debug!("Starting linsearch on [{}, {}]", w_min, w_max);

    let min = lin_space(w_min..=w_max, w_size)
        .map(|w| f(w))
        .min_by(|a, b| a.partial_cmp(b).expect("Invalid value found"))
        .expect("Error while searching for lin min");

    debug!("Found lin minimum f(?) = {}", min);

    min
}


#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::{get_maximum_condition, find_minimum};

    fn assert_floats_eq(x: f64, y:f64, assertion_eps: f64) {
        let diff = f64::abs(x - y);
        assert!(diff <= assertion_eps);
    }

    fn init_logging() {
        simple_logger::SimpleLogger::new()
            .with_utc_timestamps()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap();
    }

    #[test]
    fn test_get_maximum_condition() {
        let target = 5.0;
        let condition = |x| x <= target;
        let min_step = 0.1;
        let assertion_eps = min_step * 2.0;
        let limit = 10.0;
        let result = get_maximum_condition(condition, min_step, limit);
        assert_floats_eq(result, target, assertion_eps);

        let target = 5.0;
        let condition = |x| x <= target;
        let min_step = 0.1;
        let assertion_eps = min_step * 2.0;
        let limit = 3.0;
        let result = get_maximum_condition(condition, min_step, limit);
        assert_floats_eq(result, limit, assertion_eps);
    }


    #[test]
    fn test_find_minimum_regular() {
        let target_x = 5.0;
        let target_y = 2.0;
        let f = |x: f64| (x-target_x).powi(2) + target_y;
        let min = find_minimum(f);
        let assertion_eps = 1e-3;
        println!("Expected {}, got {}", target_y, min);
        assert_floats_eq(min, target_y, assertion_eps);
    }


    #[test]
    fn test_find_minimum_left() {
        let f = |x: f64| x;
        let min = find_minimum(f);
        let assertion_eps = 1e-3;
        let expected = 0.0f64;
        println!("Expected {}, got {}", expected, min);
        assert_floats_eq(min, expected , assertion_eps);
    }


    #[test]
    #[should_panic]
    fn test_find_minimum_right() {
        let limit = 1e10;
        let f = |x: f64| limit - x;
        let min = find_minimum(f);
        /* Should be unreachable */
    }
}