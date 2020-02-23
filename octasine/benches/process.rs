use std::time::Instant;

use vst::buffer::AudioBuffer;
use vst::plugin::Plugin;
use vst::plugin::HostCallback;
use vst::plugin::PluginParameters;

use octasine::*;


/// Benchmark OctaSine process functions
/// 
/// Example output:
/// ```txt
/// --- Benchmarking OctaSine process_f32 variant: fallback ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     23393 milliseconds
/// Processing time per sample:   1827.6355 nanoseconds
/// Estimated CPU use:            8.05962%
/// 
/// --- Benchmarking OctaSine process_f32 variant: sse2 ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     17536 milliseconds
/// Processing time per sample:   1370.048 nanoseconds
/// Estimated CPU use:            6.0417004%
/// Speed compared to fallback:   1.3339938x
/// 
/// --- Benchmarking OctaSine process_f32 variant: sse41 ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     17359 milliseconds
/// Processing time per sample:   1356.2168 nanoseconds
/// Estimated CPU use:            5.980718%
/// Speed compared to fallback:   1.3475983x
/// 
/// --- Benchmarking OctaSine process_f32 variant: avx ---
/// Total number of samples:      12800000
/// Equivalent to audio duration: 290.24942 seconds
/// Processing time in total:     10977 milliseconds
/// Processing time per sample:   857.6477 nanoseconds
/// Estimated CPU use:            3.7819197%
/// Speed compared to fallback:   2.1309862x
/// ```
fn main(){
    // Unsafe because process_fn argument is unsafe, which is necessary for simd functions
    #[inline]
    unsafe fn fallback(octasine: &mut OctaSine, audio_buffer: &mut AudioBuffer<f32>){
        octasine::gen::fallback::process_f32(octasine, audio_buffer);
    }

    #[allow(unused_variables)]
    let reference = benchmark("fallback", fallback);

    #[cfg(feature = "simd2")]
    {
        if is_x86_feature_detected!("sse2") {
            let r = benchmark("sse2", octasine::gen::simd::process_f32_sse2);
            println!("Speed compared to fallback:   {}x", reference / r);
        }
        if is_x86_feature_detected!("sse4.1") {
            let r = benchmark("sse41", octasine::gen::simd::process_f32_sse41);
            println!("Speed compared to fallback:   {}x", reference / r);
        }
        if is_x86_feature_detected!("avx") {
            let r = benchmark("avx", octasine::gen::simd::process_f32_avx);
            println!("Speed compared to fallback:   {}x", reference / r);
        }
        if is_x86_feature_detected!("avx2") {
            let r = benchmark("avx2", octasine::gen::simd::process_f32_avx2);
            println!("Speed compared to fallback:   {}x", reference / r);
        }
    }
}


fn benchmark(
    name: &str,
    process_fn: unsafe fn(&mut OctaSine, &mut AudioBuffer<f32>)
) -> f32 {
    let mut octasine = OctaSine::new(HostCallback::default());

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

    let mut results = Vec::new();

    let iterations = 50_000;

    for p in envelope_duration_parameters.iter() {
        octasine.sync_only.set_parameter(*p, 1.0);
    }
    for p in wave_type_parameters.iter() {
        octasine.sync_only.set_parameter(*p, 0.0);
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

            octasine.sync_only.set_parameter(j, (i % 64) as f32 / 64 as f32);
        }

        unsafe {
            process_fn(&mut octasine, &mut buffer);
        }

        for samples in output_1.iter().zip(output_2.iter()) {
            results.push(samples);
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
    
    let mut bla = 0;

    for (a, b) in results {
        if a == b {
            bla += 1;
        }
    }

    if bla == iterations {
        println!("dummy info: {}", bla);
    }

    processing_time_per_sample
}