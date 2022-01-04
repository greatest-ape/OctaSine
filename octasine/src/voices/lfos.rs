use crate::common::*;

const INTERPOLATION_SAMPLES: usize = 128;

#[derive(Debug, Clone)]
enum LfoStage {
    Interpolate {
        from_value: f64,
        samples_done: usize,
        stop_afterwards: bool,
    },
    Running,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct VoiceLfo {
    stage: LfoStage,
    current_shape: Option<LfoShape>,
    phase: Phase,
    last_value: f64,
}

impl Default for VoiceLfo {
    fn default() -> Self {
        Self {
            stage: LfoStage::Stopped,
            current_shape: None,
            phase: Phase(0.0),
            last_value: 0.0,
        }
    }
}

impl VoiceLfo {
    pub fn advance_one_sample(
        &mut self,
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

        let new_phase = self.phase.0 + frequency * (bpm.0 / 120.0) * time_per_sample.0;

        self.phase.0 = new_phase.fract();

        match self.stage {
            LfoStage::Interpolate {
                from_value,
                stop_afterwards,
                mut samples_done,
            } => {
                samples_done += 1;

                if samples_done == INTERPOLATION_SAMPLES {
                    if stop_afterwards {
                        self.stage = LfoStage::Stopped;
                        self.last_value = 0.0;
                    } else {
                        self.stage = LfoStage::Running;
                    }
                } else {
                    self.stage = LfoStage::Interpolate {
                        from_value,
                        samples_done,
                        stop_afterwards,
                    }
                }
            }
            LfoStage::Running => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.request_stop();
                    } else {
                        self.current_shape = Some(shape);

                        self.stage = LfoStage::Interpolate {
                            from_value: self.last_value,
                            samples_done: 0,
                            stop_afterwards: false,
                        }
                    }
                } else {
                    match self.current_shape.unwrap() {
                        LfoShape::Square | LfoShape::ReverseSquare if self.phase.0 >= 0.5 => {
                            self.stage = LfoStage::Interpolate {
                                from_value: self.last_value,
                                samples_done: 0,
                                stop_afterwards: false,
                            }
                        }
                        _ => {}
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
                stop_afterwards,
                samples_done,
            } => {
                let progress = samples_done as f64 / INTERPOLATION_SAMPLES as f64;

                if stop_afterwards {
                    from_value - from_value * progress
                } else {
                    progress * Self::calculate_curve(shape, self.phase)
                        + (1.0 - progress) * from_value
                }
            }
            LfoStage::Running => Self::calculate_curve(shape, self.phase),
            LfoStage::Stopped => {
                unreachable!()
            }
        };

        self.last_value = value;

        value * amount
    }

    fn calculate_curve(shape: LfoShape, phase: Phase) -> f64 {
        match shape {
            LfoShape::Saw => saw(phase),
            LfoShape::ReverseSaw => reverse_saw(phase),
            LfoShape::Triangle => triangle(phase),
            LfoShape::Square => square(phase),
            LfoShape::ReverseSquare => rev_square(phase),
        }
    }

    pub fn restart(&mut self) {
        self.phase = Phase(0.0);
        self.current_shape = None;

        self.stage = match self.stage {
            LfoStage::Stopped => LfoStage::Running,
            _ => LfoStage::Interpolate {
                from_value: self.last_value,
                samples_done: 0,
                stop_afterwards: false,
            },
        };
    }

    pub fn request_stop(&mut self) {
        self.stage = LfoStage::Interpolate {
            from_value: self.last_value,
            samples_done: 0,
            stop_afterwards: true,
        };
        self.phase.0 = 0.0;
    }
}

fn triangle(phase: Phase) -> f64 {
    let peak = 0.5;

    if phase.0 <= peak {
        phase.0 / peak
    } else {
        1.0 - (phase.0 - peak) / (1.0 - peak)
    }
}

fn saw(phase: Phase) -> f64 {
    phase.0
}

fn reverse_saw(phase: Phase) -> f64 {
    1.0 - phase.0
}

fn square(phase: Phase) -> f64 {
    if phase.0 <= 0.5 {
        1.0
    } else {
        0.0
    }
}

fn rev_square(phase: Phase) -> f64 {
    if phase.0 <= 0.5 {
        0.0
    } else {
        1.0
    }
}
