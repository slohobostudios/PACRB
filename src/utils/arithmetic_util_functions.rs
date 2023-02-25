use tracing::error;

pub fn i32_ceil_div(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

pub fn i32_from_u32(i: u32) -> i32 {
    i32::try_from(i).unwrap_or_else(|err| {
        error!("{:?}. input: {}", err, i);
        i32::MAX
    })
}

pub fn i32_from_usize(i: usize) -> i32 {
    i32::try_from(i).unwrap_or_else(|err| {
        error!("{:?}. input: {}", err, i);
        i32::MAX
    })
}

pub fn u16_from_usize(i: usize) -> u16 {
    u16::try_from(i).unwrap_or_else(|err| {
        error!("{:?}, input: {}", err, i);
        u16::MAX
    })
}

pub fn u32_from_usize(i: usize) -> u32 {
    u32::try_from(i).unwrap_or_else(|err| {
        error!("{:?}, input: {}", err, i);
        u32::MAX
    })
}
