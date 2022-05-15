use std::time::Instant;

use colored::*;
use octasine::parameters::{OperatorParameter, Parameter, PARAMETERS};
use sha2::{Digest, Sha256};
use vst::event::MidiEvent;
use vst::plugin::PluginParameters;

use octasine::audio::gen::simd::Simd;
use octasine::audio::gen::AudioGen;
use octasine::OctaSine;

/// Benchmark OctaSine process functions and check sample-accurate output
///
/// Example output:
/// ```txt
/// --- Benchmarking OctaSine process_f32 variant: fallback (std) ---
/// Total number of samples:        1280000
/// Equivalent to audio duration:   29.024942 seconds
/// Processing time in total:       2364 milliseconds
/// Processing time per sample:     1847.5138 nanoseconds
/// Estimated CPU use:              8.144719%
/// Output hash (first 8 bytes):    3a 05 72 a0 77 76 49 0a
/// Hash match:                     yes
///
/// --- Benchmarking OctaSine process_f32 variant: fallback (sleef) ---
/// Total number of samples:        1280000
/// Equivalent to audio duration:   29.024942 seconds
/// Processing time in total:       1639 milliseconds
/// Processing time per sample:     1280.9581 nanoseconds
/// Estimated CPU use:              5.6468673%
/// Output hash (first 8 bytes):    3a 05 72 a0 77 76 49 0a
/// Hash match:                     yes
/// Speed compared to std fallback: 1.4422905x
///
/// --- Benchmarking OctaSine process_f32 variant: sse2 ---
/// Total number of samples:        1280000
/// Equivalent to audio duration:   29.024942 seconds
/// Processing time in total:       1449 milliseconds
/// Processing time per sample:     1132.5383 nanoseconds
/// Estimated CPU use:              4.992258%
/// Output hash (first 8 bytes):    3a 05 72 a0 77 76 49 0a
/// Hash match:                     yes
/// Speed compared to std fallback: 1.6313035x
///
/// --- Benchmarking OctaSine process_f32 variant: avx ---
/// Total number of samples:        1280000
/// Equivalent to audio duration:   29.024942 seconds
/// Processing time in total:       985 milliseconds
/// Processing time per sample:     770.06256 nanoseconds
/// Estimated CPU use:              3.393633%
/// Output hash (first 8 bytes):    3a 05 72 a0 77 76 49 0a
/// Hash match:                     yes
/// Speed compared to std fallback: 2.3991737x
///
/// All sleef output hashes matched
/// ```
fn main() {
    // Ignore success status here, since output differs across platforms
    // depending on std sine implementation
    #[allow(unused_variables)]
    let (_, fallback_std) = benchmark::<octasine::audio::gen::simd::FallbackStd>(
        "fallback (std)",
        "59 ee 27 9b d0 2f 9c 08 ",
    );

    #[allow(unused_variables, unused_mut)]
    let mut all_sleef_hashes_match = true;

    #[cfg(feature = "simd")]
    {
        // Don't forget trailing space
        let hash = "59 ee 27 9b d0 2f 9c 08 ";

        {
            let (success, r) =
                benchmark::<octasine::audio::gen::simd::FallbackSleef>("fallback (sleef)", hash);

            all_sleef_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }

        if is_x86_feature_detected!("sse2") {
            let (success, r) = benchmark::<octasine::audio::gen::simd::Sse2>("sse2", hash);

            all_sleef_hashes_match &= success;

            println!("Speed compared to std fallback: {}x", fallback_std / r);
        }
        if is_x86_feature_detected!("avx") {
            let (success, r) = benchmark::<octasine::audio::gen::simd::Avx>("avx", hash);

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

    let envelope_duration_parameters: Vec<Parameter> = (0..4)
        .map(|i| {
            vec![
                Parameter::Operator(i, OperatorParameter::AttackDuration),
                Parameter::Operator(i, OperatorParameter::DecayDuration),
                Parameter::Operator(i, OperatorParameter::ReleaseDuration),
            ]
        })
        .flatten()
        .collect();

    let wave_type_parameters: Vec<Parameter> = (0..4)
        .map(|i| Parameter::Operator(i, OperatorParameter::WaveType))
        .collect();

    const SIZE: usize = 256;

    let mut lefts = vec![0.0f32; SIZE];
    let mut rights = vec![0.0f32; SIZE];

    let mut results = Sha256::new();

    let iterations = 5_000;

    for p in envelope_duration_parameters.iter() {
        octasine.sync.set_parameter(p.to_index() as i32, 0.1);
    }
    for p in wave_type_parameters.iter() {
        octasine.sync.set_parameter(p.to_index() as i32, 0.0);
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
            octasine
                .audio
                .enqueue_midi_events(key_on_events.iter().copied());
        } else if i % 1024 == 512 {
            octasine
                .audio
                .enqueue_midi_events(key_off_events.iter().copied());
        }

        for parameter in PARAMETERS.iter() {
            if envelope_duration_parameters.contains(parameter)
                || wave_type_parameters.contains(parameter)
            {
                continue;
            }

            octasine
                .sync
                .set_parameter(parameter.to_index() as i32, (i % 64) as f32 / 64.0);
        }

        octasine.update_audio_parameters();

        let mut position = 0usize;

        for (lefts, rights) in lefts
            .chunks_exact_mut(A::SAMPLES)
            .zip(rights.chunks_exact_mut(A::SAMPLES))
        {
            unsafe {
                A::process_f32(&mut octasine.audio, lefts, rights, position);
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
