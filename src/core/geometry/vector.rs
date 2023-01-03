use crate::core::{
    geometry::{transform::*, *},
    num::*,
    units::Angle,
};
use num_traits::NumCast;
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub struct Vector2<T, U> {
    pub x: T,
    pub y: T,
    _unit: PhantomData<U>,
}

pub struct Vector3<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    _unit: PhantomData<U>,
}

impl<T: Zero, U> Zero for Vector2<T, U> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Zero, U> Zero for Vector3<T, U> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: One, U> One for Vector2<T, U> {
    fn one() -> Self {
        Self::new(T::one(), T::one())
    }
}

impl<T: One, U> One for Vector3<T, U> {
    fn one() -> Self {
        Self::new(T::one(), T::one(), T::one())
    }
}

impl<T: Default, U> Default for Vector2<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Default, U> Default for Vector3<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Vector2<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entry(&self.x).entry(&self.y).finish()
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Vector3<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entry(&self.x)
            .entry(&self.y)
            .entry(&self.z)
            .finish()
    }
}

impl<T: Copy, U> Copy for Vector2<T, U> {}

impl<T: Copy, U> Copy for Vector3<T, U> {}

impl<T: Clone, U> Clone for Vector2<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Clone, U> Clone for Vector3<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Eq, U> Eq for Vector2<T, U> {}

impl<T: Eq, U> Eq for Vector3<T, U> {}

impl<T: PartialEq, U> PartialEq for Vector2<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialEq, U> PartialEq for Vector3<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Hash, U> Hash for Vector2<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Hash, U> Hash for Vector3<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: ApproxEq, U> ApproxEq for Vector2<T, U> {
    fn epsilon() -> Self {
        Self::new(T::epsilon(), T::epsilon())
    }

    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
    }
}

impl<T: ApproxEq, U> ApproxEq for Vector3<T, U> {
    fn epsilon() -> Self {
        Self::new(T::epsilon(), T::epsilon(), T::epsilon())
    }

    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x)
            && self.y.approx_eq_eps(&other.y, &eps.y)
            && self.z.approx_eq_eps(&other.z, &eps.z)
    }
}

impl<T, U> From<[T; 2]> for Vector2<T, U> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<(T, T)> for Vector2<T, U> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<(T, T, T)> for Vector3<T, U> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<[T; 3]> for Vector3<T, U> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<Size2<T, U>> for Vector2<T, U> {
    fn from(size: Size2<T, U>) -> Self {
        size.to_vector()
    }
}

impl<T, U> From<Size3<T, U>> for Vector3<T, U> {
    fn from(size: Size3<T, U>) -> Self {
        size.to_vector()
    }
}

impl<T, U1, U2> From<Translation2<T, U1, U2>> for Vector2<T, U1> {
    fn from(t: Translation2<T, U1, U2>) -> Self {
        Self::new(t.x, t.y)
    }
}

impl<T, U1, U2> From<Translation3<T, U1, U2>> for Vector3<T, U1> {
    fn from(t: Translation3<T, U1, U2>) -> Self {
        Self::new(t.x, t.y, t.z)
    }
}

impl<T, U> From<Vector2<T, U>> for [T; 2] {
    fn from(p: Vector2<T, U>) -> Self {
        [p.x, p.y]
    }
}

impl<T, U> From<Vector3<T, U>> for [T; 3] {
    fn from(p: Vector3<T, U>) -> Self {
        [p.x, p.y, p.z]
    }
}

impl<T, U> From<Vector2<T, U>> for (T, T) {
    fn from(p: Vector2<T, U>) -> Self {
        (p.x, p.y)
    }
}

impl<T, U> From<Vector3<T, U>> for (T, T, T) {
    fn from(p: Vector3<T, U>) -> Self {
        (p.x, p.y, p.z)
    }
}

