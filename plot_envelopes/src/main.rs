use plotlib::function::*;
use plotlib::view::ContinuousView;
use plotlib::page::Page;

use vst2_helpers::approximations::Log10Table;
use octasine::voices::VoiceOperatorVolumeEnvelope;


/// Generate plots to check how envelopes look.
/// 
/// Currently, editing the file to give the line a color is necessary.
fn main(){
    fn plot_envelope_stage(
        start_volume: f64,
        end_volume: f64,
        filename: &str
    ){
        let length = 1.0;

        let f = Function::new(|x| {
            VoiceOperatorVolumeEnvelope::calculate_curve(
                &Log10Table::default(),
                start_volume,
                end_volume,
                x as f64,
                length as f64,
            )
        }, 0., length);

        let v = ContinuousView::new()
            .add(&f)
            .x_range(0.0, length)
            .y_range(0.0, 1.0);
        
        Page::single(&v).save(&filename).unwrap();
    }

    plot_envelope_stage(0.0, 1.0, "attack.svg");
    plot_envelope_stage(0.5, 1.0, "decay.svg");
    plot_envelope_stage(1.0, 0.0, "release.svg");
}