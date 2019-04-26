


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


pub fn map_host_param_value_to_step_smooth(steps: &[f64], value: f64) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut s = 0.0;
    let mut prev_step = steps.first().expect("steps are empty");
    let mut prev_s = s;

    for step in steps.iter() {
        s += increment;

        if value < s {
            // Interpolate
            let interpolation_ratio = (value - prev_s) / increment;
            return prev_step + (interpolation_ratio * (step - prev_step));
        }

        prev_step = step;
        prev_s = s;
    }

    *steps.last().expect("steps are empty")
}


pub fn get_host_value_for_step<T: Copy + PartialEq>(
    steps: &[T],
    value: T,
) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut sum = 0.0;

    for step in steps.iter() {
        if *step == value {
            return sum;
        }

        sum += increment;
    }

    0.5
}