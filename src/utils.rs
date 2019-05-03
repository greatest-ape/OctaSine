


pub fn map_host_param_value_to_step<T: Copy>(steps: &[T], value: f64) -> T {
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


pub fn map_step_to_host_param_value<T: Copy + PartialEq>(
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


pub fn map_host_param_value_to_value_with_steps(
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


pub fn map_value_to_host_param_value_with_steps(
    steps: &[f64],
    internal_value: f64,
) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut sum = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");

    for step in steps {
        if internal_value <= *step {
            let ratio = (internal_value - prev_step) / (step - prev_step);

            return sum + ratio * increment;
        }

        prev_step = step;

        sum += increment;
    }

    1.0
}