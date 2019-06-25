#[cfg(not(feature = "simd"))]
fn main(){
    println!("Activate SIMD feature to run");
}

/// Compare sleef, table and libm sine functions
#[cfg(feature = "simd")]
fn main(){
    type V = f32x2;

    use std::time::Instant;

    use rand::{Rng, FromEntropy};
    use rand::rngs::SmallRng;

    use simd_sleef_sin35::SleefSin35;
    use packed_simd::*;
    use octasine::*;

    let table = SineLookupTable::new();

    let mut rng = SmallRng::from_entropy();

    let mut inputs = Vec::new();
    let mut inputs_tau = Vec::new();
    let mut inputs_simple = Vec::new();

    let iterations = 10_000_000;

    for i in 0..iterations {
        let v = rng.gen::<f32>() * (((i % 256) + 1) as f32);

        inputs_simple.push(v);

        let v = V::new(v, v * 0.4);

        inputs.push(v);
        inputs_tau.push(v * TAU);
    }

    let start_sleef = Instant::now();

    let outputs_sleef: Vec<V> = inputs_tau.into_iter()
        .map(|v| SleefSin35::sin(v))
        .collect();

    let elapsed_sleef = start_sleef.elapsed();

    let start_table = Instant::now();

    let outputs_table: Vec<V> = inputs.into_iter()
        .map(|v| table.sin_tau_simd_x2(v))
        .collect();

    let elapsed_table = start_table.elapsed();

    let start_simple = Instant::now();

    let outputs_simple: Vec<f32> = inputs_simple.into_iter()
        .map(|v| v.sin())
        .collect();

    let elapsed_simple = start_simple.elapsed();

    println!();
    println!("--- Sleef vs table vs libm sin() ---");
    println!("Number of iterations:   {}", iterations);
    println!("Test ran for:           {}ms",
        elapsed_sleef.as_millis() + elapsed_table.as_millis() +
        elapsed_simple.as_millis());
    println!("Sleef average duration: {} nanoseconds (for {} values)",
        elapsed_sleef.as_nanos() as f32 / iterations as f32, V::lanes());
    println!("Table average duration: {} nanoseconds (for {} values)",
        elapsed_table.as_nanos() as f32 / iterations as f32, V::lanes());
    println!("Libm average duration:  {} nanoseconds (for 1 value)",
        elapsed_simple.as_nanos() as f32 / iterations as f32);

    // Not very amazing way of trying to prevent compiler from optimizing
    // away stuff

    let mut dummy_counter = 0usize;

    let iterator = outputs_sleef.into_iter()
        .zip(outputs_table.into_iter()).zip(outputs_simple.into_iter());

    for ((a, b), c) in iterator {
        if a == b && a.extract(0) == c {
            dummy_counter += 1;
        }
    }

    if dummy_counter > iterations {
        println!("Dummy information: {}", dummy_counter);
    }
}