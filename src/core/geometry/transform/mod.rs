mod rotation;
mod scale;
mod transform;
mod translation;
mod homogen;

pub use homogen::HomogeneousVector;
pub use rotation::{Rotation2, Rotation3};
pub use scale::Scale;
pub use transform::{Transform2, Transform3};
pub use translation::{Translation2, Translation3};

pub trait Transformation<T, Src, Dst>: Sized {
    type Inverse: Transformation<T, Dst, Src, Inverse = Self>;

    #[must_use]
    fn identity() -> Self;

    #[must_use]
    fn is_identity(&self) -> bool;

    #[must_use]
    fn inverse(&self) -> Self::Inverse;

    #[inline]
    #[must_use]
    fn into_inverse(self) -> Self::Inverse {
        self.inverse()
    }

    #[inline]
    #[must_use]
    fn transform<A>(&self, v: A) -> Self::Output
    where
        Self: Transform<A>,
    {
        Transform::transform(self, v)
    }
}

mod _transform {
    pub trait Transform<T> {
        type Output;

        #[must_use]
        fn transform(&self, v: T) -> Self::Output;
    }
}

pub(in crate::core) use _transform::Transform;
