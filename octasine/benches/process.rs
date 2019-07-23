use std::time::Instant;

use vst::buffer::AudioBuffer;
use vst::plugin::Plugin;
use vst::plugin::HostCallback;
use vst::plugin::PluginParameters;

use octasine::*;


/// Benchmark `process` method
fn main(){
    let mut fm = OctaSine::new(HostCallback::default());

    let envelope_duration_parameters =
        [10i32, 12, 14, 24, 26, 28, 39, 41, 43, 54, 56, 58];
    
    let wave_type_parameters = [4i32, 17, 31, 46];

    const SIZE: usize = 512;

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

    let iterations = 10_000;

    for p in envelope_duration_parameters.iter() {
        fm.sync_only.set_parameter(*p, 1.0);
    }
    for p in wave_type_parameters.iter() {
        fm.sync_only.set_parameter(*p, 0.0);
    }

    let now = Instant::now();

    for i in 0..iterations {
        if i % 1024 == 0 {
            fm.process_midi_event([144, 100, 100]);
            // fm.process_midi_event([144, 101, 100]);
            // fm.process_midi_event([144, 102, 100]);
            // fm.process_midi_event([144, 103, 100]);
        }
        else if i % 1024 == 768 {
            fm.process_midi_event([128, 100, 0]);
            // fm.process_midi_event([128, 101, 0]);
            // fm.process_midi_event([128, 102, 0]);
            // fm.process_midi_event([128, 103, 0]);
        }

        for j in 0..60 {
            if envelope_duration_parameters.contains(&j) || wave_type_parameters.contains(&j){
                continue;
            }

            fm.sync_only.set_parameter(j, (i % 64) as f32 / 64 as f32);
        }

        fm.process(&mut buffer);

        for samples in output_1.iter().zip(output_2.iter()) {
            results.push(samples);
        }
    }

    let elapsed = now.elapsed();

    let elapsed_millis = elapsed.as_millis();
    let num_samples = SIZE * iterations;
    let audio_for = (SIZE * iterations) as f32 / 44100.0;

    println!("--- Benchmarking process() ---");
    println!("Samples:           {}", num_samples);
    println!("Audio for:         {} seconds", audio_for);
    println!("Processing time:   {} milliseconds", elapsed_millis);
    println!("Time per sample:   {} nanoseconds",
        (elapsed.as_nanos() as f32 / SIZE as f32) / iterations as f32);
    println!("Estimated CPU use: {}%",
        elapsed_millis as f32 / (audio_for * 10.0));
    
    let mut bla = 0;

    for (a, b) in results {
        if a == b {
            bla += 1;
        }
    }

    if bla > iterations {
        println!("blabla: {}", bla);
    }
}