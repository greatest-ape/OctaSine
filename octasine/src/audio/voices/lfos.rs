use crate::{
    audio::common::InterpolationDuration,
    common::*,
    parameters::{lfo_mode::LfoMode, lfo_shape::LfoShape},
};

const INTERPOLATION_DURATION: InterpolationDuration = InterpolationDuration::approx_3ms();

#[derive(Debug, Clone)]
enum LfoStage {
    Interpolate {
        from_value: f64,
        samples_done: usize,
    },
    Running,
    Stopping {
        from_value: f64,
        samples_done: usize,
    },
    Stopped,
}

#[derive(Debug, Clone)]
pub struct VoiceLfo {
    stage: LfoStage,
    current_shape: Option<LfoShape>,
    phase: Phase,
    last_value: f64,
    sample_rate: SampleRate,
    samples_to_interpolate: usize,
}

impl Default for VoiceLfo {
    fn default() -> Self {
        let sample_rate = SampleRate::default();
        let samples_to_interpolate = INTERPOLATION_DURATION.samples(sample_rate);

        Self {
            stage: LfoStage::Stopped,
            current_shape: None,
            phase: Phase(0.0),
            last_value: 0.0,
            sample_rate,
            samples_to_interpolate,
        }
    }
}

impl VoiceLfo {
    pub fn advance_one_sample(
        &mut self,
        sample_rate: SampleRate,
        time_per_sample: TimePerSample,
        bpm: BeatsPerMinute,
        shape: LfoShape,
        mode: LfoMode,
        frequency: f64,
    ) {
        if let LfoStage::Stopped = self.stage {
            return;
        }

        if self.current_shape.is_none() {
            self.current_shape = Some(shape);
        }

        if self.sample_rate != sample_rate {
            self.sample_rate = sample_rate;
            self.samples_to_interpolate = INTERPOLATION_DURATION.samples(sample_rate);

            // Restart interpolation
            self.stage = match self.stage {
                LfoStage::Interpolate { .. } => LfoStage::Interpolate {
                    from_value: self.last_value,
                    samples_done: 0,
                },
                LfoStage::Running => LfoStage::Running,
                LfoStage::Stopping { .. } => LfoStage::Stopping {
                    from_value: self.last_value,
                    samples_done: 0,
                },
                LfoStage::Stopped => unreachable!(),
            };
        }

        let new_phase = self.phase.0 + frequency * (bpm.0 / 120.0) * time_per_sample.0;

        self.phase.0 = new_phase.fract();

        match self.stage {
            LfoStage::Interpolate {
                from_value,
                mut samples_done,
            } => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.request_stop();
                    } else {
                        self.stage = LfoStage::Interpolate {
                            from_value: self.last_value,
                            samples_done: 0,
                        };
                    }
                } else {
                    samples_done += 1;

                    if samples_done == self.samples_to_interpolate {
                        self.stage = LfoStage::Running;
                    } else {
                        self.stage = LfoStage::Interpolate {
                            from_value,
                            samples_done,
                        }
                    }
                }
            }
            LfoStage::Running => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.request_stop();
                    } else {
                        match (self.current_shape, shape) {
                            (Some(LfoShape::Sine), LfoShape::Sine)
                            | (Some(LfoShape::ReverseSine), LfoShape::ReverseSine) => {}
                            _ => {
                                self.current_shape = Some(shape);

                                self.stage = LfoStage::Interpolate {
                                    from_value: self.last_value,
                                    samples_done: 0,
                                }
                            }
                        }
                    }
                }
            }
            LfoStage::Stopping {
                from_value,
                mut samples_done,
            } => {
                samples_done += 1;

                if samples_done == self.samples_to_interpolate {
                    self.stage = LfoStage::Stopped;
                    self.last_value = 0.0;
                } else {
                    self.stage = LfoStage::Stopping {
                        from_value,
                        samples_done,
                    }
                }
            }
            LfoStage::Stopped => {
                unreachable!()
            }
        }
    }

    pub fn get_value(&mut self, amount: f64) -> f64 {
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
            } => {
                let progress = samples_done as f64 / self.samples_to_interpolate as f64;

                progress * shape.calculate(self.phase) + (1.0 - progress) * from_value
            }
            LfoStage::Running => shape.calculate(self.phase),
            LfoStage::Stopping {
                from_value,
                samples_done,
            } => {
                let progress = samples_done as f64 / self.samples_to_interpolate as f64;

                from_value - from_value * progress
            }
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

        self.stage = if let LfoStage::Stopped = self.stage {
            LfoStage::Interpolate {
                from_value: 0.0,
                samples_done: 0,
            }
        } else {
            LfoStage::Interpolate {
                from_value: self.last_value,
                samples_done: 0,
            }
        };
    }

    pub fn request_stop(&mut self) {
        self.stage = LfoStage::Stopping {
            from_value: self.last_value,
            samples_done: 0,
        };
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.stage, LfoStage::Stopped)
    }
}
