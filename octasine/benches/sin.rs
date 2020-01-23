#[cfg(not(feature = "simd"))]
fn main(){
    println!("Activate SIMD feature to run");
}

/// Compare sleef, table and libm sine functions
#[cfg(feature = "simd")]
fn main(){
    type V = f64x4;

    use std::time::Instant;

    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;

    use simd_sleef_sin35::SleefSin35;
    use packed_simd::*;

    let mut rng = SmallRng::from_entropy();

    let mut inputs_std: Vec<f64> = Vec::new();
    let mut inputs_sleef: Vec<V> = Vec::new();

    let iterations = 50_000_000;

    for i in 0..iterations {
        let v = rng.gen::<f64>() * (((i % 256) + 1) as f64);

        inputs_std.push(v);

        let v = V::new(v, v * 0.4, v, v * 0.1);

        inputs_sleef.push(v);
    }

    let start_sleef = Instant::now();

    let outputs_sleef: Vec<V> = inputs_sleef.into_iter()
        .map(|v| SleefSin35::sin(v))
        .collect();

    let elapsed_sleef = start_sleef.elapsed();

    let start_simple = Instant::now();

    let outputs_std: Vec<f64> = inputs_std.into_iter()
        .map(|v| v.sin())
        .collect();

    let elapsed_simple = start_simple.elapsed();

    println!();
    println!("--- Sleef sin35 vs libm sin ---");
    println!("Number of iterations:   {}", iterations);
    println!("Test ran for:           {}ms",
        elapsed_sleef.as_millis() + elapsed_simple.as_millis());
    println!("Sleef average duration: {} nanoseconds (for {} values)",
        elapsed_sleef.as_nanos() as f64 / iterations as f64, V::lanes());
    println!("libm average duration:  {} nanoseconds (for 1 value)",
        elapsed_simple.as_nanos() as f64 / iterations as f64);

    // Not very amazing way of trying to prevent compiler from optimizing
    // away stuff

    let mut dummy_counter = 0usize;

    let iterator = outputs_sleef.into_iter().zip(outputs_std.into_iter());

    for (a, b) in iterator {
        if a.sum() == b {
            dummy_counter += 1;
        }
    }

    if dummy_counter > (iterations >> 2) {
        println!("Dummy information: {}", dummy_counter);
    }
}