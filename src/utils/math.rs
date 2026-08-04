use fixed::types::I16F16;
use fixed_macro::types::I16F16;

/// Based on https://bmtechjournal.wordpress.com/2020/05/27/super-fast-quadratic-sinusoid-approximation/
// TODO: benchmark and look into improving
pub fn fast_sin(x: I16F16) -> I16F16 {
    let fake_sin_2 = |x: I16F16| 2 * x * (I16F16!(1) - (2 * x).abs());
    let range_limiter_2 = |x: I16F16| x - x.floor() - I16F16!(0.5);

    -4 * fake_sin_2(range_limiter_2(x / (2 * I16F16::PI)))
}

pub fn fast_cos(x: I16F16) -> I16F16 {
    fast_sin(x + I16F16::FRAC_PI_2)
}
