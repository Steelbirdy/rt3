use crate::core::{
    geometry::{
        transform::{Transform, Transformation},
        *,
    },
    num::*,
    units::Angle,
};
use num_traits::real::Real;
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
};

pub struct Rotation2<T, Src, Dst> {
    pub angle: Angle<T>,
    _unit: PhantomData<(Src, Dst)>,
}

pub struct Rotation3<T, Src, Dst> {
    pub a: T,
    pub i: T,
    pub j: T,
    pub k: T,
    _unit: PhantomData<(Src, Dst)>,
}

macro_rules! common_impls {
    ($($ty:ident { $($field:ident),+ }),+) => {$(
impl<T: Copy, Src, Dst> Copy for $ty<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for $ty<T, Src, Dst> {
    fn clone(&self) -> Self {
        Self {
            $( $field: self.$field.clone(), )+
            _unit: PhantomData,
        }
    }
}

impl<T: Eq, Src, Dst> Eq for $ty<T, Src, Dst> {}

impl<T: PartialEq, Src, Dst> PartialEq for $ty<T, Src, Dst> {
    fn eq(&self, other: &Self) -> bool {
        $( self.$field == other.$field )&&+
    }
}

impl<T: Hash, Src, Dst> Hash for $ty<T, Src, Dst> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        $( self.$field.hash(state); )+
    }
}
    )+};
}

common_impls![Rotation2 { angle }, Rotation3 { a, i, j, k }];

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Rotation2<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Rotation").field(&self.angle.0).finish()
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for Rotation3<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { a, i, j, k, .. } = self;
        let name = if f.alternate() { "Rotation" } else { "Quat" };
        write!(f, "{name}({a:?} + {i:?}*i + {j:?}*j + {k:?}*k)")
    }
}

