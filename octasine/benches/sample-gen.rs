use rand::FromEntropy;
use rand::rngs::SmallRng;

use octasine::processing_parameters::*;
use octasine::common::*;
use octasine::voices::*;

use vst2_helpers::approximations::*;


#[cfg(not(feature = "simd"))]
fn main(){
    println!("Activate SIMD feature to run");
}

/// Sample generation benchmark, comparing SIMD and non-SIMD versions.
/// 
/// Speedup seems to average something like 20% over several runs. Example
/// output:
/// 
/// --- SIMD vs non-SIMD sample generation ---
/// Number of tests:           100
/// Non-SIMD total duration:   22763ms
/// SIMD total duration:       18166ms
/// Non-SIMD average duration: 227ms
/// SIMD average duration:     181ms
/// SIMD speedup               20.191347% (ratio 0.7980865)
/// 
/// Non-SIMD benchmark speed decreased a lot as I moved this to its own file.
#[cfg(feature = "simd")]
fn main(){
    use std::time::Instant;

    let n = 10;

    let now = Instant::now();
    let samples_1 = gen_voice_samples(n, gen::generate_voice_samples);
    let elapsed_1 = now.elapsed();

    let now = Instant::now();
    let samples_2 = gen_voice_samples(n,
        gen::generate_voice_samples_simd);
    let elapsed_2 = now.elapsed();

    let speed_ratio = elapsed_2.as_micros() as f64 /
        elapsed_1.as_micros() as f64;
    
    let num_samples = n * 44100 * 4 * 4;

    println!("--- SIMD vs non-SIMD sample generation ---");
    println!("Number of tests:           {}", n);
    println!("Non-SIMD total duration:   {}ms", elapsed_1.as_millis());
    println!("Non-SIMD average duration: {}ms", elapsed_1.as_millis() as usize / n);
    println!("Non-SIMD per sample:       {} nanoseconds", elapsed_1.as_nanos() as f64 / num_samples as f64);
    println!("SIMD total duration:       {}ms", elapsed_2.as_millis());
    println!("SIMD average duration:     {}ms", elapsed_2.as_millis() as usize / n);
    println!("SIMD per sample:           {} nanoseconds", elapsed_2.as_nanos() as f64 / num_samples as f64);
    println!("SIMD speedup               {}% (ratio {})",
        (1.0 - speed_ratio) * 100.0, speed_ratio);
    println!("SIMD estimated CPU use:    {}%", elapsed_2.as_nanos() as f64 / (n * 4 * 4 * 10_000_000) as f64);
    println!("Info: At the moment, non-SIMD benchmark doesn't seem to");
    println!("reflect real-world performance");

    // Not very amazing way of trying to prevent compiler from optimizing
    // away stuff
    let mut dummy_counter = 0usize;

    for ((left_1, right_1), (left_2, right_2)) in samples_1.iter().zip(samples_2.iter()){
        if left_1 == left_2 && right_1 == right_2 {
            dummy_counter += 1;
        }
    }

    if dummy_counter > 44100 {
        println!("Dummy information: {}", dummy_counter);
    }
}


/// Voice sample generation for benchmarking
#[allow(dead_code)]
fn gen_voice_samples(
    num_tests: usize,
    f: fn(&Log10Table, &mut SmallRng, TimeCounter, TimePerSample,
        &mut ProcessingParameters, &mut Voice) -> (f64, f64)
) -> Vec<(f64, f64)> {
    const SAMPLE_RATE: usize = 44100;

    const ITERATIONS: usize = SAMPLE_RATE * 4 * 4;
    const ITERATIONS_RECIP: f64 = 1.0 / ITERATIONS as f64;

    let envelope_duration_parameters =
        [10usize, 12, 14, 24, 26, 28, 39, 41, 43, 54, 56, 58];
    
    let wave_type_parameters = [4, 17, 31, 46];

    let log10_table = Log10Table::default();
    let midi_pitch = MidiPitch::new(60);
    let mut rng = SmallRng::from_entropy();

    let mut parameters = ProcessingParameters::default();

    for i in envelope_duration_parameters.iter() {
        parameters.get(*i).unwrap().set_from_preset_value(1.0);
    }

    let mut voice = Voice::new(midi_pitch);

    let mut time = TimeCounter(0.0);
    let time_per_sample = TimePerSample(1.0 / SAMPLE_RATE as f64);

    let mut results: Vec<(f64, f64)> = Vec::new();

    for test_i in 0..num_tests {
        for i in 0..ITERATIONS {
            if i == 0 {
                voice.press_key(100);
            } else if i == (ITERATIONS / 2) * 3 {
                voice.release_key();
            }

            let (l, r) = f(
                &log10_table,
                &mut rng,
                time,
                time_per_sample,
                &mut parameters,
                &mut voice,
            );

            results.push((l, r));

            time.0 += time_per_sample.0;
            voice.duration.0 += time_per_sample.0;

            if i % 32 == test_i % 32 {
                for parameter_index in 0..parameters.len(){
                    if envelope_duration_parameters.contains(&parameter_index){
                        continue;
                    }

                    if wave_type_parameters.contains(&parameter_index){
                        continue;
                    }

                    let parameter = parameters.get(parameter_index).unwrap();

                    let mut new_value = i as f64 * ITERATIONS_RECIP;

                    if parameter_index % 2 == 0 {
                        new_value = 1.0 - new_value;
                    }

                    if parameter_index % 4 == 1 {
                        new_value = new_value.sqrt();
                    }

                    if parameter_index % 4 == 3 {
                        new_value = new_value.sqrt().sqrt();
                    }

                    parameter.set_from_preset_value(new_value);
                }
            }
        }
    }

    results
}