use crate::core::{
    geometry::{
        transform::{Transform, Transformation},
        *,
    },
    num::*,
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
};

pub struct Translation2<T, Src, Dst> {
    pub x: T,
    pub y: T,
    _unit: PhantomData<(Src, Dst)>,
}

pub struct Translation3<T, Src, Dst> {
    pub x: T,
    pub y: T,
    pub z: T,
    _unit: PhantomData<(Src, Dst)>,
}

macro_rules! common_impls {
    ($($ty:ident { $($field:ident),+ }),+) => {$(
impl<T: Default, Src, Dst> Default for $ty<T, Src, Dst> {
    fn default() -> Self {
        Self {
            $($field: T::default(),)+
            _unit: PhantomData,
        }
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for $ty<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Translation")
            $( .field(&self.$field) )+
            .finish()
    }
}

impl<T: Copy, Src, Dst> Copy for $ty<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for $ty<T, Src, Dst> {
    fn clone(&self) -> Self {
        Self::new($(self.$field.clone()),+)
    }
}

impl<T: Eq, Src, Dst> Eq for $ty<T, Src, Dst> {}

impl<T: PartialEq, Src, Dst> PartialEq for $ty<T, Src, Dst> {
    fn eq(&self, other: &Self) -> bool {
        $(self.$field == other.$field)&&+
    }
}

impl<T: Hash, Src, Dst> Hash for $ty<T, Src, Dst> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        $( self.$field.hash(state); )+
    }
}

impl<T: Zero, Src, Dst> Zero for $ty<T, Src, Dst> {
    fn zero() -> Self {
        Self {
            $( $field: T::zero(), )+
            _unit: PhantomData,
        }
    }
}
    )+};
}

common_impls![Translation2 { x, y }, Translation3 { x, y, z }];

impl<T, Src, Dst> Translation2<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Translation3<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy, Src, Dst> Translation2<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub fn splat(v: T) -> Self {
        Self::new(v, v)
    }

    #[inline]
    #[must_use]
    pub fn to_vector(&self) -> Vector2<T, Src> {
        Vector2::new(self.x, self.y)
    }
}

