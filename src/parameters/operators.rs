use smallvec::SmallVec;

use crate::common::*;
use crate::constants::*;

use super::common::*;
use super::utils::*;



#[derive(Debug, Copy, Clone)]
pub struct OperatorStepData {
    pub step_size: f64,
    pub steps_remaining: usize,
    pub last_time: TimeCounter,
    pub num_steps: usize
}

impl Default for OperatorStepData {
    fn default() -> Self {
        Self {
            step_size: 0.0,
            steps_remaining: 0,
            last_time: TimeCounter(0.0),
            num_steps: 32,
        }
    }
}


#[macro_export]
macro_rules! create_interpolatable_automatable {
    ($struct_name:ident, $default_value:ident, $parameter_name:expr) => {

        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            current_value: f64,
            pub target_value: f64,
            step_data: OperatorStepData,
            operator_index: usize,
        }

        impl $struct_name {
            fn new(operator_index: usize) -> Self {
                Self {
                    current_value: $default_value,
                    target_value: $default_value,
                    step_data: OperatorStepData::default(),
                    operator_index: operator_index,
                }
            }
        }

        impl InterpolatableValue for $struct_name {
            fn get_value(&mut self, time: TimeCounter) -> f64 {
                if self.step_data.num_steps == 0 {
                    return self.current_value;
                }

                if time != self.step_data.last_time && self.step_data.steps_remaining > 0 {
                    self.current_value += self.step_data.step_size;
                    self.step_data.steps_remaining -= 1;
                    self.step_data.last_time = time;
                }

                self.current_value
            }

            fn set_value(&mut self, value: f64){
                self.target_value = value;

                if self.step_data.num_steps == 0 {
                    self.current_value = value;

                    return;
                }

                if value == self.current_value {
                    self.step_data.steps_remaining = 0;
                }
                else {
                    // Restart stepping process
                    let diff = value - self.current_value;
                    self.step_data.step_size = diff / self.step_data.num_steps as f64;
                    self.step_data.steps_remaining = self.step_data.num_steps;
                }
            }
        }

        impl Parameter for $struct_name {
            fn get_parameter_name(&self) -> String {
                format!("Op. {} {}", self.operator_index + 1, $parameter_name)
            }

            fn set_parameter_value_float(&mut self, value: f64){
                self.set_value(self.from_parameter_value(value));
            }
            fn set_parameter_value_text(&mut self, value: String) -> bool {
                if let Some(value) = self.parse_string_value(value){
                    self.set_value(value);

                    true
                } else {
                    false
                }
            }
            fn get_parameter_value_float(&self) -> f64 {
                self.to_parameter_value(self.target_value)
            }
            fn get_parameter_value_text(&self) -> String {
                format!("{:.2}", self.target_value)
            }
        }
    };  
}


#[macro_export]
macro_rules! create_automatable {
    ($struct_name:ident, $default_value:ident, $parameter_name:expr) => {

        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name {
            pub value: f64,
            operator_index: usize,
        }

        impl $struct_name {
            fn new(operator_index: usize) -> Self {
                $struct_name {
                    value: $default_value,
                    operator_index: operator_index,
                }
            }
        }

        impl Parameter for $struct_name {
            fn get_parameter_name(&self) -> String {
                format!("Op. {} {}", self.operator_index + 1, $parameter_name)
            }

            fn set_parameter_value_float(&mut self, value: f64){
                self.value = self.from_parameter_value(value);
            }
            fn set_parameter_value_text(&mut self, value: String) -> bool {
                if let Some(value) = self.parse_string_value(value){
                    self.value = value;

                    true
                } else {
                    false
                }
            }
            fn get_parameter_value_float(&self) -> f64 {
                self.to_parameter_value(self.value)
            }
            fn get_parameter_value_text(&self) -> String {
                format!("{:.2}", self.value)
            }
        }
    };  
}


create_interpolatable_automatable!(
    OperatorVolume,
    OPERATOR_DEFAULT_VOLUME,
    "volume"
);

