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
            shape: LfoShape::ReverseSaw,
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
        time_advancement: f64,
        bpm: BeatsPerMinute,
        shape: LfoShape,
        mode: LfoMode,
        frequency: f64,
        amount: f64,
    ) -> f64 {
        if !self.active {
            self.last_value = 0.0;

            return 0.0;
        }
        if self.first_cycle {
            self.shape = shape;
        }

        let bpm_ratio = bpm.0 / 120.0;

        let new_phase = self.phase.0 +
            frequency * bpm_ratio * time_advancement;

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

        let mut value = amount * match self.shape {
            LfoShape::Saw => triangle(self.phase, Phase(0.9)),
            LfoShape::ReverseSaw => triangle(self.phase, Phase(0.1)),
            LfoShape::Triangle => triangle(self.phase, Phase(0.5)),
            LfoShape::Square => square(self.phase),
            LfoShape::ReverseSquare => rev_square(self.phase),
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
