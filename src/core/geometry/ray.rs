use crate::core::{
    geometry::{Point3, Vector3},
    units::Time,
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Mul},
};

pub struct Ray<T, U, D = ()> {
    pub origin: Point3<T, U>,
    pub dir: Vector3<T, U>,
    pub data: D,
}

impl<T: fmt::Debug, U, D: fmt::Debug> fmt::Debug for Ray<T, U, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ray")
            .field("origin", &self.origin)
            .field("dir", &self.dir)
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Copy, U, D: Copy> Copy for Ray<T, U, D> {}

impl<T: Clone, U, D: Clone> Clone for Ray<T, U, D> {
    fn clone(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            dir: self.dir.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T: Eq, U, D: Eq> Eq for Ray<T, U, D> {}

impl<T: PartialEq, U, D: PartialEq> PartialEq for Ray<T, U, D> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.dir == other.dir && self.data == other.data
    }
}

impl<T: Hash, U, D: Hash> Hash for Ray<T, U, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.origin.hash(state);
        self.dir.hash(state);
        self.data.hash(state);
    }
}

impl<T, U> Ray<T, U> {
    #[inline]
    #[must_use]
    pub const fn new(origin: Point3<T, U>, dir: Vector3<T, U>) -> Self {
        Self::with_data(origin, dir, ())
    }
}

impl<T, U, D> Ray<T, U, D> {
    #[inline]
    #[must_use]
    pub const fn with_data(origin: Point3<T, U>, dir: Vector3<T, U>, data: D) -> Self {
        Self { origin, dir, data }
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self
    where
        T: num_traits::real::Real,
    {
        Self {
            dir: self.dir.normalize(),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn at(&self, t: Time<T>) -> Point3<T, U>
    where
        T: Copy,
        Point3<T, U>: Add<Vector3<T, U>, Output = Point3<T, U>>,
        Vector3<T, U>: Mul<T, Output = Vector3<T, U>>,
    {
        self.origin + self.dir * t.0
    }
}
