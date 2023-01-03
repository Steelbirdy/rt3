use crate::core::{
    geometry::{transform::*, *},
    num::*,
};
use num_traits::NumCast;
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub struct Point2<T, U> {
    pub x: T,
    pub y: T,
    _unit: PhantomData<U>,
}

pub struct Point3<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    _unit: PhantomData<U>,
}

impl<T: Default, U> Default for Point2<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Default, U> Default for Point3<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Point2<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Point3<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl<T: Copy, U> Copy for Point2<T, U> {}

impl<T: Copy, U> Copy for Point3<T, U> {}

impl<T: Clone, U> Clone for Point2<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Clone, U> Clone for Point3<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Eq, U> Eq for Point2<T, U> {}

impl<T: Eq, U> Eq for Point3<T, U> {}

impl<T: PartialEq, U> PartialEq for Point2<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialEq, U> PartialEq for Point3<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Hash, U> Hash for Point2<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Hash, U> Hash for Point3<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T: ApproxEq, U> ApproxEq for Point2<T, U> {
    fn epsilon() -> Self {
        Self::new(T::epsilon(), T::epsilon())
    }

    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x) && self.y.approx_eq_eps(&other.y, &eps.y)
    }
}

impl<T: ApproxEq, U> ApproxEq for Point3<T, U> {
    fn epsilon() -> Self {
        Self::new(T::epsilon(), T::epsilon(), T::epsilon())
    }

    fn approx_eq_eps(&self, other: &Self, eps: &Self) -> bool {
        self.x.approx_eq_eps(&other.x, &eps.x)
            && self.y.approx_eq_eps(&other.y, &eps.y)
            && self.z.approx_eq_eps(&other.z, &eps.z)
    }
}

impl<T, U> From<[T; 2]> for Point2<T, U> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<(T, T)> for Point2<T, U> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<(T, T, T)> for Point3<T, U> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<[T; 3]> for Point3<T, U> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<Point2<T, U>> for [T; 2] {
    fn from(p: Point2<T, U>) -> Self {
        [p.x, p.y]
    }
}

impl<T, U> From<Point3<T, U>> for [T; 3] {
    fn from(p: Point3<T, U>) -> Self {
        [p.x, p.y, p.z]
    }
}

impl<T, U> From<Point2<T, U>> for (T, T) {
    fn from(p: Point2<T, U>) -> Self {
        (p.x, p.y)
    }
}

impl<T, U> From<Point3<T, U>> for (T, T, T) {
    fn from(p: Point3<T, U>) -> Self {
        (p.x, p.y, p.z)
    }
}

impl<T, U> Point2<T, U> {
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
    pub fn origin() -> Self
    where
        T: Zero,
    {
        Self::new(T::zero(), T::zero())
    }

