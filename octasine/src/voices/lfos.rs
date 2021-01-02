use crate::common::*;


#[derive(Debug, Clone)]
pub struct VoiceLfo {
    phase: Phase,
    shape: LfoShape,
    first_cycle: bool,
    active: bool,
    last_value: f64,
    interpolate: Option<f64>,
}


impl Default for VoiceLfo {
    fn default() -> Self {
        Self {
            phase: Phase(0.0),
            shape: LfoShape::LinearDown,
            active: false,
            first_cycle: true,
            last_value: 0.0,
            interpolate: None,
        }
    }
}


impl VoiceLfo {
    pub fn get_value(
        &mut self,
        time_per_sample: TimePerSample,
        bpm: BeatsPerMinute,
        shape: LfoShape,
        mode: LfoMode,
        frequency: f64,
        magnitude: f64,
    ) -> f64 {
        if !self.active {
            self.last_value = 0.0;

            return 0.0;
        }
        if self.first_cycle {
            self.shape = shape;
        }

        let bpm_ratio = bpm.0 / 120.0;

        let new_phase = (frequency * bpm_ratio).mul_add(
            time_per_sample.0,
            self.phase.0
        );

        if new_phase >= 1.0 {
            if mode == LfoMode::Once {
                self.stop();

                return 0.0;
            }
            if shape != self.shape {
                self.shape = shape;
            }
            self.first_cycle = false;
            self.interpolate = None;
        }

        self.phase.0 = new_phase.fract();

        let mut value = match self.shape {
            LfoShape::LinearUp => {
                let phase = self.phase.0;
                let phase_cutoff = 0.9;

                let multiplier = if phase <= phase_cutoff {
                    phase / phase_cutoff
                } else {
                    1.0 - (phase - phase_cutoff) / (1.0 - phase_cutoff)
                };

                multiplier * magnitude
            },
            LfoShape::LinearDown => {
                let phase = self.phase.0;
                let phase_cutoff = 0.1;

                let multiplier = if phase <= phase_cutoff {
                    phase / phase_cutoff
                } else {
                    1.0 - (phase - phase_cutoff) / (1.0 - phase_cutoff)
                };

                multiplier * magnitude
            },
            LfoShape::Triangle => {
                let phase = self.phase.0;
                let phase_cutoff = 0.5;

                let multiplier = if phase <= phase_cutoff {
                    phase / phase_cutoff
                } else {
                    1.0 - (phase - phase_cutoff) / (1.0 - phase_cutoff)
                };

                multiplier * magnitude
            },
        };

        if let Some(interpolate) = self.interpolate {
            value = interpolate * (1.0 - self.phase.0) + value * self.phase.0;
        }

        self.last_value = value;

        value
    }

    pub fn restart(&mut self){
        if self.active {
            self.interpolate = Some(self.last_value);
        } else {
            self.interpolate = None;
        }

        self.active = true;
        self.first_cycle = true;
        self.phase = Phase(0.0);
    }

    pub fn stop(&mut self){
        self.active = false;
    }
}
