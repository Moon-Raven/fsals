use lazy_static::lazy_static;
use log::{debug, info};

use iter_num_tools::{log_space, lin_space};
use rayon::iter::Update;
use crate::types::Par;
use crate::systems::distributed_delay1;

use crate::types;
use std::sync::Mutex;


pub fn get_maximum_condition<F>(condition: F, min_step: f64, limit: f64) -> f64
where F: Fn(f64) -> bool
{
    const CONSECUTIVE_SUCCESSES_THRESHOLD: u32 = 3;
    let mut step = min_step;
    let mut consecutive_successes: u32 = 0;
    let mut x: f64 = 0.0; // Variable we are optimizing
    let mut last_attempt_successful = true;
    debug!("Searching for min cond with: limit={}, min_step={}", limit, min_step);

    loop {
        let mut x_try = x + step;
        debug!("Trying {} = {} + {}", x_try, x, step);

        if x == x_try {
            panic!("Floating point precision exceeded!");
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

            // Avoid a silly situation where we retry same value as last time
            if !last_attempt_successful {
                step /= 2.0;
            }

            // If things have been going well for a while, increase steps
            last_attempt_successful = true;
            consecutive_successes += 1;
            if consecutive_successes >= CONSECUTIVE_SUCCESSES_THRESHOLD {
                step *= 2.0;
            }
        } else {
            // Things didn't go well; decrease steps
            step /= 2.0;
            consecutive_successes = 0;
            last_attempt_successful = false;
        }

        if step < min_step {
            debug!{"Minimum step breached; aborting"}
            break;
        }
    }
    debug!("Found maximum condition fulfilled by {}", x);

    x
}

pub struct MinimizationProblemSlow<'b, F1, F2>
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
{
    pub log_space: &'b[f64],
    pub lin_steps: usize,
    pub precalculated_numerator: &'b[f64],
    pub denominator_function: &'b F1,
    pub fraction_function: &'b F2,
}


pub struct MinimizationProblemFast<'b>
{
    pub log_space: &'b[f64],
    pub lin_steps: usize,
    pub logspace_fraction_iterator: Box<dyn Iterator<Item=f64> + 'b>,
    pub linspace_fraction_generator: Box<dyn Fn(&[f64]) -> Box<dyn Iterator<Item=f64> + '_> + 'b>,
}


pub fn get_linsearch_interval(
    index_of_logmin: usize,
    log_space: &[f64],
) -> Option<(f64, f64)>
{
    let last_index = log_space.len() - 1;
    let w_min =
        if index_of_logmin == 0 {
            log_space[0]
        } else {
            log_space[index_of_logmin -1]
        };

    let w_max =
        if index_of_logmin == last_index {
            debug!("Minimum seems to be out of bounds");
            return Option::None;
        } else {
            log_space[index_of_logmin + 1]
        };
    Some((w_min, w_max))
}


lazy_static!(
    static ref min_w: Mutex<f64> = Mutex::new(f64::INFINITY);
    static ref max_w: Mutex<f64> = Mutex::new(0.0);
);


pub fn update_statistics(w: f64) {
    /* Update statistics used for optimization purposes */
    {
        let mut min_w_unlocked = min_w.lock().unwrap();
        if w < *min_w_unlocked {
            *min_w_unlocked = w;
        }
    }

    {
        let mut max_w_unlocked = max_w.lock().unwrap();
        if w > *max_w_unlocked {
            *max_w_unlocked = w;
        }
    }
}


pub fn find_minimum_fraction_slow<F1, F2>(problem: &MinimizationProblemSlow<F1, F2>) -> f64
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
{
    let log_denominator_iterator = problem.log_space
        .iter()
        .map(|w| (problem.denominator_function)(*w));

    let log_fraction_iterator = problem.precalculated_numerator
        .iter()
        .zip(log_denominator_iterator)
        .map(|(num, denom)| num / denom);

    /* Perform search on logspace */
    let minind = log_fraction_iterator
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Invalid value found"))
        .map(|(index, _)| index)
        .expect("Error while searching for log min");

    // /* Perform search on linspace */
    let (w_min, w_max) = match get_linsearch_interval(minind, problem.log_space) {
        Some(val) => val,
        None => return 0.0,
    };

    debug!("Starting linsearch on [{}, {}]", w_min, w_max);

    let min = iter_num_tools::lin_space(w_min..=w_max, problem.lin_steps)
        .map(|w| (problem.fraction_function)(w))
        .min_by(|a, b| a.partial_cmp(b).expect("Invalid value found"))
        .expect("Error while searching for lin min");

        debug!("Found lin minimum f(?) = {}", min);

    min
}


pub fn print_minmax_statistics() {
    let min = min_w.lock().unwrap();
    let max = max_w.lock().unwrap();

    info!("Roots for found for w in ({}, {})", min, max);
}


pub fn find_minimum_fraction_fast<'b>(problem: MinimizationProblemFast<'b>) -> f64 {
    /* Perform search on logspace */
    let minind = problem.logspace_fraction_iterator
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Invalid value found"))
        .map(|(index, _)| index)
        .expect("Error while searching for log min");

    /* Perform search on linspace */
    let (w_min, w_max) = match get_linsearch_interval(minind, problem.log_space) {
        Some(val) => val,
        None => return 0.0,
    };
    debug!("Starting linsearch on [{}, {}]", w_min, w_max);

    let w_linspace: Vec<f64> =
        iter_num_tools::lin_space(w_min..=w_max, problem.lin_steps).collect();
    let linspace_iter = (problem.linspace_fraction_generator)(&w_linspace);

    let min = linspace_iter
        .min_by(|a, b| a.partial_cmp(b).expect("Invalid value found"))
        .expect("Error while searching for lin min");

    debug!("Found lin minimum f(?) = {}", min);

    min
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;

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


    // #[test]
    // fn test_find_minimum() {
    //     /* Numerator is (x-x_offset)^3, denominator is (x-x_offset) */
    //     let w_min = 1e-3;
    //     let w_max = 1e5;
    //     let x_offset = 2.0;
    //     let steps = 10;
    //     let log_steps = steps;
    //     let lin_steps = steps;
    //     let log_space: Vec<f64> = iter_num_tools::log_space(w_min..=w_max, log_steps).collect();
    //     let precalculated_numerator: Vec<f64> = log_space.iter().map(|w| (w-x_offset).powi(3).abs()).collect();
    //     let denominator_function = |w: f64| (w-x_offset).abs();
    //     let fraction_function = |w: f64| (w-x_offset).powi(2);

    //     let problem = MinimizationProblem {
    //         log_space: &log_space,
    //         lin_steps: lin_steps,
    //         precalculated_numerator : &precalculated_numerator,
    //         denominator_function: &denominator_function,
    //         fraction_function: &fraction_function,

    //     };

    //     let obtained = find_minimum_fraction(&problem);
    //     let expected = 0.0;
    //     let eps = 1e-1;
    //     println!("obtained vs expected: {} vs {}", obtained, expected);
    //     assert_floats_eq(obtained, expected, eps);
    // }
}