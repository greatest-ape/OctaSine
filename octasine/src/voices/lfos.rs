use crate::common::*;

const INTERPOLATION_TIME: f64 = 0.01;

#[derive(Debug, Clone)]
enum LfoStage {
    Restart {
        value_before_restart: f64,
        current_shape: Option<LfoShape>,
    },
    Running {
        current_shape: Option<LfoShape>,
    },
    Stopping {
        value_before_stopping: f64,
    },
    Stopped
}

#[derive(Debug, Clone)]
pub struct VoiceLfo {
    stage: LfoStage,
    phase: Phase,
    last_value: f64,
}

impl Default for VoiceLfo {
    fn default() -> Self {
        Self {
            stage: LfoStage::Stopped,
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

        let new_phase = self.phase.0 + frequency * (bpm.0 / 120.0) * time_per_sample.0;

        match self.stage {
            LfoStage::Restart { current_shape, value_before_restart } => {
                let shape = current_shape.unwrap_or(shape);

                if new_phase > INTERPOLATION_TIME {
                    self.stage = LfoStage::Running {
                        current_shape: Some(shape)
                    };
                } else {
                    self.stage = LfoStage::Restart {
                        current_shape: Some(shape),
                        value_before_restart
                    }
                }

                self.phase.0 = new_phase;
            },
            LfoStage::Running { .. } => {
                if new_phase >= 1.0 {
                    if mode == LfoMode::Once {
                        self.request_stop();
                    } else {
                        self.stage = LfoStage::Running {
                            current_shape: Some(shape),
                        };

                        self.phase.0 = new_phase.fract();
                    }
                } else {
                    self.phase.0 = new_phase;
                }
            },
            LfoStage::Stopping { .. } => {
                if new_phase > INTERPOLATION_TIME {
                    self.stage = LfoStage::Stopped;
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

        let value = match self.stage {
            LfoStage::Restart { current_shape, value_before_restart } => {
                let shape = if let Some(shape) = current_shape {
                    shape
                } else {
                    return 0.0;
                };

                let value = Self::calculate_curve(shape, self.phase);

                let interpolation_advancement = self.phase.0 / INTERPOLATION_TIME;

                interpolation_advancement * value + (1.0 - interpolation_advancement) * value_before_restart
            },
            LfoStage::Running { current_shape } => {
                let shape = if let Some(shape) = current_shape {
                    shape
                } else {
                    return 0.0;
                };

                Self::calculate_curve(shape, self.phase)
            }
            LfoStage::Stopping { value_before_stopping } => {
                let interpolation_advancement = self.phase.0 / INTERPOLATION_TIME;

                value_before_stopping - value_before_stopping * interpolation_advancement
            }
            LfoStage::Stopped => {
                unreachable!()
            }
        };

        self.last_value = value;

        value * amount
    }

    fn calculate_curve(shape: LfoShape, phase: Phase) -> f64 {
        match shape {
            LfoShape::Saw => triangle(phase, Phase(0.9)),
            LfoShape::ReverseSaw => triangle(phase, Phase(0.1)),
            LfoShape::Triangle => triangle(phase, Phase(0.5)),
            LfoShape::Square => square(phase),
            LfoShape::ReverseSquare => rev_square(phase),
        }
    }

    pub fn restart(&mut self) {
        self.phase = Phase(0.0);

        match self.stage {
            LfoStage::Restart { .. } | LfoStage::Running { .. } | LfoStage::Stopping { .. } => {
                self.stage = LfoStage::Restart {
                    current_shape: None,
                    value_before_restart: self.last_value,
                }
            }
            LfoStage::Stopped => {
                self.stage = LfoStage::Running { current_shape: None };
            }
        }
    }

    pub fn request_stop(&mut self) {
        self.stage = LfoStage::Stopping {
            value_before_stopping: self.last_value,
        };
        self.phase.0 = 0.0;
    }
}

fn triangle(phase: Phase, peak: Phase) -> f64 {
    if phase.0 <= peak.0 {
        phase.0 / peak.0
    } else {
        1.0 - (phase.0 - peak.0) / (1.0 - peak.0)
    }
}

fn square(phase: Phase) -> f64 {
    let peak_start = 0.1;
    let peak_end = 0.5;
    let base_start = 0.6;

    if phase.0 <= peak_start {
        phase.0 / peak_start
    } else if phase.0 <= peak_end {
        1.0
    } else if phase.0 <= base_start {
        1.0 - (phase.0 - peak_end) / (base_start - peak_end)
    } else {
        0.0
    }
}

fn rev_square(phase: Phase) -> f64 {
    let base_end = 0.4;
    let peak_start = 0.5;
    let peak_end = 0.9;

    if phase.0 <= base_end {
        0.0
    } else if phase.0 <= peak_start {
        (phase.0 - base_end) / (peak_start - base_end)
    } else if phase.0 <= peak_end {
        1.0
    } else {
        1.0 - (phase.0 - peak_end) / (1.0 - peak_end)
    }
}
