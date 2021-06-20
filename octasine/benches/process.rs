use std::time::Instant;

use colored::*;
use sha2::{Digest, Sha256};
use vst::buffer::AudioBuffer;
use vst::plugin::PluginParameters;

use octasine::gen::AudioGen;
use octasine::OctaSine;

/// Benchmark OctaSine process functions
///
/// Example output:
/// ```txt
/// --- Benchmarking OctaSine process_f32 variant: fallback (std) ---
/// Total number of samples:        12800000
/// Equivalent to audio duration:   290.24942 seconds
/// Processing time in total:       27893 milliseconds
/// Processing time per sample:     2179.1711 nanoseconds
/// Estimated CPU use:              9.610011%
/// Output hash (first 8 bytes):    ad 0d 1d 04 5e 38 95 7f
///
/// --- Benchmarking OctaSine process_f32 variant: fallback (sleef) ---
/// Total number of samples:        12800000
/// Equivalent to audio duration:   290.24942 seconds
/// Processing time in total:       21805 milliseconds
/// Processing time per sample:     1703.5895 nanoseconds
/// Estimated CPU use:              7.5125046%
/// Output hash (first 8 bytes):    ac fd ce 1e a2 7b 79 e1
/// Speed compared to std fallback: 1.2791644x

/// --- Benchmarking OctaSine process_f32 variant: sse2 ---
/// Total number of samples:        12800000
/// Equivalent to audio duration:   290.24942 seconds
/// Processing time in total:       18445 milliseconds
/// Processing time per sample:     1441.0449 nanoseconds
/// Estimated CPU use:              6.3548794%
/// Output hash (first 8 bytes):    ac fd ce 1e a2 7b 79 e1
/// Speed compared to std fallback: 1.512216x

/// --- Benchmarking OctaSine process_f32 variant: avx ---
/// Total number of samples:        12800000
/// Equivalent to audio duration:   290.24942 seconds
/// Processing time in total:       12060 milliseconds
/// Processing time per sample:     942.26245 nanoseconds
/// Estimated CPU use:              4.155047%
/// Output hash (first 8 bytes):    ac fd ce 1e a2 7b 79 e1
/// Speed compared to std fallback: 2.3127007x
/// ```
fn main() -> Result<(), ()> {
    let mut all_hashes_match = true;

    #[allow(unused_variables)]
    let (success, fallback_std) = benchmark(
        "fallback (std)",
        "ad 0d 1d 04 5e 38 95 7f ",
        octasine::gen::FallbackStd::process_f32,
    );

    all_hashes_match &= success;

    #[cfg(feature = "simd")]
    {
        // Don't forget trailing space
        let hash = "ac fd ce 1e a2 7b 79 e1 ";

        {
            let (success, r) = benchmark(
                "fallback (sleef)",
                hash,
                octasine::gen::FallbackSleef::process_f32,
            );

            all_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }

        if is_x86_feature_detected!("sse2") {
            let (success, r) = benchmark("sse2", hash, octasine::gen::Sse2::process_f32);

            all_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }
        if is_x86_feature_detected!("avx") {
            let (success, r) = benchmark("avx", hash, octasine::gen::Avx::process_f32);

            all_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }
    }

    if all_hashes_match {
        println!("\n{}", "All output hashes matched".green());

        Ok(())
    } else {
        println!("\n{}", "Output hashes didn't match".red());

        Err(())
    }
}

fn benchmark(
    name: &str,
    expected_hash: &str,
    process_fn: unsafe fn(&mut OctaSine, &mut AudioBuffer<f32>),
) -> (bool, f32) {
    let mut octasine = OctaSine::default();

    let envelope_duration_parameters = [10i32, 12, 14, 24, 26, 28, 39, 41, 43, 54, 56, 58];

    let wave_type_parameters = [4i32, 17, 31, 46];

    const SIZE: usize = 256;

    let input_1 = vec![0.0f32; SIZE];
    let input_2 = input_1.clone();

    let mut output_1 = input_1.clone();
    let mut output_2 = input_1.clone();

    let inputs = vec![input_1.as_ptr(), input_2.as_ptr()];
    let mut outputs = vec![output_1.as_mut_ptr(), output_2.as_mut_ptr()];

    let mut buffer =
        unsafe { AudioBuffer::from_raw(2, 2, inputs.as_ptr(), outputs.as_mut_ptr(), SIZE) };

    let mut results = Sha256::new();

    let iterations = 50_000;

    for p in envelope_duration_parameters.iter() {
        octasine.sync.set_parameter(*p, 1.0);
    }
    for p in wave_type_parameters.iter() {
        octasine.sync.set_parameter(*p, 0.0);
    }

    let now = Instant::now();

    for i in 0..iterations {
        if i % 1024 == 0 {
            octasine.process_midi_event([144, 100, 100]);
            octasine.process_midi_event([144, 101, 100]);
            octasine.process_midi_event([144, 102, 100]);
            octasine.process_midi_event([144, 103, 100]);
        } else if i % 1024 == 768 {
            octasine.process_midi_event([128, 100, 0]);
            octasine.process_midi_event([128, 101, 0]);
            octasine.process_midi_event([128, 102, 0]);
            octasine.process_midi_event([128, 103, 0]);
        }

        for j in 0..60 {
            if envelope_duration_parameters.contains(&j) || wave_type_parameters.contains(&j) {
                continue;
            }

            octasine.sync.set_parameter(j, (i % 64) as f32 / 64.0);
        }

        unsafe {
            process_fn(&mut octasine, &mut buffer);
        }

        for (l, r) in output_1.iter().zip(output_2.iter()) {
            results.update(&l.to_ne_bytes());
            results.update(&r.to_ne_bytes());
        }
    }

    let elapsed = now.elapsed();

    let elapsed_millis = elapsed.as_millis();
    let num_samples = SIZE * iterations;
    let num_seconds = (SIZE * iterations) as f32 / 44100.0;

    let processing_time_per_sample = (elapsed.as_nanos() as f32 / SIZE as f32) / iterations as f32;

    println!();
    println!(
        "--- Benchmarking OctaSine process_f32 variant: {} ---",
        name
    );
    println!("Total number of samples:        {}", num_samples);
    println!("Equivalent to audio duration:   {} seconds", num_seconds);
    println!(
        "Processing time in total:       {} milliseconds",
        elapsed_millis
    );
    println!(
        "Processing time per sample:     {} nanoseconds",
        processing_time_per_sample
    );
    println!(
        "Estimated CPU use:              {}%",
        elapsed_millis as f32 / (num_seconds * 10.0)
    );

    let result_hash: String = results
        .finalize()
        .iter()
        .take(8)
        .map(|byte| format!("{:02x} ", byte))
        .collect();

    println!("Output hash (first 8 bytes):    {}", result_hash);

    let success = result_hash == expected_hash;

    let hash_match = if success { "yes".green() } else { "no".red() };

    println!("Hash match:                     {}", hash_match);

    (success, processing_time_per_sample)
}