impl OperatorVolume {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value * 2.0
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value / 2.0
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


#[derive(Debug, Clone)]
pub struct OperatorOutputOperator {
    targets: SmallVec<[usize; NUM_OPERATORS]>,
    pub target: usize,
    pub operator_index: usize,
}

impl OperatorOutputOperator {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        if operator_index <= 1 {
            None
        }
        else {
            Some(Self {
                targets: (0..operator_index).into_iter().collect(),
                target: operator_index - 1,
                operator_index: operator_index,
            })
        }
    }

    pub fn from_parameter_value(&self, value: f64) -> usize {
        let step = 1.0 / self.targets.len() as f64;
        let mut sum = 0.0;

        for t in self.targets.iter() {
            sum += step;

            if value <= sum {
                return *t
            }
        }

        *self.targets.last().expect("No targets")
    }
    pub fn to_parameter_value(&self, value: usize) -> f64 {
        let step = 1.0 / self.targets.len() as f64;

        value as f64 * step + 0.0001
    }
    pub fn parse_string_value(&self, value: String) -> Option<usize> {
        if let Ok(value) = value.parse::<usize>(){
            if value != 0 {
                let target = value - 1;

                if self.targets.contains(&target){
                    return Some(target);
                }
            }
        }

        None
    }
}

impl Parameter for OperatorOutputOperator {
    fn get_parameter_name(&self) -> String {
        format!("Op. {} {}", self.operator_index + 1, "mod out")
    }

    fn set_parameter_value_float(&mut self, value: f64){
        self.target = self.from_parameter_value(value);
    }
    fn set_parameter_value_text(&mut self, value: String) -> bool {
        if let Some(value) = self.parse_string_value(value){
            self.target = value;

            true
        } else {
            false
        }
    }
    fn get_parameter_value_float(&self) -> f64 {
        self.to_parameter_value(self.target)
    }
    fn get_parameter_value_text(&self) -> String {
        format!("Operator {}", self.target + 1)
    }
}


create_interpolatable_automatable!(
    OperatorAdditiveFactor,
    OPERATOR_DEFAULT_ADDITIVE_FACTOR,
    "additive"
);

impl OperatorAdditiveFactor {
    pub fn opt_new(operator_index: usize) -> Option<Self> {
        if operator_index == 0 {
            None
        } else {
            Some(Self::new(operator_index))
        }
    }

    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| value.max(0.0).min(1.0))
    }
}


create_interpolatable_automatable!(
    OperatorPanning,
    OPERATOR_DEFAULT_PANNING,
    "pan"
);

impl OperatorPanning {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| value.max(0.0).min(1.0))
    }

    pub fn get_left_and_right(panning: f64) -> (f64, f64) {
        let pan_phase = panning * HALF_PI;

        (pan_phase.cos(), pan_phase.sin())
    }
}


create_automatable!(
    OperatorFrequencyRatio,
    OPERATOR_DEFAULT_FREQUENCY_RATIO,
    "freq ratio"
);

impl OperatorFrequencyRatio {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        map_parameter_value_to_step(&OPERATOR_RATIO_STEPS[..], value)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        map_step_to_parameter_value(&OPERATOR_RATIO_STEPS[..], value)
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value|
            round_to_step(&OPERATOR_RATIO_STEPS[..], value)
        )
    }
}


create_automatable!(
    OperatorFrequencyFree,
    OPERATOR_DEFAULT_FREQUENCY_FREE,
    "freq free"
);

impl OperatorFrequencyFree {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        map_parameter_value_to_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_FREE_STEPS, value)
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


create_automatable!(
    OperatorFrequencyFine,
    OPERATOR_DEFAULT_FREQUENCY_FINE,
    "freq fine"
);

impl OperatorFrequencyFine {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        (value + 0.5).powf(1.0/3.0)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value.powf(3.0) - 0.5
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


create_interpolatable_automatable!(
    OperatorFeedback,
    OPERATOR_DEFAULT_FEEDBACK,
    "feedback"
);

impl OperatorFeedback {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


create_interpolatable_automatable!(
    OperatorModulationIndex,
    OPERATOR_DEFAULT_MODULATION_INDEX,
    "mod index"
);

impl OperatorModulationIndex {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        map_parameter_value_to_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        map_value_to_parameter_value_with_steps(&OPERATOR_BETA_STEPS[..], value)
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| {
            let max = self.from_parameter_value(1.0);
            let min = self.from_parameter_value(0.0);

            value.max(min).min(max)
        })
    }
}


