use crate::math::Matrix3;
use shrinkwraprs::Shrinkwrap;
use std::fmt;

// cos(R) -sin(R) x
// sin(R) cos(R)  y
// 0      0       1

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct LocalToWorld2(pub Matrix3<f32>);

impl LocalToWorld2 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(Matrix3::identity())
    }
}

impl Default for LocalToWorld2 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for LocalToWorld2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
