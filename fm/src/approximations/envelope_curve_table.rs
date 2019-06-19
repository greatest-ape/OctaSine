const TABLE_SIZE: usize = 1 << 6;
const TABLE_SIZE_FLOAT: f32 = TABLE_SIZE as f32;


/// Log10 based lookup table for envelopes
pub struct EnvelopeCurveTable {
    table: [f32; TABLE_SIZE],
}


impl EnvelopeCurveTable {
    pub fn new() -> Self {
        let mut table = [0.0; TABLE_SIZE];

        let increment = 1.0 / (TABLE_SIZE - 1) as f32;

        for (i, v) in table.iter_mut().enumerate(){
            *v = (1.0 + (i as f32 * increment) * 9.0).log10();
        }

        Self {
            table
        }
    }

    /// Get volume. Only defined where value >= 0.0 && value <= 1.0
    pub fn calculate(&self, value: f32) -> f32 {
        let index_float = value * (TABLE_SIZE_FLOAT - 1.0);
        let index_fract = index_float.fract();

        let index_floor = index_float as usize;
        let index_ceil = index_floor + 1;

        let approximation_low = self.table[index_floor];
        let approximation_high = self.table[index_ceil.min(TABLE_SIZE - 1)];

        approximation_low + (approximation_high - approximation_low) * index_fract
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    #[test]
    fn test_log10_calculate(){
        fn prop(value: f32) -> TestResult {
            if value > 1.0 || value < 0.0 {
                return TestResult::discard();
            }

            let table = EnvelopeCurveTable::new();

            let real_value = 1.0 + value * 9.0;

            let table_log10 = table.calculate(value);
            let reference_log10 = real_value.log10();
            let diff = (table_log10 - reference_log10).abs();

            let success = diff < 0.001;

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("real value:       {}", real_value);
                println!("table sin():      {}", table_log10);
                println!("reference sin():  {}", reference_log10);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }
}