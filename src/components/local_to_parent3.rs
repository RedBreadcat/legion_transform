use crate::math::Matrix4;
use shrinkwraprs::Shrinkwrap;
use std::fmt;

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct LocalToParent3(pub Matrix4<f32>);

impl LocalToParent3 {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}

impl Default for LocalToParent3 {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for LocalToParent3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
