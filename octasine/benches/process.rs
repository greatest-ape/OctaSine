use std::time::Instant;

use vst::buffer::AudioBuffer;
use vst::plugin::PluginParameters;
use sha2::{Digest, Sha256};

use octasine::OctaSine;


/// Benchmark OctaSine process functions
/// 
/// Example output:
/// ```txt
/// --- Benchmarking OctaSine process_f32 variant: reference ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     36677 milliseconds
/// Processing time per sample:   2865.3987 nanoseconds
/// Estimated CPU use:            12.6363735%
/// Output hash (first 16 bytes): ab db fa d4 4d bd 62 48 
///                               6f 75 2c 02 61 d7 ba a9 
/// 
/// --- Benchmarking OctaSine process_f32 variant: portable ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     27870 milliseconds
/// Processing time per sample:   2177.369 nanoseconds
/// Estimated CPU use:            9.602087%
/// Output hash (first 16 bytes): ac fd ce 1e a2 7b 79 e1 
///                               75 06 b6 94 fe be c9 5f 
/// Speed compared to reference:  1.3159914x
/// 
/// --- Benchmarking OctaSine process_f32 variant: sse2 ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     18010 milliseconds
/// Processing time per sample:   1407.0741 nanoseconds
/// Estimated CPU use:            6.205008%
/// Output hash (first 16 bytes): ac fd ce 1e a2 7b 79 e1 
///                               75 06 b6 94 fe be c9 5f 
/// Speed compared to reference:  2.0364234x
/// 
/// --- Benchmarking OctaSine process_f32 variant: avx ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     11717 milliseconds
/// Processing time per sample:   915.4168 nanoseconds
/// Estimated CPU use:            4.036873%
/// Output hash (first 16 bytes): ac fd ce 1e a2 7b 79 e1 
///                               75 06 b6 94 fe be c9 5f 
/// Speed compared to reference:  3.1301575x
/// ```
fn main(){
    // Unsafe because process_fn argument is unsafe, which is necessary for simd functions
    #[inline]
    unsafe fn reference(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>){
        octasine::gen::reference::process_f32(octasine, audio_buffer);
    }

    #[allow(unused_variables)]
    let reference = benchmark("reference", reference);

    #[cfg(feature = "simd")]
    {
        use octasine::gen::simd::AudioGen;

        {
            let r = benchmark("portable", octasine::gen::simd::Portable::process_f32);
            println!("Speed compared to reference:  {}x", reference / r);
        }

        if is_x86_feature_detected!("sse2") {
            let r = benchmark("sse2", octasine::gen::simd::Sse2::process_f32);
            println!("Speed compared to reference:  {}x", reference / r);
        }
        if is_x86_feature_detected!("avx") {
            let r = benchmark("avx", octasine::gen::simd::Avx::process_f32);
            println!("Speed compared to reference:  {}x", reference / r);
        }
    }
}


fn benchmark(
    name: &str,
    process_fn: unsafe fn(&mut OctaSine, &mut AudioBuffer<f32>)
) -> f32 {
    let mut octasine = OctaSine::default();

    let envelope_duration_parameters =
        [10i32, 12, 14, 24, 26, 28, 39, 41, 43, 54, 56, 58];
    
    let wave_type_parameters = [4i32, 17, 31, 46];

    const SIZE: usize = 256;

    let input_1 = vec![0.0f32; SIZE];
    let input_2 = input_1.clone();

    let mut output_1 = input_1.clone();
    let mut output_2 = input_1.clone();

    let inputs = vec![input_1.as_ptr(), input_2.as_ptr()];
    let mut outputs = vec![output_1.as_mut_ptr(), output_2.as_mut_ptr()];
    
    let mut buffer = unsafe {
        AudioBuffer::from_raw(
            2,
            2,
            inputs.as_ptr(),
            outputs.as_mut_ptr(),
            SIZE
        )
    };

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
        }
        else if i % 1024 == 768 {
            octasine.process_midi_event([128, 100, 0]);
            octasine.process_midi_event([128, 101, 0]);
            octasine.process_midi_event([128, 102, 0]);
            octasine.process_midi_event([128, 103, 0]);
        }

        for j in 0..60 {
            if envelope_duration_parameters.contains(&j) || wave_type_parameters.contains(&j){
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

    let processing_time_per_sample = 
        (elapsed.as_nanos() as f32 / SIZE as f32) / iterations as f32;

    println!();
    println!("--- Benchmarking OctaSine process_f32 variant: {} ---", name);
    println!("Total number of samples:      {}", num_samples);
    println!("Equivalent to audio duration: {} seconds", num_seconds);
    println!("Processing time in total:     {} milliseconds", elapsed_millis);
    println!("Processing time per sample:   {} nanoseconds",
        processing_time_per_sample);
    println!("Estimated CPU use:            {}%",
        elapsed_millis as f32 / (num_seconds * 10.0));

    let result_hash: String = results.finalize().iter()
        .take(16)
        .enumerate()
        .map(|(i, byte)| {
            if i == 0 {
                format!("Output hash (first 16 bytes): {:02x} ", byte)
            } else if i % 8 == 0 {
                format!("\n                              {:02x} ", byte)
            } else {
                format!("{:02x} ", byte)
            }
        })
        .collect();

    println!("{}", result_hash);

    processing_time_per_sample
}
