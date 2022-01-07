use log::{debug, info};


pub fn get_maximum_condition<F>(condition: F, min_step: f64, limit: f64) -> f64
where F: Fn(f64) -> bool
{
    const CONSECUTIVE_SUCCESSES_THRESHOLD: u32 = 5;
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
            break;
        }
    }
    debug!("Found maximum condition fulfilled by {}", x);

    x
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::get_maximum_condition;

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
        init_logging();

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
}