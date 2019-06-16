use std::f32::consts::PI;

use crate::constants::TAU;
use crate::common::Phase;

pub const TABLE_SIZE: usize = 1 << 10;
pub const TABLE_SIZE_DIVIDED_BY_PI: f32 = TABLE_SIZE as f32 / PI;
pub const TABLE_SIZE_TIMES_2_FLOAT: f32 = (TABLE_SIZE * 2) as f32;


pub struct SineLookupTable {
    table: [f32; TABLE_SIZE]
}

impl SineLookupTable {
    pub fn new() -> Self {
        let mut table = [0.0; TABLE_SIZE];

        let increment = PI / TABLE_SIZE as f32;

        for (i, v) in table.iter_mut().enumerate(){
            *v = (i as f32 * increment).sin();
        }

        Self {
            table
        }
    }

    /// Calculate approximate sine of `value` with lookup table and
    /// linear interpolation
    /// 
    /// Defined for all valid f32 numbers. Does not contain branches.
    /// Currently a lot less accurate on large input values.
    pub fn sin(&self, value: f32) -> f32 {
        let value_is_negative = value.is_sign_negative();

        let value_rem_tau = value.abs() % TAU;
        let value_rem_tau_gte_pi = value_rem_tau >= PI;

        // Calulate value_rem_tau % PI cheaply and without branching
        let subtraction_array = [0.0, PI];
        let value_rem_pi = value_rem_tau - subtraction_array[
            value_rem_tau_gte_pi as usize
        ];

        let index_float = value_rem_pi * TABLE_SIZE_DIVIDED_BY_PI;
        let index_fract = index_float.fract();

        let index_floor = index_float as usize;
        let index_ceil = index_floor + 1;

        let approximation_low = self.table[index_floor];
        let approximation_high = self.table[index_ceil % TABLE_SIZE];

        let mut approximation = (approximation_high - approximation_low)
            .mul_add(index_fract, approximation_low)
            .to_bits();

        // Change sign if necessary
        approximation ^= (value_is_negative as u32) << 31;
        approximation ^= (value_rem_tau_gte_pi as u32) << 31;

        f32::from_bits(approximation)
    }

    /// Calculate approximate sine for `value * TAU` with lookup table and
    /// linear interpolation
    /// 
    /// If phase is kept in this format anyway, this could improve performance
    /// by replacing the remainder calculation from `self.sin` with .fract().
    pub fn sin_tau(&self, phase: Phase) -> f32 {
        let phase_is_negative = phase.0.is_sign_negative();
        let value = phase.0.abs().fract();

        let value_gte_half = value >= 0.5;

        let subtraction_array = [0.0, 0.5];
        let value = value - subtraction_array[
            value_gte_half as usize
        ];

        let index_float = value * TABLE_SIZE_TIMES_2_FLOAT;
        let index_fract = index_float.fract();

        let index_floor = index_float as usize;
        let index_ceil = index_floor + 1;

        let approximation_low = self.table[index_floor];
        let approximation_high = self.table[index_ceil % TABLE_SIZE];

        let mut approximation = (approximation_high - approximation_low)
            .mul_add(index_fract, approximation_low)
            .to_bits();

        // Change sign if necessary
        approximation ^= (phase_is_negative as u32) << 31;
        approximation ^= (value_gte_half as u32) << 31;

        f32::from_bits(approximation)
    }

}


#[cfg(feature = "simd")]
impl SineLookupTable {
    #[inline]
    pub fn sin_simd(
        &self,
        inputs: ::packed_simd::f32x2
    ) -> ::packed_simd::f32x2 {
        use packed_simd::*;

        let is_negative = inputs.lt(f32x2::splat(0.0));

        let mut inputs = inputs;

        inputs = inputs.abs() % TAU;
        let is_gte_pi = inputs.ge(f32x2::splat(PI));

        inputs = is_gte_pi.select(inputs - PI, inputs);

        let indeces_float = inputs * TABLE_SIZE_DIVIDED_BY_PI;
        let indeces_floor = i32x2::from_cast(indeces_float);
        let indeces_fract = indeces_float - f32x2::from_cast(indeces_floor);
        let indeces_ceil = indeces_floor + 1;

        let low = f32x2::new(
            self.table[indeces_floor.extract(0) as usize],
            self.table[indeces_floor.extract(1) as usize]
        );
        let high = f32x2::new(
            self.table[indeces_ceil.extract(0) as usize % TABLE_SIZE],
            self.table[indeces_ceil.extract(1) as usize % TABLE_SIZE]
        );

        let mut outputs = (high - low) * indeces_fract + low;

        outputs = is_negative.select(outputs * -1.0, outputs);
        outputs = is_gte_pi.select(outputs * -1.0, outputs);

        outputs
    }
}


