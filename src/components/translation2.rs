use crate::math::{Matrix3, Vector2};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Translation2(pub Vector2<f32>);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LocalTranslation2(pub Translation2);

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

    pub fn transform(&mut self, transformation: &Matrix3<f32>) {
        unsafe {
            let x_new = transformation.get_unchecked((0, 0)) * self.0.x
                + transformation.get_unchecked((0, 1)) * self.0.y
                + transformation.get_unchecked((0, 2));
            self.0.y = transformation.get_unchecked((1, 0)) * self.0.x
                + transformation.get_unchecked((1, 1)) * self.0.y
                + transformation.get_unchecked((1, 2));
            self.0.x = x_new;
        }
    }

    pub fn transform_to_copy(&self, transformation: &Matrix3<f32>) -> Translation2 {
        unsafe {
            let x = transformation.get_unchecked((0, 0)) * self.0.x
                + transformation.get_unchecked((0, 1)) * self.0.y
                + transformation.get_unchecked((0, 2));
            let y = transformation.get_unchecked((1, 0)) * self.0.x
                + transformation.get_unchecked((1, 1)) * self.0.y
                + transformation.get_unchecked((1, 2));

            Translation2::new(x, y)
        }
    }
}


impl LocalTranslation2 {
    #[inline(always)]
    pub fn identity() -> Self {
        Self(Translation2::identity())
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