impl<T: Copy, Src, Dst> Translation3<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub fn splat(v: T) -> Self {
        Self::new(v, v, v)
    }

    #[inline]
    #[must_use]
    pub fn to_vector(&self) -> Vector3<T, Src> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Translation2<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + Neg<Output = T>,
{
    type Inverse = Translation2<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Zero::zero()
    }

    #[inline]
    fn is_identity(&self) -> bool {
        let zero = T::zero();
        self.x == zero && self.y == zero
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        Translation2::new(-self.x, -self.y)
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Translation3<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + Neg<Output = T>,
{
    type Inverse = Translation3<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Zero::zero()
    }

    #[inline]
    fn is_identity(&self) -> bool {
        let zero = T::zero();
        self.x == zero && self.y == zero && self.z == zero
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        Translation3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Copy + Add, Src, Dst> Transform<Point2<T, Src>> for Translation2<T, Src, Dst> {
    type Output = Point2<T::Output, Dst>;

    #[inline]
    fn transform(&self, p: Point2<T, Src>) -> Self::Output {
        Point2::new(p.x + self.x, p.y + self.y)
    }
}

impl<T: Copy, Src, Dst> Transform<Vector2<T, Src>> for Translation2<T, Src, Dst> {
    type Output = Vector2<T, Dst>;

    #[inline]
    fn transform(&self, v: Vector2<T, Src>) -> Self::Output {
        Vector2::new(v.x, v.y)
    }
}

impl<T: Copy, Src, Dst> Transform<Vector2<T, Normal<Src>>> for Translation2<T, Src, Dst> {
    type Output = Vector2<T, Normal<Dst>>;

    #[inline]
    fn transform(&self, v: Vector2<T, Normal<Src>>) -> Self::Output {
        Vector2::new(v.x, v.y)
    }
}

impl<T: Copy + Add, Src, Dst> Transform<Box2<T, Src>> for Translation2<T, Src, Dst> {
    type Output = Box2<T::Output, Dst>;

    #[inline]
    fn transform(&self, b: Box2<T, Src>) -> Self::Output {
        Box2::new(self.transform(b.min), self.transform(b.max))
    }
}

impl<T, Src, Dst> Transform<Size2<T, Src>> for Translation2<T, Src, Dst> {
    type Output = Size2<T, Dst>;

    #[inline]
    fn transform(&self, s: Size2<T, Src>) -> Self::Output {
        Size2::new(s.x, s.y)
    }
}

impl<T: Copy + Add, Src, Dst> Transform<Point3<T, Src>> for Translation3<T, Src, Dst> {
    type Output = Point3<T::Output, Dst>;

    #[inline]
    fn transform(&self, p: Point3<T, Src>) -> Self::Output {
        Point3::new(p.x + self.x, p.y + self.y, p.z + self.z)
    }
}

impl<T: Copy, Src, Dst> Transform<Vector3<T, Src>> for Translation3<T, Src, Dst> {
    type Output = Vector3<T, Dst>;

    #[inline]
    fn transform(&self, v: Vector3<T, Src>) -> Self::Output {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl<T: Copy, Src, Dst> Transform<Vector3<T, Normal<Src>>> for Translation3<T, Src, Dst> {
    type Output = Vector3<T, Normal<Dst>>;

    #[inline]
    fn transform(&self, v: Vector3<T, Normal<Src>>) -> Self::Output {
        Vector3::new(v.x, v.y, v.z)
    }
}

impl<T: Copy + Add, Src, Dst> Transform<Box3<T, Src>> for Translation3<T, Src, Dst> {
    type Output = Box3<T::Output, Dst>;

    #[inline]
    fn transform(&self, b: Box3<T, Src>) -> Self::Output {
        Box3::new(self.transform(b.min), self.transform(b.max))
    }
}

impl<T, Src, Dst> Transform<Size3<T, Src>> for Translation3<T, Src, Dst> {
    type Output = Size3<T, Dst>;

    #[inline]
    fn transform(&self, s: Size3<T, Src>) -> Self::Output {
        Size3::new(s.x, s.y, s.z)
    }
}

impl<T, Src, Dst> From<Vector2<T, Src>> for Translation2<T, Src, Dst> {
    #[inline]
    fn from(v: Vector2<T, Src>) -> Self {
        Self::new(v.x, v.y)
    }
}

impl<T, Src, Dst> From<Vector3<T, Src>> for Translation3<T, Src, Dst> {
    #[inline]
    fn from(v: Vector3<T, Src>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T: Add, A, B, C> Add<Translation2<T, B, C>> for Translation2<T, A, B> {
    type Output = Translation2<T::Output, A, C>;

    #[inline]
    fn add(self, rhs: Translation2<T, B, C>) -> Self::Output {
        Translation2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign, Src, Dst> AddAssign<Translation2<T, Dst, Dst>> for Translation2<T, Src, Dst> {
    #[inline]
    fn add_assign(&mut self, rhs: Translation2<T, Dst, Dst>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub, A, B, C> Sub<Translation2<T, B, C>> for Translation2<T, A, C> {
    type Output = Translation2<T::Output, A, B>;

    #[inline]
    fn sub(self, rhs: Translation2<T, B, C>) -> Self::Output {
        Translation2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign, Src, Dst> SubAssign<Translation2<T, Dst, Dst>> for Translation2<T, Src, Dst> {
    #[inline]
    fn sub_assign(&mut self, rhs: Translation2<T, Dst, Dst>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Add, A, B, C> Add<Translation3<T, B, C>> for Translation3<T, A, B> {
    type Output = Translation3<T::Output, A, C>;

    #[inline]
    fn add(self, rhs: Translation3<T, B, C>) -> Self::Output {
        Translation3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign, Src, Dst> AddAssign<Translation3<T, Dst, Dst>> for Translation3<T, Src, Dst> {
    #[inline]
    fn add_assign(&mut self, rhs: Translation3<T, Dst, Dst>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub, A, B, C> Sub<Translation3<T, B, C>> for Translation3<T, A, C> {
    type Output = Translation3<T::Output, A, B>;

    #[inline]
    fn sub(self, rhs: Translation3<T, B, C>) -> Self::Output {
        Translation3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign, Src, Dst> SubAssign<Translation3<T, Dst, Dst>> for Translation3<T, Src, Dst> {
    #[inline]
    fn sub_assign(&mut self, rhs: Translation3<T, Dst, Dst>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