#[derive(Debug, Copy, Clone)]
pub struct OperatorWaveType {
    pub value: WaveType,
    pub operator_index: usize,
}

impl OperatorWaveType {
    fn new(operator_index: usize) -> Self {
        Self {
            value: WaveType::Sine,
            operator_index
        }
    }
    pub fn from_parameter_value(&self, value: f64) -> WaveType {
        if value <= 0.5 {
            WaveType::Sine
        }
        else {
            WaveType::WhiteNoise
        }
    }
    pub fn to_parameter_value(&self, value: WaveType) -> f64 {
        match value {
            WaveType::Sine => 0.0,
            WaveType::WhiteNoise => 1.0,
        }
    }
    pub fn parse_string_value(&self, value: String) -> Option<WaveType> {
        let value = value.to_lowercase();

        if value == "sine" {
            return Some(WaveType::Sine);
        } else if value == "noise" || value == "white noise" {
            return Some(WaveType::WhiteNoise);
        }

        if let Ok(value) = value.parse::<f64>() {
            return Some(self.from_parameter_value(value));
        }

        None
    }
}

impl Parameter for OperatorWaveType {
    fn get_parameter_name(&self) -> String {
        format!("Op. {} {}", self.operator_index + 1, "wave type")
    }

    fn set_parameter_value_float(&mut self, value: f64){
        self.value = self.from_parameter_value(value);
    }
    fn set_parameter_value_text(&mut self, value: String) -> bool {
        if let Some(value) = self.parse_string_value(value){
            self.value = value;

            true
        }
        else {
            false
        }
    }
    fn get_parameter_value_float(&self) -> f64 {
        self.to_parameter_value(self.value)
    }
    fn get_parameter_value_text(&self) -> String {
        match self.value {
            WaveType::Sine => "Sine".to_string(),
            WaveType::WhiteNoise => "White noise".to_string(),
        }
    }
}


create_automatable!(
    VolumeEnvelopeAttackDuration,
    OPERATOR_DEFAULT_VOLUME_ENVELOPE_ATTACK_DURATION,
    "attack time"
);

impl VolumeEnvelopeAttackDuration {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        // Force some attack to avoid clicks
        (value * OPERATOR_ENVELOPE_MAX_DURATION)
            .max(OPERATOR_ENVELOPE_MIN_DURATION)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value / OPERATOR_ENVELOPE_MAX_DURATION
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value|
            value.max(OPERATOR_ENVELOPE_MIN_DURATION)
                .min(OPERATOR_ENVELOPE_MAX_DURATION)
        )
    }
}


create_automatable!(
    VolumeEnvelopeAttackValue,
    OPERATOR_DEFAULT_VOLUME_ENVELOPE_ATTACK_VALUE,
    "attack vol"
);

impl VolumeEnvelopeAttackValue {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| value.max(0.0).min(1.0))
    }
}


create_automatable!(
    VolumeEnvelopeDecayDuration,
    OPERATOR_DEFAULT_VOLUME_ENVELOPE_DECAY_DURATION,
    "decay time"
);

impl VolumeEnvelopeDecayDuration {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        // Force some decay to avoid clicks
        (value * OPERATOR_ENVELOPE_MAX_DURATION)
            .max(OPERATOR_ENVELOPE_MIN_DURATION)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value / OPERATOR_ENVELOPE_MAX_DURATION
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value|
            value.max(OPERATOR_ENVELOPE_MIN_DURATION)
                .min(OPERATOR_ENVELOPE_MAX_DURATION)
        )
    }
}


create_automatable!(
    VolumeEnvelopeDecayValue,
    OPERATOR_DEFAULT_VOLUME_ENVELOPE_DECAY_VALUE,
    "decay vol"
);

impl VolumeEnvelopeDecayValue {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value| value.max(0.0).min(1.0))
    }
}