#[cfg(feature = "simd")]
macro_rules! impl_sin_tau_simd {
    ($name:ident, $float_type:ty, $int_type:ty, $lanes:expr) => {
        impl SineLookupTable {
            #[inline]
            #[cfg(feature = "simd")]
            pub fn $name(
                &self,
                inputs: $float_type
            ) -> $float_type {
                use packed_simd::*;

                let is_negative = inputs.lt(<$float_type>::splat(0.0));

                let mut inputs = inputs;

                inputs = inputs.abs();
                // Fract
                inputs -= <$float_type>::from_cast(<$int_type>::from_cast(inputs));

                let is_gte_half = inputs.ge(<$float_type>::splat(0.5));

                inputs = is_gte_half.select(inputs - 0.5, inputs);

                let indeces_float = inputs * TABLE_SIZE_TIMES_2_FLOAT;
                let indeces_floor = <$int_type>::from_cast(indeces_float);
                let indeces_fract = indeces_float - <$float_type>::from_cast(indeces_floor);
                let indeces_ceil = indeces_floor + 1;

                let low_indeces: [i32; $lanes] = indeces_floor.into();
                let high_indeces: [i32; $lanes] = indeces_ceil.into();

                let mut low_approximations = [0.0f32; $lanes];
                let mut high_approximations = [0.0f32; $lanes];

                for i in 0..$lanes {
                    low_approximations[i] = self.table[low_indeces[i] as usize];
                    high_approximations[i] = self.table[high_indeces[i] as usize % TABLE_SIZE];
                }

                let low = <$float_type>::from(low_approximations);
                let high = <$float_type>::from(high_approximations);

                let mut outputs = (high - low) * indeces_fract + low;

                let flip_sign = is_negative ^ is_gte_half;
                outputs = flip_sign.select(outputs * -1.0, outputs);

                outputs
            }
        }
    };
}


#[cfg(feature = "simd")]
impl_sin_tau_simd!(sin_tau_simd_x2, ::packed_simd::f32x2, ::packed_simd::i32x2, 2);


