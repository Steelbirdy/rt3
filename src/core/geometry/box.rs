use crate::core::{
    geometry::{transform::*, *},
    num::*,
    units::Length,
};
use num_traits::NumCast;
use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::*,
};

pub struct Box2<T, U> {
    pub min: Point2<T, U>,
    pub max: Point2<T, U>,
}

pub struct Box3<T, U> {
    pub min: Point3<T, U>,
    pub max: Point3<T, U>,
}

macro_rules! common_impls {
    ($($ty:ident),+) => {$(
impl<T: fmt::Debug, U> fmt::Debug for $ty<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(stringify!($ty))
            .field(&self.min)
            .field(&self.max)
            .finish()
    }
}

impl<T: Copy, U> Copy for $ty<T, U> {}

impl<T: Clone, U> Clone for $ty<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.min.clone(), self.max.clone())
    }
}

impl<T: Eq, U> Eq for $ty<T, U> {}

impl<T: PartialEq, U> PartialEq for $ty<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl<T: Hash, U> Hash for $ty<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.min.hash(state);
        self.max.hash(state);
    }
}

impl<T: Copy + Mul, U> Mul<T> for $ty<T, U> {
    type Output = $ty<T::Output, U>;

    fn mul(self, scale: T) -> Self::Output {
        $ty::new(self.min * scale, self.max * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for $ty<T, U> {
    fn mul_assign(&mut self, scale: T) {
        self.min *= scale;
        self.max *= scale;
    }
}

impl<T: Copy + Div, U> Div<T> for $ty<T, U> {
    type Output = $ty<T::Output, U>;

    fn div(self, scale: T) -> Self::Output {
        $ty::new(self.min / scale, self.max / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for $ty<T, U> {
    fn div_assign(&mut self, scale: T) {
        self.min /= scale;
        self.max /= scale;
    }
}


impl<T: NumCast, U> Cast for $ty<T, U> {
    type Output<NewT: NumCast> = $ty<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        self.min
            .try_cast()
            .zip(self.max.try_cast())
            .map(|(min, max)| $ty::new(min, max))
    }
}

impl<T, U> ToPrimitive for $ty<T, U> where Self: Cast {}

scale_trait_impls!(<T: (Copy), U1, U2> for $ty<_, _> { min, max });
    )+};
}

common_impls!(Box2, Box3);

impl<T: Zero, U> From<Size2<T, U>> for Box2<T, U> {
    fn from(size: Size2<T, U>) -> Self {
        Self::from_size(size)
    }
}

impl<T: Zero, U> From<Size3<T, U>> for Box3<T, U> {
    fn from(size: Size3<T, U>) -> Self {
        Self::from_size(size)
    }
}

impl<T, U> Box2<T, U> {
    #[inline]
    #[must_use]
    pub const fn new(min: Point2<T, U>, max: Point2<T, U>) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn empty() -> Self
    where
        T: Zero,
    {
        Self::new(Point2::zero(), Point2::zero())
    }

    /// Creates a box with the given size at offset zero
    #[inline]
    #[must_use]
    pub fn from_size(size: Size2<T, U>) -> Self
    where
        T: Zero,
    {
        Self::new(Point2::zero(), Point2::new(size.x, size.y))
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        !(self.max.x > self.min.x && self.max.y > self.min.y)
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool
    where
        T: PartialOrd,
    {
        self.min.x < other.max.x
            && other.min.x < self.max.x
            && self.min.y < other.max.y
            && other.min.y < self.max.y
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, p: Point2<T, U>) -> bool
    where
        T: PartialOrd,
    {
        self.min.x <= p.x && p.x < self.max.x && self.min.y <= p.y && p.y < self.max.y
    }

    #[inline]
    #[must_use]
    pub fn contains_box(&self, other: &Self) -> bool
    where
        T: PartialOrd,
    {
        other.is_empty()
            || (self.min.x <= other.min.x
                && other.max.x <= self.max.x
                && self.min.y <= other.min.y
                && other.max.y <= self.max.y)
    }
}

impl<T: Copy, U> Box2<T, U> {
    #[inline]
    #[must_use]
    pub fn erase_unit(&self) -> Box2<T, UnknownUnit> {
        Box2::new(self.min.erase_unit(), self.max.erase_unit())
    }

    #[inline]
    #[must_use]
    pub fn translate(&self, by: Vector2<T, U>) -> Self
    where
        T: Add<Output = T>,
    {
        Self::new(self.min + by, self.max + by)
    }

    #[inline]
    #[must_use]
    pub fn inflate(&self, dx: T, dy: T) -> Self
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let p = Vector2::new(dx, dy);
        Self::new(self.min - p, self.max + p)
    }

    #[inline]
    #[must_use]
    pub fn range(&self, axis: Axis2) -> Range<T> {
        self.min[axis]..self.max[axis]
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Point2<T, U>
    where
        T: One + Add<Output = T> + Div<Output = T>,
    {
        let two = T::one() + T::one();
        (self.min + self.max.to_vector()) / two
    }

    #[inline]
    #[must_use]
    pub fn length(&self, axis: Axis2) -> Length<T::Output, U>
    where
        T: Sub,
    {
        let raw = self.max[axis] - self.min[axis];
        Length::new(raw)
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> Size2<T::Output, U>
    where
        T: Sub,
    {
        (self.max - self.min).to_size()
    }

    #[inline]
    #[must_use]
    pub fn area(&self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        let size = self.size();
        size.x * size.y
    }

    #[inline]
    #[must_use]
    pub fn union(&self, other: &Self) -> Self
    where
        T: PartialOrd,
    {
        if other.is_empty() {
            *self
        } else if self.is_empty() {
            *other
        } else {
            Self {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: PartialOrd,
    {
        let ret = self.intersection_unchecked(other);
        (!ret.is_empty()).then_some(ret)
    }

    #[inline]
    #[must_use]
    pub fn intersection_unchecked(&self, other: &Self) -> Self
    where
        T: PartialOrd,
    {
        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    #[must_use]
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point2<T, U>>,
        T: PartialOrd + Zero,
    {
        let mut points = points.into_iter();

        let Some(mut min) = points.next()
            else { return Self::empty(); };
        let mut max = min;

        for point in points {
            min = point.min(min);
            max = point.max(max);
        }

        Self::new(min, max)
    }

    pub fn try_from_points<I, E>(points: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Point2<T, U>, E>>,
        T: PartialOrd + Zero,
    {
        let mut points = points.into_iter();

        let mut min = match points.next() {
            Some(p) => p?,
            None => return Ok(Self::empty()),
        };
        let mut max = min;

        for point in points {
            let point = point?;
            min = point.min(min);
            max = point.max(max);
        }

        Ok(Self::new(min, max))
    }

    #[inline]
    #[must_use]
    pub fn lerp(&self, other: &Self, t: T) -> Self
    where
        T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let min = self.min.lerp(other.min, t);
        let max = self.max.lerp(other.max, t);
        Self::new(min, max)
    }
}

impl<T, U> Box3<T, U> {
    #[inline]
    #[must_use]
    pub const fn new(min: Point3<T, U>, max: Point3<T, U>) -> Self {
        Self { min, max }
    }

    #[inline]
    #[must_use]
    pub fn empty() -> Self
    where
        T: Zero,
    {
        Self::new(Point3::zero(), Point3::zero())
    }

    /// Creates a box with the given size at offset zero
    #[inline]
    #[must_use]
    pub fn from_size(size: Size3<T, U>) -> Self
    where
        T: Zero,
    {
        Self::new(Point3::zero(), Point3::new(size.x, size.y, size.z))
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool
    where
        T: PartialOrd,
    {
        !(self.max.x > self.min.x && self.max.y > self.min.y && self.max.z > self.min.z)
    }

    #[inline]
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool
    where
        T: PartialOrd,
    {
        self.min.x < other.max.x
            && other.min.x < self.max.x
            && self.min.y < other.max.y
            && other.min.y < self.max.y
            && self.min.z < other.max.z
            && other.min.z < self.max.z
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, p: Point3<T, U>) -> bool
    where
        T: PartialOrd,
    {
        self.min.x <= p.x
            && p.x < self.max.x
            && self.min.y <= p.y
            && p.y < self.max.y
            && self.min.z <= p.z
            && p.z < self.max.z
    }

    #[inline]
    #[must_use]
    pub fn contains_box(&self, other: &Self) -> bool
    where
        T: PartialOrd,
    {
        other.is_empty()
            || (self.min.x <= other.min.x
                && other.max.x <= self.max.x
                && self.min.y <= other.min.y
                && other.max.y <= self.max.y
                && self.min.z <= other.min.z
                && other.max.z <= self.max.z)
    }
}

impl<T: Copy, U> Box3<T, U> {
    #[inline]
    #[must_use]
    pub fn erase_unit(&self) -> Box3<T, UnknownUnit> {
        Box3::new(self.min.erase_unit(), self.max.erase_unit())
    }

    #[inline]
    #[must_use]
    pub fn translate(&self, by: Vector3<T, U>) -> Self
    where
        T: Add<Output = T>,
    {
        Self::new(self.min + by, self.max + by)
    }

    #[inline]
    #[must_use]
    pub fn inflate(&self, dx: T, dy: T, dz: T) -> Self
    where
        T: Add<Output = T> + Sub<Output = T>,
    {
        let p = Vector3::new(dx, dy, dz);
        Self::new(self.min - p, self.max + p)
    }

    #[inline]
    #[must_use]
    pub fn range(&self, axis: Axis3) -> Range<T> {
        self.min[axis]..self.max[axis]
    }

    #[inline]
    #[must_use]
    pub fn center(&self) -> Point3<T, U>
    where
        T: One + Add<Output = T> + Div<Output = T>,
    {
        let two = T::one() + T::one();
        (self.min + self.max.to_vector()) / two
    }

    #[inline]
    #[must_use]
    pub fn length(&self, axis: Axis3) -> Length<T::Output, U>
    where
        T: Sub,
    {
        let raw = self.max[axis] - self.min[axis];
        Length::new(raw)
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> Size3<T::Output, U>
    where
        T: Sub,
    {
        (self.max - self.min).to_size()
    }

    #[inline]
    #[must_use]
    pub fn area(&self, axis1: Axis3, axis2: Axis3) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        (self.max[axis1] - self.min[axis1]) * (self.max[axis2] - self.min[axis2])
    }

    #[inline]
    #[must_use]
    pub fn volume(&self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        let size = self.size();
        size.x * size.y * size.z
    }

    #[inline]
    #[must_use]
    pub fn union(&self, other: &Self) -> Self
    where
        T: PartialOrd,
    {
        if other.is_empty() {
            *self
        } else if self.is_empty() {
            *other
        } else {
            Self {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self>
    where
        T: PartialOrd,
    {
        let ret = self.intersection_unchecked(other);
        (!ret.is_empty()).then_some(ret)
    }

    #[inline]
    #[must_use]
    pub fn intersection_unchecked(&self, other: &Self) -> Self
    where
        T: PartialOrd,
    {
        Self {
            min: self.min.max(other.min),
            max: self.max.min(other.max),
        }
    }

    #[must_use]
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point3<T, U>>,
        T: PartialOrd + Zero,
    {
        let mut points = points.into_iter();

        let Some(mut min) = points.next()
            else { return Self::empty(); };
        let mut max = min;

        for point in points {
            min = point.min(min);
            max = point.max(max);
        }

        Self::new(min, max)
    }

    pub fn try_from_points<I, E>(points: I) -> Result<Self, E>
    where
        I: IntoIterator<Item = Result<Point3<T, U>, E>>,
        T: PartialOrd + Zero,
    {
        let mut points = points.into_iter();

        let mut min = match points.next() {
            Some(p) => p?,
            None => return Ok(Self::empty()),
        };
        let mut max = min;

        for point in points {
            let point = point?;
            min = point.min(min);
            max = point.max(max);
        }

        Ok(Self::new(min, max))
    }

    #[inline]
    #[must_use]
    pub fn lerp(&self, other: &Self, t: T) -> Self
    where
        T: One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let min = self.min.lerp(other.min, t);
        let max = self.max.lerp(other.max, t);
        Self::new(min, max)
    }

    /// The original box contains the resulting box
    #[inline]
    #[must_use]
    pub fn round_in(&self) -> Self
    where
        T: Ceil + Floor,
    {
        Self::new(self.min.ceil(), self.max.floor())
    }

    /// The original box is contained in the resulting box
    #[inline]
    #[must_use]
    pub fn round_out(&self) -> Self
    where
        T: Ceil + Floor,
    {
        Self::new(self.min.floor(), self.max.ceil())
    }
}
