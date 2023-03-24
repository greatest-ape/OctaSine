/// Triangle wave
#[inline]
pub fn triangle(x: f64) -> f64 {
    let x = x + 0.25;

    (2.0 * (2.0 * (x - (x + 0.5).floor())).abs()) - 1.0
}

/// Square wave with smooth transitions
///
/// Check absence of branches by removing #[inline] statement and running:
/// cargo asm --lib --no-default-features --full-name --rust -p octasine "octasine::math::wave::square"
#[inline]
pub fn square(x: f64) -> f64 {
    // If x is negative, final result should be negated
    let negate_if_x_negative: f64 = if x.is_sign_negative() { -1.0 } else { 1.0 };

    // x is now between 0.0 and 1.0
    let mut x = x.abs().fract();

    // If x > 0.5, final result should be negated
    let negate_if_x_gt_half: f64 = if x > 0.5 { -1.0 } else { 1.0 };

    let sign_mask = negate_if_x_negative.to_bits() ^ negate_if_x_gt_half.to_bits();

    // Adjust for x > 0.5
    if x > 0.5 {
        x = 1.0 - x;
    }

    // More iterations cause "tighter interpolation"
    //
    // Do repeated multiplications instead of using powf to be consistent with
    // SIMD implementations.
    let a = x * 4.0 - 1.0;
    let a2 = a * a;
    let a4 = a2 * a2;
    let a8 = a4 * a4;
    let a16 = a8 * a8;
    let a32 = a16 * a16;
    let a64 = a32 * a32;
    let a128 = a64 * a64;

    let approximation = 2.0 * ((1.0 / (1.0 + a128)) - 0.5);

    f64::from_bits(approximation.to_bits() ^ sign_mask)
}

/// Saw wave with smooth transitions
///
/// Check absence of branches by removing #[inline] statement and running:
/// cargo asm --lib --no-default-features --full-name --rust -p octasine "octasine::math::wave::saw"
#[inline]
pub fn saw(x: f64) -> f64 {
    const DOWN_FACTOR: f64 = 50.0;
    const X_INTERSECTION: f64 = 1.0 - (1.0 / DOWN_FACTOR);
    const UP_FACTOR: f64 = 1.0 / X_INTERSECTION;

    let x_is_negative = x < 0.0;

    let mut x = x.abs().fract();

    if x_is_negative {
        x = 1.0 - x;
    }

    let up = x * UP_FACTOR;
    let down = DOWN_FACTOR - DOWN_FACTOR * x;

    let y = if x < X_INTERSECTION { up } else { down };

    (y - 0.5) * 2.0
}
