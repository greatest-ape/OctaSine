use std::time::Instant;

use colored::*;
use sha2::{Digest, Sha256};
use vst::event::MidiEvent;
use vst::plugin::PluginParameters;

use octasine::gen::simd::Simd;
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
fn main() {
    // Ignore success status here, since output differs across platforms
    // depending on std sine implementation
    #[allow(unused_variables)]
    let (_, fallback_std) =
        benchmark::<octasine::gen::simd::FallbackStd>("fallback (std)", "30 2e 0a ed c3 ec 10 83 ");

    #[allow(unused_variables, unused_mut)]
    let mut all_sleef_hashes_match = true;

    #[cfg(feature = "simd")]
    {
        // Don't forget trailing space
        let hash = "30 2e 0a ed c3 ec 10 83 ";

        {
            let (success, r) =
                benchmark::<octasine::gen::simd::FallbackSleef>("fallback (sleef)", hash);

            all_sleef_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }

        if is_x86_feature_detected!("sse2") {
            let (success, r) = benchmark::<octasine::gen::simd::Sse2>("sse2", hash);

            all_sleef_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }
        if is_x86_feature_detected!("avx") {
            let (success, r) = benchmark::<octasine::gen::simd::Avx>("avx", hash);

            all_sleef_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }
    }

    if all_sleef_hashes_match {
        println!("\n{}", "All sleef output hashes matched".green());
    } else {
        println!("\n{}", "Sleef output hashes didn't match".red());
    }
}

fn benchmark<A: AudioGen + Simd>(name: &str, expected_hash: &str) -> (bool, f32) {
    let mut octasine = OctaSine::default();

    let envelope_duration_parameters = [10i32, 12, 14, 24, 26, 28, 39, 41, 43, 54, 56, 58];

    let wave_type_parameters = [4i32, 17, 31, 46];

    const SIZE: usize = 256;

    let mut lefts = vec![0.0f32; SIZE];
    let mut rights = vec![0.0f32; SIZE];

    let mut results = Sha256::new();

    let iterations = 5_000;

    for p in envelope_duration_parameters.iter() {
        octasine.sync.set_parameter(*p, 0.1);
    }
    for p in wave_type_parameters.iter() {
        octasine.sync.set_parameter(*p, 0.0);
    }

    let now = Instant::now();

    let key_on_events: Vec<MidiEvent> = (0..=3usize)
        .map(|i| MidiEvent {
            data: [144, 100 + i as u8, 100],
            delta_frames: i as i32,
            live: false,
            note_length: None,
            note_offset: None,
            detune: 0,
            note_off_velocity: 0,
        })
        .collect();

    let key_off_events: Vec<MidiEvent> = (0..=3usize)
        .map(|i| MidiEvent {
            data: [128, 100 + i as u8, 0],
            delta_frames: i as i32,
            live: false,
            note_length: None,
            note_offset: None,
            detune: 0,
            note_off_velocity: 0,
        })
        .collect();

    for i in 0..iterations {
        if i % 1024 == 0 {
            octasine.enqueue_midi_events(key_on_events.iter().copied());
        } else if i % 1024 == 512 {
            octasine.enqueue_midi_events(key_off_events.iter().copied());
        }

        for j in 0..87 {
            if envelope_duration_parameters.contains(&j) || wave_type_parameters.contains(&j) {
                continue;
            }

            octasine.sync.set_parameter(j, (i % 64) as f32 / 64.0);
        }

        octasine.update_processing_parameters();

        let mut position = 0usize;

        for (lefts, rights) in lefts
            .chunks_exact_mut(A::SAMPLES)
            .zip(rights.chunks_exact_mut(A::SAMPLES))
        {
            unsafe {
                A::process_f32(&mut octasine, lefts, rights, position);
            }

            position += A::SAMPLES;
        }

        for (l, r) in lefts.iter().zip(rights.iter()) {
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
