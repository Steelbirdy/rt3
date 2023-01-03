use std::{marker::PhantomData, hash::{Hash, Hasher}};
use num_traits::NumOps;
use crate::core::{geometry::*, num::*};

pub struct HomogeneousVector<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    _unit: PhantomData<U>,
}

impl<T: Copy, U> Copy for HomogeneousVector<T, U> {}

impl<T: Clone, U> Clone for HomogeneousVector<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
    }
}

impl<T: Eq, U> Eq for HomogeneousVector<T, U> {}

impl<T: PartialEq, U> PartialEq for HomogeneousVector<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<T: Hash, U> Hash for HomogeneousVector<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}

impl<T, U> HomogeneousVector<T, U> {
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w, _unit: PhantomData }
    }
}

impl<T, U> TryFrom<HomogeneousVector<T, U>> for Point2<T, U>
where
    T: Copy + PartialOrd + Zero + One + NumOps,
{
    type Error = ();

    #[inline]
    fn try_from(v: HomogeneousVector<T, U>) -> Result<Self, Self::Error> {
        if v.w > T::zero() {
            let w_inv = T::one() / v.w;
            Ok(Self::new(v.x * w_inv, v.y * w_inv))
        } else {
            Err(())
        }
    }
}

impl<T, U> TryFrom<HomogeneousVector<T, U>> for Point3<T, U>
where
    T: Copy + PartialOrd + Zero + One + NumOps,
{
    type Error = ();

    #[inline]
    fn try_from(v: HomogeneousVector<T, U>) -> Result<Self, Self::Error> {
        if v.w > T::zero() {
            let w_inv = T::one() / v.w;
            Ok(Self::new(v.x * w_inv, v.y * w_inv, v.z * w_inv))
        } else {
            Err(())
        }
    }
}

impl<T: Zero, U> From<Vector2<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(v: Vector2<T, U>) -> Self {
        Self::new(v.x, v.y, T::zero(), T::zero())
    }
}

impl<T: Zero, U> From<Vector3<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(v: Vector3<T, U>) -> Self {
        Self::new(v.x, v.y, v.z, T::zero())
    }
}

impl<T: Zero + One, U> From<Point2<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(p: Point2<T, U>) -> Self {
        Self::new(p.x, p.y, T::zero(), T::one())
    }
}

impl<T: One, U> From<Point3<T, U>> for HomogeneousVector<T, U> {
    #[inline]
    fn from(p: Point3<T, U>) -> Self {
        Self::new(p.x, p.y, p.z, T::one())
    }
}