create_automatable!(
    VolumeEnvelopeReleaseDuration,
    OPERATOR_DEFAULT_VOLUME_ENVELOPE_RELEASE_DURATION,
    "release time"
);

impl VolumeEnvelopeReleaseDuration {
    pub fn from_parameter_value(&self, value: f64) -> f64 {
        // Force some release to avoid clicks
        (value * OPERATOR_ENVELOPE_MAX_DURATION)
            .max(OPERATOR_ENVELOPE_MIN_DURATION)
    }
    pub fn to_parameter_value(&self, value: f64) -> f64 {
        value / OPERATOR_ENVELOPE_MAX_DURATION
    }
    pub fn parse_string_value(&self, value: String) -> Option<f64> {
        value.parse::<f64>().ok().map(|value|
            value.max(OPERATOR_ENVELOPE_MIN_DURATION)
                .min(OPERATOR_ENVELOPE_MAX_DURATION)
        )
    }
}


#[derive(Debug, Copy, Clone)]
pub struct OperatorVolumeEnvelope {
    pub attack_duration: VolumeEnvelopeAttackDuration,
    pub attack_end_value: VolumeEnvelopeAttackValue,
    pub decay_duration: VolumeEnvelopeDecayDuration,
    pub decay_end_value: VolumeEnvelopeDecayValue,
    pub release_duration: VolumeEnvelopeReleaseDuration,
}

