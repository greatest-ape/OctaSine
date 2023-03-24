use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointMarker, PointStyle};
use plotlib::view::{ContinuousView, View};

use octasine::audio::parameters::OperatorEnvelopeAudioParameters;
use octasine::audio::voices::envelopes::VoiceOperatorVolumeEnvelope;
use octasine::audio::voices::lfos::*;
use octasine::audio::voices::log10_table::Log10Table;
use octasine::common::*;
use octasine::parameters::lfo_mode::LfoMode;
use octasine::parameters::lfo_shape::LfoShape;

pub fn run() -> anyhow::Result<()> {
    // plot_lfo_values("tmp/lfo.svg");
    plot_square("tmp/square-wave.svg");
    plot_triangle("tmp/triangle-wave.svg");
    plot_saw("tmp/saw-wave.svg");
    // plot_envelope_stage(0.1, 0.0, 1.0, "tmp/attack.svg");

    Ok(())
}

fn plot_saw(filename: &str) {
    use octasine::simd::SimdPackedDouble;

    fn make_plot<Simd: SimdPackedDouble>(color: &str) -> Plot {
        let start = -2.0;
        let end = 2.0;

        let step_size = (end - start) / 2000.;

        let data = (0..)
            .map(|x| start + (f64::from(x) * step_size))
            .take_while(|&x| x <= end)
            .map(|s| {
                let saw = unsafe { Simd::new(s).saw().to_arr()[0] };

                (s, saw)
            })
            .collect();

        Plot::new(data).line_style(LineStyle::new().colour(color).width(0.2))
    }

    let fallback = make_plot::<octasine::simd::FallbackPackedDouble>("green");
    let sse2 = make_plot::<octasine::simd::Sse2PackedDouble>("red");
    let avx = make_plot::<octasine::simd::AvxPackedDouble>("blue");

    let mut v = ContinuousView::new()
        .add(fallback)
        .add(sse2)
        .add(avx)
        .x_range(-2.0, 2.0)
        .y_range(-2.0, 2.0);

    v.add_grid(Grid::new(4, 4));

    Page::single(&v).save(&filename).unwrap();
}

fn plot_square(filename: &str) {
    use octasine::simd::SimdPackedDouble;

    fn make_plot<Simd: SimdPackedDouble>(color: &str) -> Plot {
        let start = -2.0;
        let end = 2.0;

        let step_size = (end - start) / 2000.;

        let data = (0..)
            .map(|x| start + (f64::from(x) * step_size))
            .take_while(|&x| x <= end)
            .map(|s| {
                let square = unsafe { Simd::new(s).square().to_arr()[0] };

                (s, square)
            })
            .collect();

        Plot::new(data).line_style(LineStyle::new().colour(color).width(0.2))
    }

    let fallback = make_plot::<octasine::simd::FallbackPackedDouble>("green");
    let sse2 = make_plot::<octasine::simd::Sse2PackedDouble>("red");
    let avx = make_plot::<octasine::simd::AvxPackedDouble>("blue");

    let mut v = ContinuousView::new()
        .add(fallback)
        .add(sse2)
        .add(avx)
        .x_range(-2.0, 2.0)
        .y_range(-2.0, 2.0);

    v.add_grid(Grid::new(4, 4));

    Page::single(&v).save(&filename).unwrap();
}

fn plot_triangle(filename: &str) {
    use octasine::simd::SimdPackedDouble;

    fn make_plot<Simd: SimdPackedDouble>(color: &str) -> Plot {
        let start = -2.0;
        let end = 2.0;

        let step_size = (end - start) / 2000.;

        let data = (0..)
            .map(|x| start + (f64::from(x) * step_size))
            .take_while(|&x| x <= end)
            .map(|s| {
                let triangle = unsafe { Simd::new(s).triangle().to_arr()[0] };

                (s, triangle)
            })
            .collect();

        Plot::new(data).line_style(LineStyle::new().colour(color).width(0.2))
    }

    let fallback = make_plot::<octasine::simd::FallbackPackedDouble>("green");
    let sse2 = make_plot::<octasine::simd::Sse2PackedDouble>("red");
    let avx = make_plot::<octasine::simd::AvxPackedDouble>("blue");

    let mut v = ContinuousView::new()
        .add(fallback)
        .add(sse2)
        .add(avx)
        .x_range(-2.0, 2.0)
        .y_range(-2.0, 2.0);

    v.add_grid(Grid::new(4, 4));

    Page::single(&v).save(&filename).unwrap();
}

