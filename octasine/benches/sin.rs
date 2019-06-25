#[cfg(not(feature = "simd"))]
fn main(){
    println!("Activate SIMD feature to run");
}

/// Compare sleef and table sine functions
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

    let iterations = 10_000_000;

    for i in 0..iterations {
        let v = rng.gen::<f32>() * (((i % 256) + 1) as f32);
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

    println!();
    println!("--- Sleef vs table sin() ---");
    println!("Number of iterations:   {}", iterations);
    println!("Test ran for:           {}ms", elapsed_sleef.as_millis() + elapsed_table.as_millis());
    println!("Sleef average duration: {} nanoseconds", elapsed_sleef.as_nanos() as f32 / iterations as f32);
    println!("Table average duration: {} nanoseconds", elapsed_table.as_nanos() as f32 / iterations as f32);

    // Not very amazing way of trying to prevent compiler from optimizing
    // away stuff
    let mut dummy_counter = 0usize;

    for (a, b) in outputs_sleef.iter().zip(outputs_table.iter()){
        if a == b {
            dummy_counter += 1;
        }
    }

    println!("Dummy information: {}", dummy_counter);
}