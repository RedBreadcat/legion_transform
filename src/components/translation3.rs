use crate::math::{self, Vector3};
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct Translation3(pub math::Translation3<f32>);

impl Translation3 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(math::Translation3::identity())
    }

    #[inline(always)]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(math::Translation3::new(x, y, z))
    }
}

impl Default for Translation3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Vector3<f32>> for Translation3 {
    fn from(translation: Vector3<f32>) -> Self {
        Self( math::Translation3::from(translation))
    }
}

impl From<math::Translation3<f32>> for Translation3 {
    fn from(translation: math::Translation3<f32>) -> Self {
        Self(translation)
    }
}
