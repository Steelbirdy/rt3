use crate::core::{
    geometry::{
        transform::{Transform, Transformation},
        *,
    },
    num::One,
    units::Length,
};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
};

pub struct Scale<T, Src, Dst>(pub T, PhantomData<(Src, Dst)>);

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Scale<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Copy, Src, Dst> Copy for Scale<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for Scale<T, Src, Dst> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T: Eq, Src, Dst> Eq for Scale<T, Src, Dst> {}

impl<T: PartialEq, Src, Dst> PartialEq for Scale<T, Src, Dst> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Ord, Src, Dst> Ord for Scale<T, Src, Dst> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: PartialOrd, Src, Dst> PartialOrd for Scale<T, Src, Dst> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Hash, Src, Dst> Hash for Scale<T, Src, Dst> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, Src, Dst> Scale<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub const fn new(factor: T) -> Self {
        Self(factor, PhantomData)
    }

    #[inline]
    #[must_use]
    pub fn get(self) -> T {
        self.0
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Scale<T, Src, Dst>
where
    T: Copy + PartialEq + One + Div<Output = T>,
{
    type Inverse = Scale<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Self::one()
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.0 == T::one()
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        Scale::new(T::one() / self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Length<T, U1>> for Scale<T, U1, U2> {
    type Output = Length<T::Output, U2>;

    #[inline]
    fn transform(&self, length: Length<T, U1>) -> Self::Output {
        Length::new(length.0 * self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Point2<T, U1>> for Scale<T, U1, U2> {
    type Output = Point2<T::Output, U2>;

    #[inline]
    fn transform(&self, p: Point2<T, U1>) -> Self::Output {
        Point2::new(p.x * self.0, p.y * self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Point3<T, U1>> for Scale<T, U1, U2> {
    type Output = Point3<T::Output, U2>;

    #[inline]
    fn transform(&self, p: Point3<T, U1>) -> Self::Output {
        Point3::new(p.x * self.0, p.y * self.0, p.z * self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Vector2<T, U1>> for Scale<T, U1, U2> {
    type Output = Vector2<T::Output, U2>;

    #[inline]
    fn transform(&self, v: Vector2<T, U1>) -> Self::Output {
        Vector2::new(v.x * self.0, v.y * self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Vector3<T, U1>> for Scale<T, U1, U2> {
    type Output = Vector3<T::Output, U2>;

    #[inline]
    fn transform(&self, v: Vector3<T, U1>) -> Self::Output {
        Vector3::new(v.x * self.0, v.y * self.0, v.z * self.0)
    }
}

impl<T: Copy + Div, U1, U2> Transform<Vector2<T, Normal<U1>>> for Scale<T, U1, U2> {
    type Output = Vector2<T::Output, Normal<U2>>;

    #[inline]
    fn transform(&self, n: Vector2<T, Normal<U1>>) -> Self::Output {
        Vector2::new(n.x / self.0, n.y / self.0)
    }
}

impl<T: Copy + Div, U1, U2> Transform<Vector3<T, Normal<U1>>> for Scale<T, U1, U2> {
    type Output = Vector3<T::Output, Normal<U2>>;

    #[inline]
    fn transform(&self, n: Vector3<T, Normal<U1>>) -> Self::Output {
        Vector3::new(n.x / self.0, n.y / self.0, n.z / self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Box2<T, U1>> for Scale<T, U1, U2> {
    type Output = Box2<T::Output, U2>;

    #[inline]
    fn transform(&self, b: Box2<T, U1>) -> Self::Output {
        Box2::new(self.transform(b.min), self.transform(b.max))
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Box3<T, U1>> for Scale<T, U1, U2> {
    type Output = Box3<T::Output, U2>;

    #[inline]
    fn transform(&self, b: Box3<T, U1>) -> Self::Output {
        Box3::new(self.transform(b.min), self.transform(b.max))
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Size2<T, U1>> for Scale<T, U1, U2> {
    type Output = Size2<T::Output, U2>;

    #[inline]
    fn transform(&self, size: Size2<T, U1>) -> Self::Output {
        Size2::new(size.x * self.0, size.y * self.0)
    }
}

impl<T: Copy + Mul, U1, U2> Transform<Size3<T, U1>> for Scale<T, U1, U2> {
    type Output = Size3<T::Output, U2>;

    #[inline]
    fn transform(&self, size: Size3<T, U1>) -> Self::Output {
        Size3::new(size.x * self.0, size.y * self.0, size.z * self.0)
    }
}

impl<T: One, Src, Dst> One for Scale<T, Src, Dst> {
    #[inline]
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: Add, Src, Dst> Add<Self> for Scale<T, Src, Dst> {
    type Output = Scale<T::Output, Src, Dst>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Scale::new(self.0 + rhs.0)
    }
}

impl<T: Sub, Src, Dst> Sub<Self> for Scale<T, Src, Dst> {
    type Output = Scale<T::Output, Src, Dst>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Scale::new(self.0 - rhs.0)
    }
}

impl<T: Mul, A, B, C> Mul<Scale<T, B, C>> for Scale<T, A, B> {
    type Output = Scale<T::Output, A, C>;

    #[inline]
    fn mul(self, rhs: Scale<T, B, C>) -> Self::Output {
        Scale::new(self.0 * rhs.0)
    }
}
