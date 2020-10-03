use crate::math::Matrix3;
use shrinkwraprs::Shrinkwrap;

// cos(R) -sin(R) x
// sin(R) cos(R)  y
// 0      0       1

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct Rotation2(pub f32);
impl Rotation2 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self { 0: 0.0 }
    }

    #[rustfmt::skip]
    pub fn to_homogeneous(&self) -> Matrix3<f32> {
        let cos = self.0.cos();
        let sin = self.0.sin();

        Matrix3::<f32>::new(
            cos, -sin, 0.0,
            sin, cos, 0.0, 
            0.0, 0.0, 1.0
        )
    }

    pub fn constrained(&self) -> Self {
        let mut copy = *self;
        copy.constrain();
        copy
    }

    pub fn top() -> Self {
        Rotation2(0.0)
    }

    pub fn right() -> Self {
        Rotation2(std::f32::consts::FRAC_PI_2)
    }

    pub fn bottom() -> Self {
        Rotation2(std::f32::consts::PI)
    }

    pub fn left() -> Self {
        Rotation2(3.0 * std::f32::consts::PI / 2.0)
    }

    pub fn constrain(&mut self) {
        while self.0 >= 2.0 * std::f32::consts::PI {
            self.0 -= 2.0 * std::f32::consts::PI;
        }

        while self.0 < 0.0 {
            self.0 += 2.0 * std::f32::consts::PI;
        }
    }

    #[allow(clippy::float_cmp)]
    pub fn is_cardinal(&self) -> bool {
        debug_assert_eq!(self.0, self.constrained().0);

        self.0 == Rotation2::top().0
            || self.0 == Rotation2::right().0
            || self.0 == Rotation2::bottom().0
            || self.0 == Rotation2::left().0
    }
}

impl Default for Rotation2 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<f32> for Rotation2 {
    fn from(rotation: f32) -> Self {
        Self(rotation)
    }
}
