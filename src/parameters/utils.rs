

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
    let increment = 1.0 / steps.len() as f64;
    let mut sum = 0.0;

    for step in steps.iter() {
        if *step == step_value {
            return sum + increment / 2.0;
        }

        sum += increment;
    }

    0.5 // Default if step_value is not in steps
}


pub fn map_parameter_value_to_value_with_steps(
    steps: &[f64],
    host_value: f64
) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut sum = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");
    let mut prev_sum = sum;

    for step in steps.iter() {
        sum += increment;

        if host_value <= sum {
            // Interpolate
            let interpolation_ratio = (host_value - prev_sum) / increment;
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
    let increment = 1.0 / steps.len() as f64;
    let mut sum = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");

    for step in steps {
        if internal_value <= *step {
            // TODO: fix algorithm instead of this check
            if step == prev_step {
                return 0.0;
            }

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

    fn valid_parameter_value(value: f64) -> bool {
        !(value.is_nan() || value > 1.0 || value < 0.0)
    }

    #[test]
    fn test_map_step_to_parameter_value(){
        fn prop(index: usize) -> TestResult {
            let steps = OPERATOR_RATIO_STEPS;

            if index >= steps.len() {
                return TestResult::discard();
            }

            let value = map_step_to_parameter_value(&steps[..], steps[index]);

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(usize) -> TestResult);
    }

    #[test]
    fn test_map_value_to_parameter_value_with_steps(){
        fn prop(value: f64) -> TestResult {
            let steps = OPERATOR_RATIO_STEPS;

            let value = map_value_to_parameter_value_with_steps(&steps[..], value);

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}