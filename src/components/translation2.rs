use crate::math::{Vector2, Matrix3};
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct Translation2(pub Vector2<f32>);

impl Translation2 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(Vector2::zeros())
    }

    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }

    #[rustfmt::skip]
    pub fn to_homogeneous(&self) -> Matrix3<f32> {
        Matrix3::<f32>::new(
            1.0, 0.0, self.0.x,
            0.0, 1.0, self.0.y, 
            0.0, 0.0, 1.0
        )
    }
}

impl Default for Translation2 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Vector2<f32>> for Translation2 {
    fn from(translation: Vector2<f32>) -> Self {
        Self(translation)
    }
}

/*impl From<math::Translation2<f32>> for Translation2 {
    fn from(translation: math::Translation2<f32>) -> Self {
        Self(translation)
    }
}
*/