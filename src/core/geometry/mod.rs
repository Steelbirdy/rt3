mod r#box;
mod mask;
mod point;
mod ray;
mod size;
pub mod transform;
mod vector;

pub use mask::{Mask2, Mask3};
pub use point::{Point2, Point3};
pub use r#box::{Box2, Box3};
pub use ray::Ray;
pub use size::{Size2, Size3};
pub use vector::{Vector2, Vector3};

pub struct Normal<U>(std::marker::PhantomData<U>);

pub enum UnknownUnit {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis2 {
    X,
    Y,
}

impl Axis2 {
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    #[inline]
    #[must_use]
    pub fn next(self) -> Self {
        use Axis2::*;
        match self {
            X => Y,
            Y => X,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Axis3 {
    X,
    Y,
    Z,
}

impl Axis3 {
    pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];

    #[inline]
    #[must_use]
    pub fn next(self) -> Self {
        use Axis3::*;
        match self {
            X => Y,
            Y => Z,
            Z => X,
        }
    }
}