    #[inline]
    #[must_use]
    pub fn erase_unit(self) -> Point2<T, UnknownUnit> {
        Point2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_vector(self) -> Vector2<T, U> {
        Vector2::new(self.x, self.y)
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
    pub fn is_finite(self) -> bool
    where
        T: num_traits::Float,
    {
        self.x.is_finite() && self.y.is_finite()
    }

    #[inline]
    #[must_use]
    pub fn extend(self, z: T) -> Point3<T, U> {
        Point3::new(self.x, self.y, z)
    }
}

impl<T, U> Point3<T, U> {
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
    pub fn origin() -> Self
    where
        T: Zero,
    {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    #[inline]
    #[must_use]
    pub fn erase_unit(self) -> Point3<T, UnknownUnit> {
        Point3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_vector(self) -> Vector3<T, U> {
        Vector3::new(self.x, self.y, self.z)
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
    pub fn is_finite(self) -> bool
    where
        T: num_traits::Float,
    {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl<T: PartialOrd, U> Point2<T, U> {
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

impl<T: PartialOrd, U> Point3<T, U> {
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

impl<T: NumCast, U> Cast for Point2<T, U> {
    type Output<NewT: NumCast> = Point2<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .map(|(x, y)| Point2::new(x, y))
    }
}

impl<T, U> ToPrimitive for Point2<T, U> where Self: Cast {}

impl<T: NumCast, U> Cast for Point3<T, U> {
    type Output<NewT: NumCast> = Point3<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .zip(NumCast::from(self.z))
            .map(|((x, y), z)| Point3::new(x, y, z))
    }
}

impl<T, U> ToPrimitive for Point3<T, U> where Self: Cast {}

scale_trait_impls!(<T: (Copy), U1, U2> for Point2<_, _> { x (.0), y (.0) });

scale_trait_impls!(<T: (Copy), U1, U2> for Point3<_, _> { x (.0), y (.0), z (.0) });

impl<T, U> std::ops::Index<Axis2> for Point2<T, U> {
    type Output = T;

    #[inline]
    fn index(&self, axis: Axis2) -> &Self::Output {
        match axis {
            Axis2::X => &self.x,
            Axis2::Y => &self.y,
        }
    }
}

impl<T, U> std::ops::IndexMut<Axis2> for Point2<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut Self::Output {
        match axis {
            Axis2::X => &mut self.x,
            Axis2::Y => &mut self.y,
        }
    }
}

impl<T, U> std::ops::Index<Axis3> for Point3<T, U> {
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

impl<T, U> std::ops::IndexMut<Axis3> for Point3<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut Self::Output {
        match axis {
            Axis3::X => &mut self.x,
            Axis3::Y => &mut self.y,
            Axis3::Z => &mut self.z,
        }
    }
}

impl<T: Zero, U> Zero for Point2<T, U> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Zero, U> Zero for Point3<T, U> {
    fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Neg, U> Neg for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Point2::new(-self.x, -self.y)
    }
}

impl<T: Neg, U> Neg for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Point3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Copy + Mul, U> Mul<T> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Point2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Copy + Mul, U> Mul<T> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Point2<T, U> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Point3<T, U> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Copy + Div, U> Div<T> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Point2::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Copy + Div, U> Div<T> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Point3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Point2<T, U> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Point3<T, U> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Add, U> Add<Vector2<T, U>> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn add(self, rhs: Vector2<T, U>) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Add, U> Add<Vector3<T, U>> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn add(self, rhs: Vector3<T, U>) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign, U> AddAssign<Vector2<T, U>> for Point2<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2<T, U>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: AddAssign, U> AddAssign<Vector3<T, U>> for Point3<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Vector3<T, U>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub, U> Sub<Vector2<T, U>> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Vector2<T, U>) -> Self::Output {
        Point2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Sub, U> Sub<Vector3<T, U>> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Vector3<T, U>) -> Self::Output {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign, U> SubAssign<Vector2<T, U>> for Point2<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2<T, U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: SubAssign, U> SubAssign<Vector3<T, U>> for Point3<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector3<T, U>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Sub, U> Sub<Point2<T, U>> for Point2<T, U> {
    type Output = Vector2<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Point2<T, U>) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Sub, U> Sub<Point3<T, U>> for Point3<T, U> {
    type Output = Vector3<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Point3<T, U>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Add, U> Add<Size2<T, U>> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn add(self, offset: Size2<T, U>) -> Self::Output {
        Point2::new(self.x + offset.x, self.y + offset.y)
    }
}

impl<T: AddAssign, U> AddAssign<Size2<T, U>> for Point2<T, U> {
    fn add_assign(&mut self, offset: Size2<T, U>) {
        self.x += offset.x;
        self.y += offset.y;
    }
}

impl<T: Sub, U> Sub<Size2<T, U>> for Point2<T, U> {
    type Output = Point2<T::Output, U>;

    #[inline]
    fn sub(self, offset: Size2<T, U>) -> Self::Output {
        Point2::new(self.x - offset.x, self.y - offset.y)
    }
}

impl<T: SubAssign, U> SubAssign<Size2<T, U>> for Point2<T, U> {
    fn sub_assign(&mut self, offset: Size2<T, U>) {
        self.x -= offset.x;
        self.y -= offset.y;
    }
}

impl<T: Add, U> Add<Size3<T, U>> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn add(self, offset: Size3<T, U>) -> Self::Output {
        Point3::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }
}

impl<T: AddAssign, U> AddAssign<Size3<T, U>> for Point3<T, U> {
    fn add_assign(&mut self, offset: Size3<T, U>) {
        self.x += offset.x;
        self.y += offset.y;
        self.z += offset.z;
    }
}

impl<T: Sub, U> Sub<Size3<T, U>> for Point3<T, U> {
    type Output = Point3<T::Output, U>;

    #[inline]
    fn sub(self, offset: Size3<T, U>) -> Self::Output {
        Point3::new(self.x - offset.x, self.y - offset.y, self.z - offset.z)
    }
}

impl<T: SubAssign, U> SubAssign<Size3<T, U>> for Point3<T, U> {
    fn sub_assign(&mut self, offset: Size3<T, U>) {
        self.x -= offset.x;
        self.y -= offset.y;
        self.z -= offset.z;
    }
}

impl<T: Round, U> Round for Point2<T, U> {
    #[inline]
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }
}

impl<T: Round, U> Round for Point3<T, U> {
    #[inline]
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round(), self.z.round())
    }
}

impl<T: Ceil, U> Ceil for Point2<T, U> {
    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }
}

impl<T: Ceil, U> Ceil for Point3<T, U> {
    #[inline]
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
    }
}

impl<T: Floor, U> Floor for Point2<T, U> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Floor, U> Floor for Point3<T, U> {
    #[inline]
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}
