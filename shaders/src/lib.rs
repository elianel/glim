// https://jack.wrenn.fyi/blog/include-transmute/

macro_rules! include_transmute {
    ($file:expr, $type:ty) => {
        unsafe { core::mem::transmute(*include_bytes!($file)) }
    };
}

pub fn get_test_shader() -> &'static [u32] {
    const LEN: usize = include_bytes!(concat!(env!("OUT_DIR"), "/test.spv")).len() / 4;

    static SHADER: [u32; LEN] =
        include_transmute!(concat!(env!("OUT_DIR"), "/test.spv"), [u32; LEN]);

    &SHADER
}
