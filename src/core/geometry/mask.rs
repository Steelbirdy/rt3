use crate::core::geometry::{Axis2, Axis3, Point2, Point3, Vector2, Vector3};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mask2 {
    pub x: bool,
    pub y: bool,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Mask3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

impl Mask2 {
    #[inline]
    #[must_use]
    const fn new(x: bool, y: bool) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn all(self) -> bool {
        self.x && self.y
    }

    #[inline]
    #[must_use]
    pub const fn any(self) -> bool {
        self.x || self.y
    }

    #[inline]
    #[must_use]
    pub const fn none(self) -> bool {
        !self.any()
    }

    #[inline]
    #[must_use]
    pub const fn not(self) -> Self {
        Self::new(!self.x, !self.y)
    }

    #[inline]
    #[must_use]
    pub const fn and(self, rhs: Self) -> Self {
        Self::new(self.x && rhs.x, self.y && rhs.y)
    }

    #[inline]
    #[must_use]
    pub const fn or(self, rhs: Self) -> Self {
        Self::new(self.x || rhs.x, self.y || rhs.y)
    }

    pub fn select<T>(self, where_true: T, where_false: T) -> T
    where
        Self: Select<T>,
    {
        Select::select(self, where_true, where_false)
    }
}

impl Mask3 {
    #[inline]
    #[must_use]
    const fn new(x: bool, y: bool, z: bool) -> Self {
        Self { x, y, z }
    }

    #[inline]
    #[must_use]
    pub const fn all(self) -> bool {
        self.x && self.y && self.z
    }

    #[inline]
    #[must_use]
    pub const fn any(self) -> bool {
        self.x || self.y || self.z
    }

    #[inline]
    #[must_use]
    pub const fn none(self) -> bool {
        !self.any()
    }

    #[inline]
    #[must_use]
    pub const fn not(self) -> Self {
        Self::new(!self.x, !self.y, !self.z)
    }

    #[inline]
    #[must_use]
    pub const fn and(self, rhs: Self) -> Self {
        Self::new(self.x && rhs.x, self.y && rhs.y, self.z && rhs.z)
    }

    #[inline]
    #[must_use]
    pub const fn or(self, rhs: Self) -> Self {
        Self::new(self.x || rhs.x, self.y || rhs.y, self.z || rhs.z)
    }

    pub fn select<T>(self, where_true: T, where_false: T) -> T
    where
        Self: Select<T>,
    {
        Select::select(self, where_true, where_false)
    }
}

impl std::ops::Index<Axis2> for Mask2 {
    type Output = bool;

    #[inline]
    fn index(&self, axis: Axis2) -> &Self::Output {
        match axis {
            Axis2::X => &self.x,
            Axis2::Y => &self.y,
        }
    }
}

impl std::ops::IndexMut<Axis2> for Mask2 {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut Self::Output {
        match axis {
            Axis2::X => &mut self.x,
            Axis2::Y => &mut self.y,
        }
    }
}

impl std::ops::Index<Axis3> for Mask3 {
    type Output = bool;

    #[inline]
    fn index(&self, axis: Axis3) -> &Self::Output {
        match axis {
            Axis3::X => &self.x,
            Axis3::Y => &self.y,
            Axis3::Z => &self.z,
        }
    }
}

impl std::ops::IndexMut<Axis3> for Mask3 {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut Self::Output {
        match axis {
            Axis3::X => &mut self.x,
            Axis3::Y => &mut self.y,
            Axis3::Z => &mut self.z,
        }
    }
}

pub trait Select<T> {
    fn select(self, a: T, b: T) -> T;
}

impl<T, U> Select<Point2<T, U>> for Mask2 {
    fn select(self, a: Point2<T, U>, b: Point2<T, U>) -> Point2<T, U> {
        Point2::new(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
        )
    }
}

impl<T, U> Select<Point3<T, U>> for Mask3 {
    fn select(self, a: Point3<T, U>, b: Point3<T, U>) -> Point3<T, U> {
        Point3::new(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
            if self.z { a.z } else { b.z },
        )
    }
}

impl<T, U> Select<Vector2<T, U>> for Mask2 {
    fn select(self, a: Vector2<T, U>, b: Vector2<T, U>) -> Vector2<T, U> {
        Vector2::new(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
        )
    }
}

impl<T, U> Select<Vector3<T, U>> for Mask3 {
    fn select(self, a: Vector3<T, U>, b: Vector3<T, U>) -> Vector3<T, U> {
        Vector3::new(
            if self.x { a.x } else { b.x },
            if self.y { a.y } else { b.y },
            if self.z { a.z } else { b.z },
        )
    }
}
