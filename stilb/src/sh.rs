use crate::math::Vector3;

#[repr(C)]
pub struct SHProbe {
    pub l0: Vector3,
    pub sample_count: f32,

    pub l1x: Vector3,
    pub position_x: f32,

    pub l1y: Vector3,
    pub position_y: f32,

    pub l1z: Vector3,
    pub position_z: f32,
}
