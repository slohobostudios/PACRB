use std::ops::Sub;

use tracing::error;

/// # Usage
///
///```
/// # use utils::clamp_to_primitive_bounds;
/// assert_eq!(clamp_to_primitive_bounds!(u8, u16::MAX), u8::MAX);
/// assert_eq!(clamp_to_primitive_bounds!(i16, i32::MIN), i16::MIN);
///```
#[macro_export]
macro_rules! clamp_to_primitive_bounds {
    ( $to:ty, $input:expr ) => {
        $input.clamp(<$to>::MIN.into(), <$to>::MAX.into()) as $to
    };
}

/// # Usage
///
///```
/// # use utils::arithmetic_util_functions::*;
/// assert_eq!(i32_ceil_div(7, 5), 2);
///```
pub fn i32_ceil_div(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

macro_rules! convert_int_or_print_error_and_return_max {
    ( $function_name:ident, $from:ty, $to:ty ) => {
        pub fn $function_name(i: $from) -> $to {
            <$to>::try_from(i).unwrap_or_else(|err| {
                error!("{:?}, input: {}", err, i);
                <$to>::MAX
            })
        }
    };
}
convert_int_or_print_error_and_return_max!(u32_from_usize, usize, u32);
convert_int_or_print_error_and_return_max!(u16_from_usize, usize, u16);
convert_int_or_print_error_and_return_max!(i32_from_usize, usize, i32);
convert_int_or_print_error_and_return_max!(i32_from_u32, u32, i32);

macro_rules! convert_int_or_print_error_and_return_min {
    ( $function_name:ident, $from:ty, $to:ty ) => {
        pub fn $function_name(i: $from) -> $to {
            <$to>::try_from(i).unwrap_or_else(|err| {
                error!("{:?}, input: {}", err, i);
                <$to>::MIN
            })
        }
    };
}
convert_int_or_print_error_and_return_min!(u32_from_i32, i32, u32);

/// Calculates whether two values are within a set standard deviation between eachother
/// UB if you pass in a float that is -inf, inf, or NaN.
pub fn values_within_standard_deviation<T: PartialOrd + Sub>(
    val: T,
    other_val: T,
    standard_deviation: T,
) -> bool
where
    <T as Sub>::Output: PartialOrd<T>,
{
    let (min, max) = if val > other_val {
        (other_val, val)
    } else {
        (val, other_val)
    };

    max - min <= standard_deviation
}
