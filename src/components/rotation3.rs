use crate::math::UnitQuaternion;
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct Rotation3(pub UnitQuaternion<f32>);
impl Rotation3 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(UnitQuaternion::identity())
    }

    #[inline(always)]
    pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Self {
        Self(UnitQuaternion::from_euler_angles(roll, pitch, yaw))
    }
}

impl Default for Rotation3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<UnitQuaternion<f32>> for Rotation3 {
    fn from(rotation: UnitQuaternion<f32>) -> Self {
        Self(rotation)
    }
}