#[cfg(feature = "simd")]
impl_sin_tau_simd!(sin_tau_simd_x4, ::packed_simd::f32x4, ::packed_simd::i32x4, 4);


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use quickcheck::{TestResult, quickcheck};
    use rand::{Rng, FromEntropy};
    use rand::rngs::SmallRng;
    use statistical;

    use super::*;

    #[test]
    fn test_table_sin(){
        const REFERENCE_ERROR_MARGIN: f32 = 0.000003;

        let table = SineLookupTable::new();

        assert_eq!(table.sin(0.0), 0.0);
        assert_eq!(table.sin(PI / 2.0), 1.0);
        assert_eq!(table.sin(PI), 0.0);
        assert_eq!(table.sin(TAU), 0.0);

        table.sin(::std::f32::MAX);
        table.sin(-::std::f32::MAX);

        assert_approx_eq!(
            table.sin(PI - ::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );

        assert_approx_eq!(
            table.sin(PI + ::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );

        assert_approx_eq!(
            table.sin(TAU - ::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );

        assert_approx_eq!(
            table.sin(TAU + ::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );

        assert_approx_eq!(
            table.sin(-::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );

        assert_approx_eq!(
            table.sin(::std::f32::EPSILON),
            0.0,
            REFERENCE_ERROR_MARGIN
        );
    }

    #[test]
    fn test_table_sin_quickcheck(){
        fn prop(value: f32) -> TestResult {
            let table = SineLookupTable::new();

            let table_sin = table.sin(value);
            let reference_sin = value.sin();
            let diff = (table_sin - reference_sin).abs();

            let success = diff < 0.000003; // Not of large importance

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("table sin():      {}", table_sin);
                println!("reference sin():  {}", reference_sin);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[test]
    fn test_table_sin_phase_quickcheck(){
        fn prop(value: f32) -> TestResult {
            if value.abs() > 400.0 {
                return TestResult::discard();
            }

            let table = SineLookupTable::new();

            let real_value = value * TAU;

            let table_sin = table.sin_tau(Phase(value));
            let reference_sin = real_value.sin();
            let diff = (table_sin - reference_sin).abs();

            let success = diff < 0.0001; // Not of large importance

            if !success {
                println!();
                println!("input value:          {}", value);
                println!("real value:           {}", real_value);
                println!("table sin_phase():    {}", table_sin);
                println!("reference sin():      {}", reference_sin);
                println!("difference:           {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_table_sin_simd_quickcheck(){
        use packed_simd::*;

        fn prop(value: f32) -> TestResult {
            let table = SineLookupTable::new();

            let v = f32x2::new(value, 0.0);

            let table_sin = table.sin_simd(v).extract(0);
            let reference_sin = value.sin();
            let diff = (table_sin - reference_sin).abs();

            let success = diff < 0.000003; // Not of large importance

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("table sin():      {}", table_sin);
                println!("reference sin():  {}", reference_sin);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_table_sin_tau_simd_quickcheck(){
        use packed_simd::*;

        fn prop(value: f32) -> TestResult {
            if value.abs() > 400.0 {
                return TestResult::discard();
            }

            let x2 = f32x2::splat(value);
            let x4 = f32x4::splat(value);

            let table = SineLookupTable::new();

            let real_value = value * TAU;

            let x2_sin = table.sin_tau_simd_x2(x2).extract(0);
            let x4_sin = table.sin_tau_simd_x4(x4).extract(0);

            assert_eq!(x2_sin, x4_sin);

            let reference_sin = real_value.sin();
            let diff = (x2_sin - reference_sin).abs();

            let success = diff < 0.00005;

            if !success {
                println!();
                println!("input value:      {}", value);
                println!("real value:       {}", real_value);
                println!("table sine:       {}", x2_sin);
                println!("reference sine:   {}", reference_sin);
                println!("difference:       {}", diff);
            }

            TestResult::from_bool(success)
        }

        quickcheck(prop as fn(f32) -> TestResult);
    }

    /// Test accuracy of SineLookupTable
    /// 
    /// The reference values are just based on results and serve as a kind
    /// of documentation.
    /// 
    /// Shows strongly decreasing accuracy on increasing values, especially
    /// worst-case accuracy, where tau sin function is worse. It seems to
    /// have a bit better median accuracy than normal sin function when a
    /// large value range is used though.
    /// 
    /// Since large input values are caused by modulation as long as the phase
    /// is kept small, decreasing accuracy on large values shouldn't matter.
    #[test]
    fn test_sin_table_accuracy(){
        let table = SineLookupTable::new();
        let mut rng = SmallRng::from_entropy();
        let mut sin_differences = Vec::new();
        let mut sin_tau_differences = Vec::new();
        let mut gen_values = Vec::new();

        let multiple = 200.0;

        for _ in 0..200000 {
            let value = (rng.gen::<f32>() - 0.5) * multiple * 2.0;
            let value_x_tau = value * TAU;
            gen_values.push(value);

            let reference_sin = value_x_tau.sin();

            let table_sin = table.sin(value_x_tau);
            let sin_diff = (table_sin - reference_sin).abs();
            sin_differences.push(sin_diff);

            let table_sin_tau = table.sin_tau(Phase(value));
            let sin_tau_diff = (table_sin_tau - reference_sin).abs();
            sin_tau_differences.push(sin_tau_diff);
        }

        fn f(differences: &[f32], values: &[f32]) -> (f32, f32, f32, f32, f32){
            let mean = statistical::mean(&differences);
            let median = statistical::median(&differences);

            let mut max = 0.0;
            let mut min = ::std::f32::MAX;

            let mut max_value = 0.0;

            for (diff, value) in differences.iter().zip(values) {
                let diff = *diff;

                if diff > max {
                    max = diff;
                    max_value = *value;
                }
                if diff < min {
                    min = diff;
                }
            }

            (min, max, mean, median, max_value)
        }

        let (a_min, a_max, a_mean, a_median, a_max_value) = f(&sin_differences, &gen_values);
        let (b_min, b_max, b_mean, b_median, b_max_value) = f(&sin_tau_differences, &gen_values);

        let a_success = a_mean < 0.00002 && a_median < 0.00001 && a_max < 0.00004;
        let b_success = b_mean < 0.00002 && b_median < 0.00001 && b_max < 0.0001;

        let success = a_success && b_success;

        if !success {
            println!();
            println!("-- normal table sin differences from reference sin --");
            println!("min:      {}", a_min);
            println!("max:      {} (value: {})", a_max, a_max_value);
            println!("mean:     {}", a_mean);
            println!("median:   {}", a_median);
            println!();
            println!("-- tau table sin differences from reference sin --");
            println!("min:      {}", b_min);
            println!("max:      {} (value: {})", b_max, b_max_value);
            println!("mean:     {}", b_mean);
            println!("median:   {}", b_median);
        }

        assert!(success);
    }
}