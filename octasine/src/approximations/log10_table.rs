const TABLE_SIZE: usize = 1 << 5;
const TABLE_SIZE_MINUS_ONE_FLOAT: f64 = (TABLE_SIZE - 1) as f64;


/// Log10 based lookup table for envelope curve, with linear interpolation
/// 
/// Maps inputs 0.0-1.0 to output 0.0-1.0
pub struct Log10Table {
    table: [f64; TABLE_SIZE],
}


impl Log10Table {
    #[inline]
    pub fn reference(value: f64) -> f64 {
        (1.0 + value * 9.0).log10()
    }

    /// Get volume. Only defined where value >= 0.0 && value <= 1.0
    #[inline]
    pub fn calculate(&self, value: f64) -> f64 {
        let index_float = value * TABLE_SIZE_MINUS_ONE_FLOAT;
        let index_fract = index_float.fract();

        let index_floor = index_float as usize;
        let index_ceil = index_floor + 1;

        let approximation_low = self.table[index_floor];
        let approximation_high = self.table[index_ceil.min(TABLE_SIZE - 1)];

        approximation_low + index_fract *
            (approximation_high - approximation_low)
    }
}

impl Default for Log10Table {
    fn default() -> Self {
        let mut table = [0.0; TABLE_SIZE];

        let increment = 1.0 / TABLE_SIZE_MINUS_ONE_FLOAT;

        for (i, v) in table.iter_mut().enumerate(){
            *v = Self::reference(i as f64 * increment);
        }

        Self {
            table
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;

    #[test]
    fn test_table_calculate(){
        fn prop(value: f64) -> TestResult {
            if value > 1.0 || value < 0.0 {
                return TestResult::discard();
            }

            let table = Log10Table::default();

            let table_result = table.calculate(value);
            let reference_result = Log10Table::reference(value);
            let diff = (table_result - reference_result).abs();

            let success = diff < 0.005;

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("table result:     {}", table_result);
                println!("reference result: {}", reference_result);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f64) -> TestResult);
    }
}