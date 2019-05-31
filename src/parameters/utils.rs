

pub fn map_parameter_value_to_step<T: Copy>(steps: &[T], value: f64) -> T {
    let increment = 1.0 / steps.len() as f64;
    let mut s = 0.0;

    for step in steps.iter() {
        s += increment;

        if value <= s {
            return *step;
        }
    }

    *steps.last().expect("steps are empty")
}


pub fn map_step_to_parameter_value<T: Copy + PartialEq>(
    steps: &[T],
    step_value: T,
) -> f64 {
    for (i, step) in steps.iter().enumerate(){
        if *step == step_value {
            let increment = 1.0 / (steps.len() - 1).max(1) as f64;

            return i as f64 * increment;
        }
    }

    0.5 // Default if step_value is not in steps
}


pub fn map_parameter_value_to_value_with_steps(
    steps: &[f64],
    parameter_value: f64
) -> f64 {
    let increment = 1.0 / (steps.len() - 1).max(1) as f64;

    let mut sum = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");
    let mut prev_sum = sum;

    for step in steps[1..].iter() {
        sum += increment;

        if parameter_value <= sum {
            // Interpolate
            let interpolation_ratio = (parameter_value - prev_sum) / increment;

            return prev_step + (interpolation_ratio * (step - prev_step));
        }

        prev_step = step;
        prev_sum = sum;
    }

    *steps.last().expect("steps are empty")
}


pub fn map_value_to_parameter_value_with_steps(
    steps: &[f64],
    internal_value: f64,
) -> f64 {
    let increment = 1.0 / (steps.len() - 1).max(1) as f64;

    let mut sum = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");

    for step in steps[1..].iter() {
        if internal_value <= *step {
            let ratio = (internal_value - prev_step) / (step - prev_step);

            return sum + ratio * increment;
        }

        prev_step = step;

        sum += increment;
    }

    1.0
}