impl<T, Src, Dst> Rotation2<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub const fn new(angle: Angle<T>) -> Self {
        Self {
            angle,
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Rotation3<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub const fn new_unchecked(a: T, i: T, j: T, k: T) -> Self {
        Self {
            a,
            i,
            j,
            k,
            _unit: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn vector_part(&self) -> Vector3<T, UnknownUnit>
    where
        T: Copy,
    {
        Vector3::new(self.i, self.j, self.k)
    }
}

impl<T: Real, Src, Dst> Rotation3<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub fn new(a: T, i: T, j: T, k: T) -> Self {
        Self::new_unchecked(a, i, j, k).normalize()
    }

    #[must_use]
    pub fn from_euler_angles(roll: Angle<T>, pitch: Angle<T>, yaw: Angle<T>) -> Self {
        let half = T::one() / (T::one() + T::one());

        let (sy, cy) = Real::sin_cos(half * yaw.radians());
        let (sp, cp) = Real::sin_cos(half * pitch.radians());
        let (sr, cr) = Real::sin_cos(half * roll.radians());

        Self::new_unchecked(
            cy * cr * cp + sy * sr * sp,
            cy * sr * cp - sy * cr * sp,
            cy * cr * sp + sy * sr * cp,
            sy * cr * cp - cy * sr * sp,
        )
    }

    pub fn around_axis(axis: Vector3<T, Src>, angle: Angle<T>) -> Self {
        let axis = axis.normalize();
        let two = T::one() + T::one();
        let (sin, cos) = (angle / two).radians().sin_cos();
        Self::new_unchecked(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }

    #[inline]
    #[must_use]
    pub fn norm(&self) -> T {
        self.norm_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn norm_squared(&self) -> T {
        self.a * self.a + self.i * self.i + self.j * self.j + self.k * self.k
    }

    #[inline]
    #[must_use]
    pub fn normalize(&self) -> Self {
        self.mul(T::one() / self.norm())
    }

    #[inline]
    #[must_use]
    pub fn is_normalized(&self) -> bool
    where
        T: ApproxEq,
    {
        let eps: T = num_traits::NumCast::from(1e-5).unwrap();
        self.norm_squared().approx_eq_eps(&T::one(), &eps)
    }

    #[inline]
    pub fn then<NewDst>(&self, other: &Rotation3<T, Dst, NewDst>) -> Rotation3<T, Src, NewDst>
    where
        T: ApproxEq,
    {
        debug_assert!(self.is_normalized());
        debug_assert!(other.is_normalized());

        let (r1, r2) = (self, other);
        Rotation3::new_unchecked(
            r2.a * r1.a - r2.i * r1.i - r2.j * r1.j - r2.k * r1.k,
            r2.i * r1.a + r2.a * r1.i + r2.j * r1.k - r2.k * r1.j,
            r2.j * r1.a + r2.a * r1.j + r2.k * r1.i - r2.i * r1.k,
            r2.k * r1.a + r2.a * r1.k + r2.i * r1.j - r2.j * r1.i,
        )
    }

    #[inline]
    #[must_use]
    pub fn slerp(&self, other: &Self, t: T) -> Self
    where
        T: ApproxEq,
    {
        debug_assert!(self.is_normalized());
        debug_assert!(other.is_normalized());

        let r1 = *self;
        let mut r2 = *other;

        let mut dot = r1.a * r2.a + r1.i * r2.i + r1.j * r2.j + r1.k * r2.k;

        let one = T::one();

        if dot.approx_eq(&one) {
            return r1.lerp(&r2, t);
        }

        if dot < T::zero() {
            r2 = r2.mul(-one);
            dot = -dot;
        }

        dot = Real::min(dot, one);

        let theta = Real::acos(dot) * t;

        let r3 = r2.sub(r1.mul(dot)).normalize();
        let (sin, cos) = Real::sin_cos(theta);
        r1.mul(cos).add(r3.mul(sin))
    }

    #[inline]
    pub fn lerp(&self, other: &Self, t: T) -> Self {
        let one_minus_t = T::one() - t;
        self.mul(one_minus_t).add(other.mul(t)).normalize()
    }

    #[inline]
    fn add(&self, other: Self) -> Self {
        Self::new_unchecked(
            self.a + other.a,
            self.i + other.i,
            self.j + other.j,
            self.k + other.k,
        )
    }

    #[inline]
    fn sub(&self, other: Self) -> Self {
        Self::new_unchecked(
            self.a - other.a,
            self.i - other.i,
            self.j - other.j,
            self.k - other.k,
        )
    }

    #[inline]
    fn mul(&self, factor: T) -> Self {
        Self::new_unchecked(
            self.a * factor,
            self.i * factor,
            self.j * factor,
            self.k * factor,
        )
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Rotation2<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + Neg<Output = T>,
{
    type Inverse = Rotation2<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Self::zero()
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.angle == Angle::zero()
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        Rotation2::new(-self.angle)
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Rotation3<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + One + Neg<Output = T>,
{
    type Inverse = Rotation3<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Self::new_unchecked(T::one(), T::zero(), T::zero(), T::zero())
    }

    #[inline]
    fn is_identity(&self) -> bool {
        let zero = T::zero();
        self.a == T::one() && self.i == zero && self.j == zero && self.k == zero
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        Rotation3::new_unchecked(self.a, -self.i, -self.j, -self.k)
    }
}

impl<T: Real, Src, Dst> Transform<Point2<T, Src>> for Rotation2<T, Src, Dst> {
    type Output = Point2<T, Dst>;

    #[inline]
    fn transform(&self, p: Point2<T, Src>) -> Self::Output {
        let (sin, cos) = Real::sin_cos(self.angle.0);
        Point2::new(p.x * cos - p.y * sin, p.y * cos + p.x * sin)
    }
}

impl<T: Real, Src, Dst> Transform<Vector2<T, Src>> for Rotation2<T, Src, Dst> {
    type Output = Vector2<T, Dst>;

    #[inline]
    fn transform(&self, v: Vector2<T, Src>) -> Self::Output {
        Transform::transform(self, v.to_point()).to_vector()
    }
}

impl<T: Real, Src, Dst> Transform<Vector2<T, Normal<Src>>> for Rotation2<T, Src, Dst> {
    type Output = Vector2<T, Normal<Dst>>;

    #[inline]
    fn transform(&self, n: Vector2<T, Normal<Src>>) -> Self::Output {
        Transform::transform(self, n.to_vector()).to_normal()
    }
}

impl<T: Real, Src, Dst> Transform<Box2<T, Src>> for Rotation2<T, Src, Dst> {
    type Output = Box2<T, Dst>;

    #[inline]
    fn transform(&self, b: Box2<T, Src>) -> Self::Output {
        Box2::new(
            Transform::transform(self, b.min),
            Transform::transform(self, b.max),
        )
    }
}

impl<T, Src, Dst> Transform<Size2<T, Src>> for Rotation2<T, Src, Dst> {
    type Output = Size2<T, Dst>;

    #[inline]
    fn transform(&self, s: Size2<T, Src>) -> Self::Output {
        Size2::new(s.x, s.y)
    }
}

impl<T: Real, Src, Dst> Transform<Point3<T, Src>> for Rotation3<T, Src, Dst> {
    type Output = Point3<T, Dst>;

    #[inline]
    fn transform(&self, p: Point3<T, Src>) -> Self::Output {
        let two = T::one() + T::one();
        let cross = self.vector_part().cross(p.to_vector().erase_unit()) * two;
        Point3::new(
            p.x + self.a * cross.x + self.j * cross.z - self.k * cross.y,
            p.y + self.a * cross.y + self.k * cross.x - self.i * cross.z,
            p.z + self.a * cross.z + self.i * cross.y - self.j * cross.x,
        )
    }
}

impl<T: Real, Src, Dst> Transform<Vector3<T, Src>> for Rotation3<T, Src, Dst> {
    type Output = Vector3<T, Dst>;

    #[inline]
    fn transform(&self, v: Vector3<T, Src>) -> Self::Output {
        Transform::transform(self, v.to_point()).to_vector()
    }
}

impl<T: Real, Src, Dst> Transform<Vector3<T, Normal<Src>>> for Rotation3<T, Src, Dst> {
    type Output = Vector3<T, Normal<Dst>>;

    #[inline]
    fn transform(&self, n: Vector3<T, Normal<Src>>) -> Self::Output {
        Transform::transform(self, n.to_vector()).to_normal()
    }
}

impl<T: Real, Src, Dst> Transform<Box3<T, Src>> for Rotation3<T, Src, Dst> {
    type Output = Box3<T, Dst>;

    #[inline]
    fn transform(&self, b: Box3<T, Src>) -> Self::Output {
        Box3::new(
            Transform::transform(self, b.min),
            Transform::transform(self, b.max),
        )
    }
}

impl<T, Src, Dst> Transform<Size3<T, Src>> for Rotation3<T, Src, Dst> {
    type Output = Size3<T, Dst>;

    #[inline]
    fn transform(&self, s: Size3<T, Src>) -> Self::Output {
        Size3::new(s.x, s.y, s.z)
    }
}

impl<T: Zero, Src, Dst> Zero for Rotation2<T, Src, Dst> {
    #[inline]
    fn zero() -> Self {
        Self::new(Angle::zero())
    }
}

impl<T: ApproxEq, Src, Dst> ApproxEq<T> for Rotation2<T, Src, Dst> {
    #[inline]
    fn epsilon() -> T {
        T::epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        Angle::approx_eq_eps(&self.angle, &other.angle, eps)
    }
}

impl<T, Src, Dst> ApproxEq<T> for Rotation3<T, Src, Dst>
where
    T: Copy + ApproxEq + Neg<Output = T>,
{
    #[inline]
    fn epsilon() -> T {
        T::epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        (self.a.approx_eq_eps(&other.a, eps)
            && self.i.approx_eq_eps(&other.i, eps)
            && self.j.approx_eq_eps(&other.j, eps)
            && self.k.approx_eq_eps(&other.k, eps))
            || (self.a.approx_eq_eps(&-other.a, eps)
                && self.i.approx_eq_eps(&-other.i, eps)
                && self.j.approx_eq_eps(&-other.j, eps)
                && self.k.approx_eq_eps(&-other.k, eps))
    }
}
