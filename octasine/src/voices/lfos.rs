use crate::common::*;

use super::VoiceDuration;


const INTERPOLATION_STEPS: usize = 256;
const INTERPOLATION_STEPS_FLOAT: f64 = INTERPOLATION_STEPS as f64;


fn calculate_cycle_length(
    bpm: BeatsPerMinute,
    speed: f64
) -> f64 {
    let bpm_ratio = 120.0 / bpm.0;
    let cycle_length = bpm_ratio / speed;

    cycle_length
}


fn calculate_curve(
    shape: LfoShape,
    time_progress: f64,
    magnitude: f64
) -> f64 {
    match shape {
        LfoShape::LinearUp => {
            magnitude * time_progress
        },
        LfoShape::LinearDown => {
            magnitude - (magnitude * time_progress)
        },
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
struct LfoChangableData {
    time_per_sample: TimePerSample,
    bpm: BeatsPerMinute,
    shape: LfoShape,
    mode: LfoMode,
    speed: f64,
    magnitude: f64,
}


#[derive(Debug, Clone, Copy)]
struct LfoPersistentData {
    last_value: f64,
    last_changable_data: Option<LfoChangableData>,
    duration_at_cycle_start: VoiceDuration,
    cycles_complete: bool,
}


impl Default for LfoPersistentData {
    fn default() -> Self {
        Self {
            last_value: 0.0,
            last_changable_data: None,
            duration_at_cycle_start: VoiceDuration(0.0),
            cycles_complete: false,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LfoInterpolating {
    steps_left: usize,
    step_size: f64
}


impl LfoInterpolating {
    fn new(
        voice_duration: VoiceDuration,
        persistent_data: LfoPersistentData,
        changable_data: LfoChangableData,
    ) -> Self {
        let target_value = if persistent_data.cycles_complete {
            0.0
        } else {
            let duration_since_cycle_start = voice_duration.0 -
                persistent_data.duration_at_cycle_start.0;
            let predicted_interpolation_time = duration_since_cycle_start +
                (INTERPOLATION_STEPS_FLOAT * changable_data.time_per_sample.0);

            let cycle_length = calculate_cycle_length(
                changable_data.bpm,
                changable_data.speed
            );
            // FIXME: risk of values higher than 1.0 at very high speeds
            let time_progress = predicted_interpolation_time / cycle_length;

            calculate_curve(changable_data.shape, time_progress, changable_data.magnitude)
        };

        Self {
            steps_left: INTERPOLATION_STEPS,
            step_size: (target_value - persistent_data.last_value) / INTERPOLATION_STEPS_FLOAT
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LfoStatus {
    Interpolating(LfoInterpolating),
    DrawingShape,
    /// Waiting for data from audio gen call necessary to setup interpolation
    WaitingToInterpolate,
    Off
}


impl LfoStatus {
    fn get_value_and_new_status(
        &self,
        voice_duration: VoiceDuration,
        persistent_data: LfoPersistentData,
        changable_data: LfoChangableData,
    ) -> (f64, Self) {
        match self {
            Self::Interpolating(data) => {
                let new_value = persistent_data.last_value + data.step_size;
                let new_steps_left = data.steps_left - 1;

                let new_status = if new_steps_left == 0 {
                    if persistent_data.cycles_complete {
                        LfoStatus::Off
                    } else {
                        LfoStatus::DrawingShape
                    }
                } else {
                    Self::Interpolating(LfoInterpolating {
                        step_size: data.step_size,
                        steps_left: new_steps_left,
                    })
                };

                (new_value, new_status)
            },
            Self::DrawingShape => {
                let cycle_length = calculate_cycle_length(changable_data.bpm, changable_data.speed);

                let duration_since_cycle_start = voice_duration.0 -
                    persistent_data.duration_at_cycle_start.0;
                let time_progress = duration_since_cycle_start / cycle_length;

                let new_value = calculate_curve(changable_data.shape, time_progress, changable_data.magnitude);

                (new_value, Self::DrawingShape)
            },
            Self::WaitingToInterpolate => {
                // TODO: skip interpolation if next step is very close to
                // current, for e.g. sine waves

                let status = LfoStatus::Interpolating(LfoInterpolating::new(
                    voice_duration,
                    persistent_data,
                    changable_data
                ));

                status.get_value_and_new_status(
                    voice_duration,
                    persistent_data,
                    changable_data
                )
            },
            Self::Off => {
                (0.0, Self::Off)
            }
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct VoiceLfo {
    pub status: LfoStatus,
    persistent_data: LfoPersistentData,
}


impl Default for VoiceLfo {
    fn default() -> Self {
        Self {
            status: LfoStatus::Off,
            persistent_data: LfoPersistentData::default(),
        }
    }
}


impl VoiceLfo {
    fn should_new_cycle_start(
        voice_duration: VoiceDuration,
        persistent_data: LfoPersistentData,
        changable_data: LfoChangableData,
    ) -> bool {
        let cycle_length = calculate_cycle_length(changable_data.bpm, changable_data.speed);

        match changable_data.mode {
            LfoMode::Forever if voice_duration.0 >= persistent_data.duration_at_cycle_start.0 + cycle_length => {
                true
            },
            _ => false
        }
    }

    fn should_cycles_end(
        voice_duration: VoiceDuration,
        persistent_data: LfoPersistentData,
        changable_data: LfoChangableData,
    ) -> bool {
        let cycles_complete = persistent_data.cycles_complete;
        let cycle_length = calculate_cycle_length(changable_data.bpm, changable_data.speed);

        match changable_data.mode {
            LfoMode::Half if !cycles_complete && voice_duration.0 >= cycle_length / 2.0 => {
                true
            },
            LfoMode::Once if !cycles_complete && voice_duration.0 >= cycle_length => {
                true
            },
            _ => false,
        }
    }

    pub fn get_value(
        &mut self,
        voice_duration: VoiceDuration,
        time_per_sample: TimePerSample,
        bpm: BeatsPerMinute,
        shape: LfoShape,
        mode: LfoMode,
        speed: f64,
        magnitude: f64,
    ) -> f64 {
        if self.status == LfoStatus::Off {
            return 0.0;
        }

        let new_changable_data = LfoChangableData {
            time_per_sample,
            bpm,
            shape,
            mode,
            speed,
            magnitude,
        };

        // changable_data is Some unless lfo just restarted
        if let Some(last_changable_data) = self.persistent_data.last_changable_data {
            if last_changable_data != new_changable_data {
                self.status = LfoStatus::WaitingToInterpolate;
                self.persistent_data.duration_at_cycle_start = voice_duration;
            }

            // What happens when changing status to WaitingToInterpolate while interpolating?
            if !matches!(self.status, LfoStatus::Interpolating(_)){
                // FIXME: how do these relate to changed data?
                if Self::should_cycles_end(voice_duration, self.persistent_data, new_changable_data){
                    self.status = LfoStatus::WaitingToInterpolate;
                    self.persistent_data.cycles_complete = true;
                } else if Self::should_new_cycle_start(voice_duration, self.persistent_data, new_changable_data){
                    self.status = LfoStatus::WaitingToInterpolate;
                    self.persistent_data.duration_at_cycle_start = voice_duration;
                }
            }
        }

        let (new_value, new_status) = self.status.get_value_and_new_status(
            voice_duration,
            self.persistent_data,
            new_changable_data
        );
        
        self.status = new_status;

        self.persistent_data = LfoPersistentData {
            last_value: new_value,
            last_changable_data: Some(new_changable_data),
            duration_at_cycle_start: self.persistent_data.duration_at_cycle_start,
            cycles_complete: self.persistent_data.cycles_complete,
        };

        new_value
    }

    #[inline]
    pub fn restart(&mut self){
        let mut new_persistent_data = LfoPersistentData::default();

        new_persistent_data.last_value = self.persistent_data.last_value;

        self.persistent_data = new_persistent_data;

        self.status = match self.status {
            LfoStatus::Off => {
                LfoStatus::DrawingShape
            },
            _ => {
                LfoStatus::WaitingToInterpolate
            }
        };
    }

    #[inline]
    pub fn stop(&mut self){
        self.status = LfoStatus::Off;
        self.persistent_data = LfoPersistentData::default();
    }
}