#[allow(dead_code)]
fn plot_envelope_stage(length: f64, start_volume: f64, end_volume: f64, filename: &str) {
    let log10table = Log10Table::default();

    let plot = Plot::from_function(
        |x| {
            VoiceOperatorVolumeEnvelope::calculate_curve(
                &log10table,
                start_volume as f32,
                end_volume as f32,
                x as f64,
                length,
            ) as f64
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

/*
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

    let sample_rate = SampleRate(44100.0);
    let time_per_sample = sample_rate.into();
    let bpm_lfo_multiplier = BeatsPerMinute(120.0).into();
    let shape = LfoShape::Saw;
    let mode = LfoMode::Forever;
    let speed = 2.0;
    let magnitude = 1.0;

    let log10table = Log10Table::default();
    let mut lfo = VoiceLfo::default();
    let mut envelope = VoiceOperatorVolumeEnvelope::default();
    let mut processing_parameter_envelope = OperatorEnvelopeAudioParameters::default();
    let mut key_pressed = false;

    let mut lfo_value_points = Vec::with_capacity(num_samples);
    let mut restart_points = Vec::new();
    let mut seconds_points = Vec::new();

    let mut envelope_value_points = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        lfo.advance_one_sample(
            sample_rate,
            time_per_sample,
            bpm_lfo_multiplier,
            shape,
            mode,
            speed,
        );

        let lfo_value = lfo.get_value(magnitude);

        lfo_value_points.push((i as f64, lfo_value as f64));

        envelope.advance_one_sample(
            &mut processing_parameter_envelope,
            key_pressed,
            time_per_sample,
        );

        let envelope_value = envelope.get_volume(&log10table, &mut processing_parameter_envelope);

        envelope_value_points.push((i as f64, envelope_value as f64));

        if i % 44100 == 0 {
            seconds_points.push((i as f64, 0.0));
        }

        if press_key_at_samples.contains(&i) {
            lfo.restart();

            key_pressed = true;
            envelope.restart();

            restart_points.push((i as f64, 0.0));
            restart_points.push((i as f64, 1.0));
        } else if release_key_at_samples.contains(&i) {
            key_pressed = false;
        }

        if envelope.is_ended() {
            lfo.envelope_ended();
        }
    }

    let value_product_points = lfo_value_points
        .iter()
        .zip(envelope_value_points.iter())
        .map(|((lfo_x, lfo_y), (_, envelope_y))| (*lfo_x, lfo_y * envelope_y))
        .collect::<Vec<(f64, f64)>>();

    /*
    let value_product_points_nonzero_base = lfo_value_points
        .iter()
        .zip(envelope_value_points.iter())
        .map(|((lfo_x, lfo_y), (_, envelope_y))| (*lfo_x, (1.0 + lfo_y) * envelope_y))
        .collect::<Vec<(f64, f64)>>();
    */

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

    // let value_product_plot_nonzero_base = Plot::new(value_product_points_nonzero_base)
    //     .line_style(LineStyle::new().colour("orange").width(0.1));

    let lfo_values_plot =
        Plot::new(lfo_value_points).line_style(LineStyle::new().colour("green").width(0.1));

    let envelope_values_plot =
        Plot::new(envelope_value_points).line_style(LineStyle::new().colour("blue").width(0.1));

    let v = ContinuousView::new()
        .add(seconds_plot)
        .add(restarts_plot)
        .add(value_product_plot)
        // .add(value_product_plot_nonzero_base)
        .add(lfo_values_plot)
        .add(envelope_values_plot)
        .y_range(-2.0, 2.0);

    Page::single(&v).save(&filename).unwrap();
}
*/
