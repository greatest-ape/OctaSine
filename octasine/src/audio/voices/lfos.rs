use crate::{
    audio::common::InterpolationDuration,
    common::*,
    parameters::{lfo_mode::LfoMode, lfo_shape::LfoShape},
};

const INTERPOLATION_DURATION_SHORT: InterpolationDuration = InterpolationDuration::exactly_10ms();
const INTERPOLATION_DURATION_LONG: InterpolationDuration = InterpolationDuration::approx_3ms();

#[derive(Debug, Clone)]
enum LfoStage {
    Interpolate {
        from_value: f32,
        samples_done: usize,
        samples_to_interpolate: usize,
    },
    Running,
    OneshotComplete,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct VoiceLfo {
    stage: LfoStage,
    current_shape: Option<LfoShape>,
    phase: Phase,
    last_value: f32,
    sample_rate: SampleRate,
}

impl Default for VoiceLfo {
    fn default() -> Self {
        let sample_rate = SampleRate::default();

        Self {
            stage: LfoStage::Stopped,
            current_shape: None,
            phase: Phase(0.0),
            last_value: 0.0,
            sample_rate,
        }
    }
}

impl VoiceLfo {
    pub fn advance_one_sample(
        &mut self,
        sample_rate: SampleRate,
        time_per_sample: TimePerSample,
        bpm_lfo_multiplier: BpmLfoMultiplier,
        shape: LfoShape,
        mode: LfoMode,
        frequency: f64,
    ) {
        if let LfoStage::Stopped | LfoStage::OneshotComplete = self.stage {
            return;
        }

        if self.current_shape.is_none() {
            self.current_shape = Some(shape);
        }

        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;

            // Restart interpolation
            self.stage = match self.stage {
                LfoStage::Interpolate { .. } => LfoStage::Interpolate {
                    from_value: self.last_value,
                    samples_done: 0,
                    samples_to_interpolate: INTERPOLATION_DURATION_SHORT.samples(sample_rate),
                },
                LfoStage::Running => LfoStage::Running,
                LfoStage::OneshotComplete | LfoStage::Stopped => unreachable!(),
            };
        }

        let new_phase = self.phase.0 + frequency * bpm_lfo_multiplier.0 * time_per_sample.0;

        self.phase.0 = new_phase.fract();

        match self.stage {
            LfoStage::Interpolate {
                from_value,
                mut samples_done,
                samples_to_interpolate,
            } => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.stage = LfoStage::OneshotComplete;
                    } else {
                        self.stage = LfoStage::Interpolate {
                            from_value: self.last_value,
                            samples_done: 0,
                            samples_to_interpolate: INTERPOLATION_DURATION_SHORT
                                .samples(self.sample_rate),
                        };
                    }
                } else {
                    samples_done += 1;

                    if samples_done == samples_to_interpolate {
                        self.stage = LfoStage::Running;
                    } else {
                        self.stage = LfoStage::Interpolate {
                            from_value,
                            samples_done,
                            samples_to_interpolate: INTERPOLATION_DURATION_SHORT
                                .samples(self.sample_rate),
                        };
                    }
                }
            }
            LfoStage::Running => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.stage = LfoStage::OneshotComplete;
                    } else {
                        match (self.current_shape, shape) {
                            (Some(LfoShape::Sine), LfoShape::Sine)
                            | (Some(LfoShape::ReverseSine), LfoShape::ReverseSine) => {}
                            _ => {
                                self.current_shape = Some(shape);

                                self.stage = LfoStage::Interpolate {
                                    from_value: self.last_value,
                                    samples_done: 0,
                                    samples_to_interpolate: INTERPOLATION_DURATION_SHORT
                                        .samples(self.sample_rate),
                                };
                            }
                        }
                    }
                }
            }
            LfoStage::OneshotComplete | LfoStage::Stopped => {
                unreachable!()
            }
        }
    }

    pub fn get_value(&mut self, amount: f32) -> f32 {
        if let LfoStage::Stopped = self.stage {
            return 0.0;
        }

        let shape = if let Some(shape) = self.current_shape {
            shape
        } else {
            return 0.0;
        };

        let value = match self.stage {
            LfoStage::Interpolate {
                from_value,
                samples_done,
                samples_to_interpolate,
            } => {
                let progress = samples_done as f32 / samples_to_interpolate as f32;

                progress * shape.calculate(self.phase) + (1.0 - progress) * from_value
            }
            LfoStage::Running => shape.calculate(self.phase),
            LfoStage::OneshotComplete => self.last_value,
            LfoStage::Stopped => {
                unreachable!()
            }
        };

        self.last_value = value;

        value * amount
    }

    pub fn restart(&mut self) {
        self.phase = Phase(0.0);
        self.current_shape = None;

        match self.stage {
            LfoStage::Stopped => {
                self.stage = LfoStage::Interpolate {
                    from_value: 0.0,
                    samples_done: 0,
                    samples_to_interpolate: INTERPOLATION_DURATION_SHORT.samples(self.sample_rate),
                };
            }
            LfoStage::OneshotComplete => {
                self.stage = LfoStage::Interpolate {
                    from_value: self.last_value,
                    samples_done: 0,
                    samples_to_interpolate: INTERPOLATION_DURATION_LONG.samples(self.sample_rate),
                };
            }
            _ => {
                self.stage = LfoStage::Interpolate {
                    from_value: self.last_value,
                    samples_done: 0,
                    samples_to_interpolate: INTERPOLATION_DURATION_SHORT.samples(self.sample_rate),
                };
            }
        }
    }

    pub fn envelope_ended(&mut self) {
        self.stage = LfoStage::Stopped;
        self.last_value = 0.0;
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.stage, LfoStage::Stopped)
    }
}
