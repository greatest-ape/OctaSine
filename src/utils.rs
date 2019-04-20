


pub fn map_host_param_value_to_step(steps: Vec<f64>, value: f64) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut s = 0.0;

    for step in steps.iter() {
        if value <= s {
            return *step;
        }

        s += increment;
    }

    *steps.last().expect("steps are empty")
}


pub fn get_host_value_for_default_step(
    steps: Vec<f64>,
    default_step: f64,
) -> f64 {
    let increment = 1.0 / steps.len() as f64;
    let mut s = 0.0;

    for step in steps.iter() {
        if *step == default_step {
            return s;
        }

        s += increment;
    }

    0.5
}