impl OperatorVolumeEnvelope {
    fn new(operator_index: usize) -> Self {
        Self {
            attack_duration: VolumeEnvelopeAttackDuration::new(operator_index),
            attack_end_value: VolumeEnvelopeAttackValue::new(operator_index),
            decay_duration: VolumeEnvelopeDecayDuration::new(operator_index),
            decay_end_value: VolumeEnvelopeDecayValue::new(operator_index),
            release_duration: VolumeEnvelopeReleaseDuration::new(operator_index),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Operator {
    pub volume: OperatorVolume,
    pub wave_type: OperatorWaveType,
    pub panning: OperatorPanning,
    pub additive_factor: Option<OperatorAdditiveFactor>,
    pub output_operator: Option<OperatorOutputOperator>,
    pub frequency_ratio: OperatorFrequencyRatio,
    pub frequency_free: OperatorFrequencyFree,
    pub frequency_fine: OperatorFrequencyFine,
    pub feedback: OperatorFeedback,
    pub modulation_index: OperatorModulationIndex,
    pub volume_envelope: OperatorVolumeEnvelope,
}

impl Operator {
    pub fn new(operator_index: usize) -> Self {
        Self {
            volume: OperatorVolume::new(operator_index),
            panning: OperatorPanning::new(operator_index),
            wave_type: OperatorWaveType::new(operator_index),
            additive_factor: OperatorAdditiveFactor::opt_new(operator_index),
            output_operator: OperatorOutputOperator::opt_new(operator_index),
            frequency_ratio: OperatorFrequencyRatio::new(operator_index),
            frequency_free: OperatorFrequencyFree::new(operator_index),
            frequency_fine: OperatorFrequencyFine::new(operator_index),
            feedback: OperatorFeedback::new(operator_index),
            modulation_index: OperatorModulationIndex::new(operator_index),
            volume_envelope: OperatorVolumeEnvelope::new(operator_index),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_volume_text(){
        let mut operator = Operator::new(3);

        assert!(operator.volume.set_parameter_value_text("-1.0".to_string()));
        assert_eq!(operator.volume.target_value, 0.0);

        assert!(operator.volume.set_parameter_value_text("0".to_string()));
        assert_eq!(operator.volume.target_value, 0.0);

        assert!(operator.volume.set_parameter_value_text("0.0".to_string()));
        assert_eq!(operator.volume.target_value, 0.0);

        assert!(operator.volume.set_parameter_value_text("1.0".to_string()));
        assert_eq!(operator.volume.target_value, 1.0);

        assert!(operator.volume.set_parameter_value_text("1.2".to_string()));
        assert_eq!(operator.volume.target_value, 1.2);

        assert!(operator.volume.set_parameter_value_text("2.0".to_string()));
        assert_eq!(operator.volume.target_value, 2.0);

        assert!(operator.volume.set_parameter_value_text("3.0".to_string()));
        assert_eq!(operator.volume.target_value, 2.0);
    }

    #[test]
    fn test_set_output_operator_text(){
        let operator = Operator::new(3);
        let mut o = operator.output_operator.unwrap();

        assert!(!o.set_parameter_value_text("abc".to_string()));
        assert!(!o.set_parameter_value_text("0".to_string()));
        assert!(!o.set_parameter_value_text("0.5".to_string()));
        assert!(!o.set_parameter_value_text("4".to_string()));

        assert!(o.set_parameter_value_text("1".to_string()));
        assert_eq!(o.target, 0);

        assert!(o.set_parameter_value_text("2".to_string()));
        assert_eq!(o.target, 1);

        assert!(o.set_parameter_value_text("3".to_string()));
        assert_eq!(o.target, 2);
    }

    #[test]
    fn test_set_frequency_ratio_text(){
        let mut operator = Operator::new(3);

        assert!(operator.frequency_ratio.set_parameter_value_text("0.0".to_string()));
        assert_eq!(operator.frequency_ratio.value, OPERATOR_RATIO_STEPS[0]);

        assert!(operator.frequency_ratio.set_parameter_value_text("10000000.0".to_string()));
        assert_eq!(operator.frequency_ratio.value, *OPERATOR_RATIO_STEPS.last().unwrap());

        assert!(operator.frequency_ratio.set_parameter_value_text("1.0".to_string()));
        assert_eq!(operator.frequency_ratio.value, 1.0);

        assert!(operator.frequency_ratio.set_parameter_value_text("0.99".to_string()));
        assert_eq!(operator.frequency_ratio.value, 1.0);

        assert!(operator.frequency_ratio.set_parameter_value_text("0.5".to_string()));
        assert_eq!(operator.frequency_ratio.value, 0.5);

        assert!(operator.frequency_ratio.set_parameter_value_text("0.51".to_string()));
        assert_eq!(operator.frequency_ratio.value, 0.5);
    }

    #[test]
    fn test_set_frequency_free_text(){
        let mut operator = Operator::new(3);

        assert!(operator.frequency_free.set_parameter_value_text("1.0".to_string()));
        assert_eq!(operator.frequency_free.value, 1.0);

        assert!(operator.frequency_free.set_parameter_value_text("1".to_string()));
        assert_eq!(operator.frequency_free.value, 1.0);

        assert!(operator.frequency_free.set_parameter_value_text("0.0".to_string()));
        assert!((operator.frequency_free.value - OPERATOR_FREE_STEPS[0]).abs() < 0.00001);

        assert!(operator.frequency_free.set_parameter_value_text("256.0".to_string()));
        assert!((operator.frequency_free.value - OPERATOR_FREE_STEPS.last().unwrap()).abs() < 0.00001);
    }

    #[test]
    fn test_set_wave_type_text(){
        let mut operator = Operator::new(3);

        assert!(operator.wave_type.set_parameter_value_text("sine".to_string()));
        assert_eq!(operator.wave_type.value, WaveType::Sine);

        assert!(operator.wave_type.set_parameter_value_text("noise".to_string()));
        assert_eq!(operator.wave_type.value, WaveType::WhiteNoise);
    }

    #[test]
    fn test_set_attack_duration_text(){
        let mut operator = Operator::new(3);

        assert!(operator.volume_envelope.attack_duration
            .set_parameter_value_text("0.0".to_string()));
        assert_eq!(operator.volume_envelope.attack_duration.value, OPERATOR_ENVELOPE_MIN_DURATION);

        assert!(operator.volume_envelope.attack_duration
            .set_parameter_value_text("1.0".to_string()));
        assert_eq!(operator.volume_envelope.attack_duration.value, 1.0);

        assert!(operator.volume_envelope.attack_duration
            .set_parameter_value_text("10".to_string()));
        assert_eq!(operator.volume_envelope.attack_duration.value,
            OPERATOR_ENVELOPE_MAX_DURATION);
    }
}