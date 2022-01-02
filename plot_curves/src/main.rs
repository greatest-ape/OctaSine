use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

use octasine::common::*;
use octasine::parameters::processing::OperatorEnvelopeProcessingParameter;
use octasine::voices::envelopes::VoiceOperatorVolumeEnvelope;
use octasine::voices::lfos::*;
use octasine::voices::VoiceDuration;

#[allow(dead_code)]
fn plot_envelope_stage(start_volume: f64, end_volume: f64, filename: &str) {
    let length = 1.0;

    let plot = Plot::from_function(
        |x| {
            VoiceOperatorVolumeEnvelope::calculate_curve(start_volume, end_volume, x as f64, length)
        },
        0.,
        length,
    )
    .line_style(LineStyle::new().colour("green"));

    let v = ContinuousView::new()
        .add(plot)
        .x_range(0.0, length)
        .y_range(0.0, 1.0);

    Page::single(&v).save(&filename).unwrap();
}

fn plot_lfo_values(filename: &str) {
    let press_key_at_samples = vec![
        15000,
        35000, // ok restart
        44100 + 25000,
        44100 + 35000, // bad restart
    ];
    let release_key_at_samples: Vec<usize> = press_key_at_samples
        .iter()
        .map(|sample| sample + 1000)
        .collect();

    let num_samples = 44_100usize * 4;

    let time_per_sample = TimePerSample(1.0 / 44100.0);
    let bpm = BeatsPerMinute(120.0);
    let shape = LfoShape::Saw;
    let mode = LfoMode::Forever;
    let speed = 2.0;
    let magnitude = 1.0;

    let mut lfo = VoiceLfo::default();
    let mut envelope = VoiceOperatorVolumeEnvelope::default();
    let mut processing_parameter_envelope = OperatorEnvelopeProcessingParameter::default();
    let mut voice_duration = VoiceDuration(0.0);
    let mut key_pressed = false;

    let mut lfo_value_points = Vec::with_capacity(num_samples);
    let mut restart_points = Vec::new();
    let mut seconds_points = Vec::new();

    let mut envelope_value_points = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let lfo_value = lfo.get_value(time_per_sample.0, bpm, shape, mode, speed, magnitude);

        lfo_value_points.push((i as f64, lfo_value));

        envelope.advance_one_sample(
            &mut processing_parameter_envelope,
            key_pressed,
            time_per_sample,
        );

        let envelope_value = envelope.get_volume(&mut processing_parameter_envelope);

        envelope_value_points.push((i as f64, envelope_value));

        if i % 44100 == 0 {
            seconds_points.push((i as f64, 0.0));
        }

        if press_key_at_samples.contains(&i) {
            lfo.restart();

            key_pressed = true;
            envelope.restart();

            voice_duration.0 = 0.0;

            restart_points.push((i as f64, 0.0));
            restart_points.push((i as f64, 1.0));
        } else if release_key_at_samples.contains(&i) {
            key_pressed = false;
        }

        if envelope.is_ended() {
            lfo.stop();
        }

        voice_duration.0 += time_per_sample.0;
    }

    let value_product_points = lfo_value_points
        .iter()
        .zip(envelope_value_points.iter())
        .map(|((lfo_x, lfo_y), (_, envelope_y))| (*lfo_x, lfo_y * envelope_y))
        .collect::<Vec<(f64, f64)>>();

    let value_product_points_nonzero_base = lfo_value_points
        .iter()
        .zip(envelope_value_points.iter())
        .map(|((lfo_x, lfo_y), (_, envelope_y))| (*lfo_x, (1.0 + lfo_y) * envelope_y))
        .collect::<Vec<(f64, f64)>>();

    let seconds_plot = Plot::new(seconds_points).point_style(
        PointStyle::new()
            .marker(PointMarker::Circle)
            .colour("#aaa")
            .size(1.0),
    );

    let restarts_plot = Plot::new(restart_points).point_style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#ccc")
            .size(1.0),
    );

    let value_product_plot =
        Plot::new(value_product_points).line_style(LineStyle::new().colour("red").width(0.1));

    let value_product_plot_nonzero_base = Plot::new(value_product_points_nonzero_base)
        .line_style(LineStyle::new().colour("orange").width(0.1));

    let lfo_values_plot =
        Plot::new(lfo_value_points).line_style(LineStyle::new().colour("green").width(0.1));

    let envelope_values_plot =
        Plot::new(envelope_value_points).line_style(LineStyle::new().colour("blue").width(0.1));

    let v = ContinuousView::new()
        .add(seconds_plot)
        .add(restarts_plot)
        .add(value_product_plot)
        .add(value_product_plot_nonzero_base)
        .add(lfo_values_plot)
        .add(envelope_values_plot)
        .y_range(-2.0, 2.0);

    Page::single(&v).save(&filename).unwrap();
}

fn main() {
    // plot_lfo_values("tmp/lfo.svg");

    plot_envelope_stage(0.0, 1.0, "tmp/attack.svg");
    plot_envelope_stage(0.5, 1.0, "tmp/decay.svg");
    plot_envelope_stage(1.0, 0.0, "tmp/release.svg");
}