pub fn round_to_step(steps: &[f64], value: f64) -> f64 {
    let mut prev_step = *steps.first().expect("steps are empty");

    for step in &steps[1..] {
        if value <= *step {
            let prev_diff = (value - prev_step).abs();
            let current_diff = (value - step).abs();

            if prev_diff < current_diff {
                return prev_step;
            }
            else {
                return *step;
            }
        }

        prev_step = *step;
    }

    *steps.last().expect("steps are empty")
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};
    use assert_approx_eq::assert_approx_eq;

    use crate::constants::*;

    use super::*;

    #[test]
    fn test_round_to_step(){
        let steps = [1.0, 2.0, 4.0];

        assert_eq!(round_to_step(&steps, -10.0), 1.0);
        assert_eq!(round_to_step(&steps, 0.0), 1.0);
        assert_eq!(round_to_step(&steps, 1.0), 1.0);
        assert_eq!(round_to_step(&steps, 1.1), 1.0);
        assert_eq!(round_to_step(&steps, 1.49), 1.0);
        assert_eq!(round_to_step(&steps, 1.5), 2.0);
        assert_eq!(round_to_step(&steps, 4.0), 4.0);
        assert_eq!(round_to_step(&steps, 100.0), 4.0);
    }

    fn get_all_steps() -> Vec<f64> {
        let mut steps = Vec::new();

        steps.append(&mut OPERATOR_RATIO_STEPS.to_vec());
        steps.append(&mut OPERATOR_FREE_STEPS.to_vec());
        steps.append(&mut OPERATOR_FINE_STEPS.to_vec());
        steps.append(&mut OPERATOR_BETA_STEPS.to_vec());
        steps.append(&mut MASTER_FREQUENCY_STEPS.to_vec());

        steps.sort_by(|a, b| a.partial_cmp(b).unwrap());
        steps.dedup();

        steps
    }

    fn valid_parameter_value(value: f64) -> bool {
        !(value.is_nan() || value > 1.0 || value < 0.0)
    }

    #[test]
    fn test_map_step_to_parameter_value_valid_number(){
        fn prop(index: usize) -> TestResult {
            let steps = get_all_steps();

            if index >= steps.len() {
                return TestResult::discard();
            }

            let value = map_step_to_parameter_value(&steps[..], steps[index]);

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(usize) -> TestResult);
    }

    #[test]
    fn test_map_value_to_parameter_value_with_steps_valid_number(){
        fn prop(value: f64) -> TestResult {
            if value < 0.0 {
                return TestResult::discard();
            }

            let value = map_value_to_parameter_value_with_steps(
                &get_all_steps()[..],
                value
            );

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }

    #[test]
    fn test_map_parameter_value_to_step() {
        let steps = [1, 2, 3];

        assert_eq!(map_parameter_value_to_step(&steps[..], 0.0), 1);
        assert_eq!(map_parameter_value_to_step(&steps[..], 0.5), 2);
        assert_eq!(map_parameter_value_to_step(&steps[..], 0.66), 2);
        assert_eq!(map_parameter_value_to_step(&steps[..], 0.67), 3);
        assert_eq!(map_parameter_value_to_step(&steps[..], 1.0), 3);
    }

    #[test]
    fn test_map_step_to_parameter_value() {
        let steps = [1, 2, 3, 4, 5];

        assert_approx_eq!(map_step_to_parameter_value(&steps[..], 1), 0.0);
        assert_approx_eq!(map_step_to_parameter_value(&steps[..], 2), 0.25);
        assert_approx_eq!(map_step_to_parameter_value(&steps[..], 3), 0.5);
        assert_approx_eq!(map_step_to_parameter_value(&steps[..], 4), 0.75);
        assert_approx_eq!(map_step_to_parameter_value(&steps[..], 5), 1.0);
    }

    #[test]
    fn test_map_parameter_value_to_value_with_steps(){
        let steps = [1.0, 2.0, 3.0];

        assert_approx_eq!(
            map_parameter_value_to_value_with_steps(&steps[..], 0.0),
            1.0
        );
        assert_approx_eq!(
            map_parameter_value_to_value_with_steps(&steps[..], 0.5),
            2.0
        );
        assert_approx_eq!(
            map_parameter_value_to_value_with_steps(&steps[..], 0.75),
            2.5
        );
        assert_approx_eq!(
            map_parameter_value_to_value_with_steps(&steps[..], 1.0),
            3.0
        );
    }

    #[test]
    fn test_map_value_to_parameter_value_with_steps(){
        let steps = [1.0, 2.0, 3.0];

        assert_approx_eq!(
            map_value_to_parameter_value_with_steps(&steps[..], 1.0),
            0.0
        );
        assert_approx_eq!(
            map_value_to_parameter_value_with_steps(&steps[..], 2.0),
            0.5
        );
        assert_approx_eq!(
            map_value_to_parameter_value_with_steps(&steps[..], 2.5),
            0.75
        );
        assert_approx_eq!(
            map_value_to_parameter_value_with_steps(&steps[..], 3.0),
            1.0
        );
    }

    #[test]
    fn test_smooth_step_mapping(){
        fn prop(parameter_value: f64) -> TestResult {
            if parameter_value < 0.0 || parameter_value > 1.0 {
                return TestResult::discard();
            }

            let steps = get_all_steps();

            let internal_value = map_parameter_value_to_value_with_steps(
                &steps[..],
                parameter_value
            );
            let new_parameter_value = map_value_to_parameter_value_with_steps(
                &steps[..],
                internal_value
            );

            let diff = (parameter_value - new_parameter_value).abs();
            let success = diff < 0.0001;

            if !success {
                println!("steps: {:?}", steps);
                println!("parameter value: {}", parameter_value);
                println!("internal value: {}", internal_value);
                println!("new parameter value: {}", new_parameter_value);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }

    #[test]
    fn test_step_mapping(){
        fn prop(value: f64) -> TestResult {
            if value < 0.0 || value > 1.0 {
                return TestResult::discard();
            }

            let steps = get_all_steps();

            let inner = map_parameter_value_to_step(
                &steps[..],
                value
            );
            let new_value = map_step_to_parameter_value(
                &steps[..],
                inner
            );
            // Simulate the VST sending back the recieved parameter value.
            // Result must be identical to last result.
            let new_inner = map_parameter_value_to_step(
                &steps[..],
                new_value
            );

            TestResult::from_bool(inner == new_inner)
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}