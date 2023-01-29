pub fn map_patch_value_to_step<T: Copy>(steps: &[T], value: f32) -> T {
    let value = value.max(0.0).min(1.0);
    let len = steps.len();

    steps[((value * len as f32) as usize).min(len - 1)]
}

pub fn map_step_to_patch_value<T: Copy + PartialEq>(steps: &[T], step_value: T) -> f32 {
    for (index, step) in steps.iter().enumerate() {
        if *step == step_value {
            let fraction = 1.0 / (steps.len() - 1) as f32;

            return fraction * index as f32;
        }
    }

    0.5 // Default if step_value is not in steps
}

pub fn map_patch_to_audio_value_with_steps(steps: &[f32], patch_value: f32) -> f32 {
    let max_index = steps.len() - 1;

    let index_float = patch_value.max(0.0).min(1.0) * max_index as f32;
    let index_fract = index_float.fract();

    let index_low = index_float as usize;

    if index_low == max_index {
        steps[index_low]
    } else {
        let step_low = steps[index_low];
        let step_high = steps[index_low + 1];

        step_low + index_fract * (step_high - step_low)
    }
}

pub fn map_audio_to_patch_value_with_steps(steps: &[f32], audio_value: f32) -> f32 {
    let mut prev_step = *steps.first().expect("steps are empty");

    for (index, step) in steps[1..].iter().enumerate() {
        let step = *step;

        if audio_value <= step {
            let ratio = (audio_value - prev_step) / (step - prev_step);
            let fraction = ((steps.len() - 1) as f32).recip();

            return ratio * fraction + fraction * index as f32;
        }

        prev_step = step;
    }

    1.0
}

pub fn round_to_step(steps: &[f32], value: f32) -> f32 {
    let mut prev_step = *steps.first().expect("steps are empty");

    for step in &steps[1..] {
        if value <= *step {
            let prev_diff = (value - prev_step).abs();
            let current_diff = (value - step).abs();

            if prev_diff < current_diff {
                return prev_step;
            } else {
                return *step;
            }
        }

        prev_step = *step;
    }

    *steps.last().expect("steps are empty")
}

pub fn parse_valid_f32(text: &str, min: f32, max: f32) -> Option<f32> {
    let value: f32 = text.parse().ok()?;

    if value.is_infinite() | value.is_nan() {
        None
    } else {
        Some(value.min(max).max(min))
    }
}