impl<T, U> Vector2<T, U> {
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
            _unit: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v)
    }

    #[inline]
    #[must_use]
    pub fn erase_unit(self) -> Vector2<T, UnknownUnit> {
        Vector2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_point(self) -> Point2<T, U> {
        Point2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_normal(self) -> Vector2<T, Normal<U>> {
        Vector2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_size(self) -> Size2<T, U> {
        Size2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_array(self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use]
    pub fn to_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    #[must_use]
    pub fn abs(self) -> Self
    where
        T: num_traits::Signed,
    {
        Self::new(self.x.abs(), self.y.abs())
    }

    #[inline]
    #[must_use]
    pub fn round(self) -> Self
    where
        T: Round,
    {
        Round::round(self)
    }

    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self
    where
        T: Ceil,
    {
        Ceil::ceil(self)
    }

    #[inline]
    #[must_use]
    pub fn floor(self) -> Self
    where
        T: Floor,
    {
        Floor::floor(self)
    }

    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let one_minus_t = T::one() - t;
        Self::new(
            one_minus_t * self.x + t * other.x,
            one_minus_t * self.y + t * other.y,
        )
    }

    #[inline]
    #[must_use]
    pub fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    #[must_use]
    pub fn cross(self, other: Self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T>,
    {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    #[must_use]
    pub fn component_mul(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }

    #[inline]
    #[must_use]
    pub fn component_div(self, rhs: Self) -> Self
    where
        T: Div<Output = T>,
    {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }

    #[inline]
    #[must_use]
    pub fn angle_between(self, other: Self) -> Angle<T>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Trig,
    {
        Angle::from_radians(Trig::fast_atan2(self.cross(other), self.dot(other)))
    }

    #[inline]
    #[must_use]
    pub fn project_onto(self, onto: Self) -> Self
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        onto * (self.dot(onto) / onto.length_squared())
    }

    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Vector2<T, Normal<U>>) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let two = T::one() + T::one();
        let normal = Self::new(normal.x, normal.y);
        self - normal * two * self.dot(normal)
    }

    #[inline]
    #[must_use]
    pub fn robust_normalize(self) -> Self
    where
        T: num_traits::Float,
    {
        let length = self.length();
        if length.is_infinite() {
            let scaled = self / T::max_value();
            scaled / scaled.length()
        } else {
            self / length
        }
    }

    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool
    where
        T: num_traits::Float,
    {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    #[must_use]
    pub fn extend(self, z: T) -> Vector3<T, U> {
        Vector3::new(self.x, self.y, z)
    }
}

impl<T, U> Vector2<T, Normal<U>> {
    #[inline]
    #[must_use]
    pub fn to_vector(self) -> Vector2<T, U> {
        Vector2::new(self.x, self.y)
    }
}

impl<T: num_traits::real::Real, U> Vector2<T, U> {
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        if len == T::zero() {
            None
        } else {
            Some(self / len)
        }
    }

    #[inline]
    #[must_use]
    pub fn with_length(self, length: T) -> Self {
        self.normalize() * length
    }

    #[inline]
    #[must_use]
    pub fn with_max_length(self, max_length: T) -> Self {
        let square_length = self.length_squared();
        if square_length > max_length * max_length {
            self * (max_length / square_length.sqrt())
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn with_min_length(self, min_length: T) -> Self {
        let square_length = self.length_squared();
        if square_length < min_length * min_length {
            self * (min_length / square_length.sqrt())
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: T, max: T) -> Self {
        debug_assert!(min <= max);
        self.with_min_length(min).with_max_length(max)
    }
}

impl<T, U> Vector3<T, U> {
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

    #[inline]
    #[must_use]
    pub const fn splat(v: T) -> Self
    where
        T: Copy,
    {
        Self::new(v, v, v)
    }

    #[inline]
    #[must_use]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    #[inline]
    #[must_use]
    pub fn erase_unit(self) -> Vector3<T, UnknownUnit> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_point(self) -> Point3<T, U> {
        Point3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_normal(self) -> Vector3<T, Normal<U>> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_size(self) -> Size3<T, U> {
        Size3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_array(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    #[must_use]
    pub fn to_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn length_squared(self) -> T
    where
        T: Copy + Add<Output = T> + Mul<Output = T>,
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    #[must_use]
    pub fn abs(self) -> Self
    where
        T: num_traits::Signed,
    {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    #[inline]
    #[must_use]
    pub fn round(self) -> Self
    where
        T: Round,
    {
        Round::round(self)
    }

    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self
    where
        T: Ceil,
    {
        Ceil::ceil(self)
    }

    #[inline]
    #[must_use]
    pub fn floor(self) -> Self
    where
        T: Floor,
    {
        Floor::floor(self)
    }

    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let one_minus_t = T::one() - t;
        Self::new(
            one_minus_t * self.x + t * other.x,
            one_minus_t * self.y + t * other.y,
            one_minus_t * self.z + t * other.z,
        )
    }

    #[inline]
    #[must_use]
    pub fn dot(self, other: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    #[must_use]
    pub fn cross(self, other: Self) -> Self
    where
        T: Copy + Sub<Output = T> + Mul<Output = T>,
    {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    #[inline]
    #[must_use]
    pub fn component_mul(self, rhs: Self) -> Self
    where
        T: Mul<Output = T>,
    {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }

    #[inline]
    #[must_use]
    pub fn component_div(self, rhs: Self) -> Self
    where
        T: Div<Output = T>,
    {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }

    #[inline]
    #[must_use]
    pub fn angle_between(self, other: Self) -> Angle<T>
    where
        T: num_traits::real::Real + Trig,
    {
        Angle::from_radians(Trig::fast_atan2(
            self.cross(other).length(),
            self.dot(other),
        ))
    }

    #[inline]
    #[must_use]
    pub fn project_onto(self, onto: Self) -> Self
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        onto * (self.dot(onto) / onto.length_squared())
    }

    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Vector3<T, Normal<U>>) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let two = T::one() + T::one();
        let normal = Self::new(normal.x, normal.y, normal.z);
        self - normal * two * self.dot(normal)
    }

    #[inline]
    #[must_use]
    pub fn robust_normalize(self) -> Self
    where
        T: num_traits::Float,
    {
        let length = self.length();
        if length.is_infinite() {
            let scaled = self / T::max_value();
            scaled / scaled.length()
        } else {
            self / length
        }
    }

    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool
    where
        T: num_traits::Float,
    {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<T, U> Vector3<T, Normal<U>> {
    #[inline]
    #[must_use]
    pub fn to_vector(self) -> Vector3<T, U> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn face_towards(self, v: Vector3<T, U>) -> Self
    where
        T: num_traits::real::Real,
    {
        let n = self.to_vector();
        if n.dot(v).is_sign_negative() {
            -self
        } else {
            self
        }
    }
}

impl<T: num_traits::real::Real, U> Vector3<T, U> {
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        if len == T::zero() {
            None
        } else {
            Some(self / len)
        }
    }

    #[inline]
    #[must_use]
    pub fn with_length(self, length: T) -> Self {
        self.normalize() * length
    }

    #[inline]
    #[must_use]
    pub fn with_max_length(self, max_length: T) -> Self {
        let square_length = self.length_squared();
        if square_length > max_length * max_length {
            self * (max_length / square_length.sqrt())
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn with_min_length(self, min_length: T) -> Self {
        let square_length = self.length_squared();
        if square_length < min_length * min_length {
            self * (min_length / square_length.sqrt())
        } else {
            self
        }
    }

    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: T, max: T) -> Self {
        debug_assert!(min <= max);
        self.with_min_length(min).with_max_length(max)
    }
}

impl<T: PartialEq, U> Vector2<T, U> {
    #[inline]
    #[must_use]
    pub fn cmp_eq(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x == other.x,
            y: self.y == other.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_ne(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x != other.x,
            y: self.y != other.y,
        }
    }
}

impl<T: PartialOrd, U> Vector2<T, U> {
    #[inline]
    #[must_use]
    pub fn cmp_lt(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x < other.x,
            y: self.y < other.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_le(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x <= other.x,
            y: self.y <= other.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_gt(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x > other.x,
            y: self.y > other.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_ge(self, other: Self) -> Mask2 {
        Mask2 {
            x: self.x >= other.x,
            y: self.y >= other.y,
        }
    }

    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::new(min(self.x, other.x), min(self.y, other.y))
    }

    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::new(max(self.x, other.x), max(self.y, other.y))
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

impl<T: PartialEq, U> Vector3<T, U> {
    #[inline]
    #[must_use]
    pub fn cmp_eq(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x == other.x,
            y: self.y == other.y,
            z: self.z == other.z,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_ne(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x != other.x,
            y: self.y != other.y,
            z: self.z != other.z,
        }
    }
}

impl<T: PartialOrd, U> Vector3<T, U> {
    #[inline]
    #[must_use]
    pub fn cmp_lt(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x < other.x,
            y: self.y < other.y,
            z: self.z < other.z,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_le(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x <= other.x,
            y: self.y <= other.y,
            z: self.z <= other.z,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_gt(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x > other.x,
            y: self.y > other.y,
            z: self.z > other.z,
        }
    }

    #[inline]
    #[must_use]
    pub fn cmp_ge(self, other: Self) -> Mask3 {
        Mask3 {
            x: self.x >= other.x,
            y: self.y >= other.y,
            z: self.z >= other.z,
        }
    }

    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::new(
            min(self.x, other.x),
            min(self.y, other.y),
            min(self.z, other.z),
        )
    }

    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::new(
            max(self.x, other.x),
            max(self.y, other.y),
            max(self.z, other.z),
        )
    }

    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

impl<T: NumCast, U> Cast for Vector2<T, U> {
    type Output<NewT: NumCast> = Vector2<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .map(|(x, y)| Vector2::new(x, y))
    }
}

impl<T, U> ToPrimitive for Vector2<T, U> where Self: Cast {}

impl<T: NumCast, U> Cast for Vector3<T, U> {
    type Output<NewT: NumCast> = Vector3<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .zip(NumCast::from(self.z))
            .map(|((x, y), z)| Vector3::new(x, y, z))
    }
}

impl<T, U> ToPrimitive for Vector3<T, U> where Self: Cast {}

scale_trait_impls!(<T: (Copy), U1, U2> for Vector2<_, _> { x (.0), y (.0) });

scale_trait_impls!(<T: (Copy), U1, U2> for Vector3<_, _> { x (.0), y (.0), z (.0) });

impl<T, U> std::ops::Index<Axis2> for Vector2<T, U> {
    type Output = T;

    #[inline]
    fn index(&self, axis: Axis2) -> &Self::Output {
        match axis {
            Axis2::X => &self.x,
            Axis2::Y => &self.y,
        }
    }
}

impl<T, U> std::ops::IndexMut<Axis2> for Vector2<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut Self::Output {
        match axis {
            Axis2::X => &mut self.x,
            Axis2::Y => &mut self.y,
        }
    }
}

impl<T, U> std::ops::Index<Axis3> for Vector3<T, U> {
    type Output = T;

    #[inline]
    fn index(&self, axis: Axis3) -> &Self::Output {
        match axis {
            Axis3::X => &self.x,
            Axis3::Y => &self.y,
            Axis3::Z => &self.z,
        }
    }
}

impl<T, U> std::ops::IndexMut<Axis3> for Vector3<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut Self::Output {
        match axis {
            Axis3::X => &mut self.x,
            Axis3::Y => &mut self.y,
            Axis3::Z => &mut self.z,
        }
    }
}

impl<T: Neg, U> Neg for Vector2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}

impl<T: Neg, U> Neg for Vector3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Copy + Mul, U> Mul<T> for Vector2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Copy + Mul, U> Mul<T> for Vector3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Vector2<T, U> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Vector3<T, U> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Copy + Div, U> Div<T> for Vector2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Copy + Div, U> Div<T> for Vector3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Vector2<T, U> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Vector3<T, U> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Add, U> Add<Vector2<T, U>> for Vector2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn add(self, rhs: Vector2<T, U>) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Add, U> Add<Vector3<T, U>> for Vector3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn add(self, rhs: Vector3<T, U>) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign, U> AddAssign<Vector2<T, U>> for Vector2<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<T, U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: AddAssign, U> AddAssign<Vector3<T, U>> for Vector3<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector3<T, U>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Zero + Add<Output = T>, U> std::iter::Sum for Vector2<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T, U> std::iter::Sum<&'a Self> for Vector2<T, U>
where
    T: 'a + Copy + Zero + Add<Output = T>,
    U: 'a,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self::zero(), Add::add)
    }
}

impl<T: Zero + Add<Output = T>, U> std::iter::Sum for Vector3<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T, U> std::iter::Sum<&'a Self> for Vector3<T, U>
where
    T: 'a + Copy + Zero + Add<Output = T>,
    U: 'a,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self::zero(), Add::add)
    }
}

impl<T: Sub, U> Sub<Vector2<T, U>> for Vector2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Vector2<T, U>) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Sub, U> Sub<Vector3<T, U>> for Vector3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Vector3<T, U>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign, U> SubAssign<Vector2<T, U>> for Vector2<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2<T, U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: SubAssign, U> SubAssign<Vector3<T, U>> for Vector3<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector3<T, U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Round, U> Round for Vector2<T, U> {
    #[inline]
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }
}

impl<T: Round, U> Round for Vector3<T, U> {
    #[inline]
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }
}

impl<T: Ceil, U> Ceil for Vector2<T, U> {
    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }
}

impl<T: Ceil, U> Ceil for Vector3<T, U> {
    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl<T: Floor, U> Floor for Vector2<T, U> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Floor, U> Floor for Vector3<T, U> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}