pub fn parse_valid_f64(text: &str, min: f64, max: f64) -> Option<f64> {
    let value: f64 = text.parse().ok()?;

    if value.is_infinite() | value.is_nan() {
        None
    } else {
        Some(value.min(max).max(min))
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use quickcheck::{quickcheck, TestResult};

    use super::*;

    fn get_all_steps() -> Vec<f32> {
        let mut steps = vec![0.0];

        for i in 0..300 {
            steps.push(0.001 * i as f32 + (i >> 3) as f32 * 0.1);
        }

        steps.dedup();

        steps
    }

    fn valid_parameter_value(value: f32) -> bool {
        !(value.is_nan() || value > 1.0 || value < 0.0)
    }

    #[test]
    fn test_map_step_to_patch_value_valid_number() {
        fn prop(index: usize) -> TestResult {
            let steps = get_all_steps();

            if index >= steps.len() {
                return TestResult::discard();
            }

            let value = map_step_to_patch_value(&steps[..], steps[index]);

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(usize) -> TestResult);
    }

    #[test]
    fn test_map_patch_value_to_step() {
        let steps = [1, 2, 3];

        assert_eq!(map_patch_value_to_step(&steps[..], 0.0), 1);
        assert_eq!(map_patch_value_to_step(&steps[..], 0.5), 2);
        assert_eq!(map_patch_value_to_step(&steps[..], 0.66), 2);
        assert_eq!(map_patch_value_to_step(&steps[..], 0.67), 3);
        assert_eq!(map_patch_value_to_step(&steps[..], 1.0), 3);
    }

    #[test]
    fn test_map_step_to_patch_value() {
        let steps = [1, 2, 3, 4, 5];

        assert_approx_eq!(map_step_to_patch_value(&steps[..], 1), 0.0);
        assert_approx_eq!(map_step_to_patch_value(&steps[..], 2), 0.25);
        assert_approx_eq!(map_step_to_patch_value(&steps[..], 3), 0.5);
        assert_approx_eq!(map_step_to_patch_value(&steps[..], 4), 0.75);
        assert_approx_eq!(map_step_to_patch_value(&steps[..], 5), 1.0);
    }

    #[test]
    fn test_step_mapping() {
        fn prop(value: f32) -> TestResult {
            if value < 0.0 || value > 1.0 {
                return TestResult::discard();
            }

            let steps = get_all_steps();

            let inner = map_patch_value_to_step(&steps[..], value);
            let new_value = map_step_to_patch_value(&steps[..], inner);
            // Simulate the VST sending back the recieved parameter value.
            // Result must be identical to last result.
            let new_inner = map_patch_value_to_step(&steps[..], new_value);

            #[allow(clippy::float_cmp)]
            TestResult::from_bool(inner == new_inner)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[test]
    fn test_map_audio_to_patch_value_with_steps_valid_number() {
        fn prop(value: f32) -> TestResult {
            if value < 0.0 {
                return TestResult::discard();
            }

            let value = map_audio_to_patch_value_with_steps(&get_all_steps()[..], value);

            TestResult::from_bool(valid_parameter_value(value))
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[test]
    fn test_map_patch_to_audio_value_with_steps() {
        let steps = [1.0, 2.0, 3.0];

        assert_approx_eq!(map_patch_to_audio_value_with_steps(&steps[..], 0.0), 1.0);
        assert_approx_eq!(map_patch_to_audio_value_with_steps(&steps[..], 0.5), 2.0);
        assert_approx_eq!(map_patch_to_audio_value_with_steps(&steps[..], 0.75), 2.5);
        assert_approx_eq!(map_patch_to_audio_value_with_steps(&steps[..], 1.0), 3.0);
    }

    #[test]
    fn test_map_audio_to_patch_value_with_steps() {
        let steps = [1.0, 2.0, 3.0];

        assert_approx_eq!(map_audio_to_patch_value_with_steps(&steps[..], 1.0), 0.0);
        assert_approx_eq!(map_audio_to_patch_value_with_steps(&steps[..], 2.0), 0.5);
        assert_approx_eq!(map_audio_to_patch_value_with_steps(&steps[..], 2.5), 0.75);
        assert_approx_eq!(map_audio_to_patch_value_with_steps(&steps[..], 3.0), 1.0);
    }

    #[test]
    fn test_smooth_step_mapping() {
        fn prop(parameter_value: f32) -> TestResult {
            if parameter_value < 0.0 || parameter_value > 1.0 || parameter_value.is_nan() {
                return TestResult::discard();
            }

            let steps = get_all_steps();

            let internal_value = map_patch_to_audio_value_with_steps(&steps[..], parameter_value);
            let new_parameter_value =
                map_audio_to_patch_value_with_steps(&steps[..], internal_value);

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

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn test_round_to_step() {
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

    #[test]
    fn test_parse_valid_f32() {
        fn prop(v: f32, min: f32, max: f32) -> TestResult {
            if min.is_infinite() | min.is_nan() | max.is_infinite() | max.is_nan() {
                return TestResult::discard();
            }
            if min > max {
                return TestResult::discard();
            }

            let result = parse_valid_f32(&v.to_string(), min, max);

            if v.is_infinite() | v.is_nan() {
                return TestResult::from_bool(result.is_none());
            }

            let parsed_v = result.unwrap();

            if v < min {
                TestResult::from_bool(parsed_v == min)
            } else if v > max {
                TestResult::from_bool(parsed_v == max)
            } else if parsed_v < min || parsed_v > max {
                TestResult::failed()
            } else {
                TestResult::from_bool(v == parsed_v)
            }
        }

        quickcheck(prop as fn(f32, f32, f32) -> TestResult);
    }
}
