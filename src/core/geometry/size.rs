use crate::core::{
    geometry::{transform::*, Axis2, Axis3, Mask2, Mask3, Normal, Vector2, Vector3},
    num::*,
    units::Length,
};
use num_traits::NumCast;
use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
};

pub struct Size2<T, U> {
    pub x: T,
    pub y: T,
    _unit: PhantomData<U>,
}

pub struct Size3<T, U> {
    pub x: T,
    pub y: T,
    pub z: T,
    _unit: PhantomData<U>,
}

impl<T: Zero, U> Zero for Size2<T, U> {
    #[inline]
    fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Zero, U> Zero for Size3<T, U> {
    #[inline]
    fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Default, U> Default for Size2<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Default, U> Default for Size3<T, U> {
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Size2<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.x, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.y, f)
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Size3<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.x, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.y, f)?;
        write!(f, "x")?;
        fmt::Debug::fmt(&self.z, f)
    }
}

impl<T: Copy, U> Copy for Size2<T, U> {}

impl<T: Copy, U> Copy for Size3<T, U> {}

impl<T: Clone, U> Clone for Size2<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone())
    }
}

impl<T: Clone, U> Clone for Size3<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.x.clone(), self.y.clone(), self.z.clone())
    }
}

impl<T: Eq, U> Eq for Size2<T, U> {}

impl<T: Eq, U> Eq for Size3<T, U> {}

impl<T: PartialEq, U> PartialEq for Size2<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialEq, U> PartialEq for Size3<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Hash, U> Hash for Size2<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Hash, U> Hash for Size3<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl<T, U> Size2<T, U> {
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
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>) -> Self {
        Self::new(x.0, y.0)
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
    pub fn to_vector(self) -> Vector2<T, U> {
        Vector2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn to_normal(self) -> Vector2<T, Normal<U>> {
        Vector2::new(self.x, self.y)
    }

    #[inline]
    #[must_use]
    pub fn area(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.x * self.y
    }

    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let one_minus_t = T::one() - t;
        self * one_minus_t + other * t
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
    pub fn is_positive(self) -> bool
    where
        T: num_traits::Signed,
    {
        self.x.is_positive() && self.y.is_positive()
    }

    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool
    where
        T: num_traits::Float,
    {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl<T, U> Size3<T, U> {
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
    pub fn from_lengths(x: Length<T, U>, y: Length<T, U>, z: Length<T, U>) -> Self {
        Self::new(x.0, y.0, z.0)
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
    pub fn to_vector(self) -> Vector3<T, U> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn to_normal(self) -> Vector3<T, Normal<U>> {
        Vector3::new(self.x, self.y, self.z)
    }

    #[inline]
    #[must_use]
    pub fn volume(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.x * self.y * self.z
    }

    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let one_minus_t = T::one() - t;
        self * one_minus_t + other * t
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
    pub fn is_positive(self) -> bool
    where
        T: num_traits::Signed,
    {
        self.x.is_positive() && self.y.is_positive() && self.z.is_positive()
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

impl<T: PartialOrd, U> Size2<T, U> {
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
        self.min(max).max(min)
    }

    #[inline]
    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        self.cmp_ge(other).all()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(self) -> bool
    where
        T: Zero,
    {
        let zero = T::zero();
        !(self.x > zero && self.y > zero)
    }

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
}

impl<T: PartialOrd, U> Size3<T, U> {
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
        self.min(max).max(min)
    }

    #[inline]
    #[must_use]
    pub fn contains(self, other: Self) -> bool {
        self.cmp_ge(other).all()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(self) -> bool
    where
        T: Zero,
    {
        let zero = T::zero();
        !(self.x > zero && self.y > zero && self.z > zero)
    }

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
}

impl<T: Neg, U> Neg for Size2<T, U> {
    type Output = Size2<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size2::new(-self.x, -self.y)
    }
}

impl<T: Neg, U> Neg for Size3<T, U> {
    type Output = Size3<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Size3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Add, U> Add<Self> for Size2<T, U> {
    type Output = Size2<T::Output, U>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Size2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: AddAssign, U> AddAssign<Self> for Size2<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Zero + Add<Output = T>, U> std::iter::Sum for Size2<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T, U> std::iter::Sum<&'a Self> for Size2<T, U>
where
    T: 'a + Copy + Zero + Add<Output = T>,
    U: 'a,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self::zero(), Add::add)
    }
}

impl<T: Sub, U> Sub<Self> for Size2<T, U> {
    type Output = Size2<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Size2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: SubAssign, U> SubAssign<Self> for Size2<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Add, U> Add<Self> for Size3<T, U> {
    type Output = Size3<T::Output, U>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Size3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: AddAssign, U> AddAssign<Self> for Size3<T, U> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Zero + Add<Output = T>, U> std::iter::Sum for Size3<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T, U> std::iter::Sum<&'a Self> for Size3<T, U>
where
    T: 'a + Copy + Zero + Add<Output = T>,
    U: 'a,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self::zero(), Add::add)
    }
}

impl<T: Sub, U> Sub<Self> for Size3<T, U> {
    type Output = Size3<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Size3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: SubAssign, U> SubAssign<Self> for Size3<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Copy + Mul, U> Mul<T> for Size2<T, U> {
    type Output = Size2<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Size2::new(self.x * scale, self.y * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Size2<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.x *= scale;
        self.y *= scale;
    }
}

impl<T: Copy + Div, U> Div<T> for Size2<T, U> {
    type Output = Size2<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Size2::new(self.x / scale, self.y / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Size2<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.x /= scale;
        self.y /= scale;
    }
}

impl<T: Copy + Mul, U> Mul<T> for Size3<T, U> {
    type Output = Size3<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Size3::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl<T: Copy + MulAssign, U> MulAssign<T> for Size3<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

impl<T: Copy + Div, U> Div<T> for Size3<T, U> {
    type Output = Size3<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Size3::new(self.x / scale, self.y / scale, self.z / scale)
    }
}

impl<T: Copy + DivAssign, U> DivAssign<T> for Size3<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
    }
}

scale_trait_impls!(<T: (Copy), U1, U2> for Size2<_, _> { x (.0), y (.0) });

scale_trait_impls!(<T: (Copy), U1, U2> for Size3<_, _> { x (.0), y (.0), z (.0) });

impl<T, U> Index<Axis2> for Size2<T, U> {
    type Output = T;

    #[inline]
    fn index(&self, axis: Axis2) -> &Self::Output {
        match axis {
            Axis2::X => &self.x,
            Axis2::Y => &self.y,
        }
    }
}

impl<T, U> IndexMut<Axis2> for Size2<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis2) -> &mut Self::Output {
        match axis {
            Axis2::X => &mut self.x,
            Axis2::Y => &mut self.y,
        }
    }
}

impl<T, U> Index<Axis3> for Size3<T, U> {
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

impl<T, U> IndexMut<Axis3> for Size3<T, U> {
    #[inline]
    fn index_mut(&mut self, axis: Axis3) -> &mut Self::Output {
        match axis {
            Axis3::X => &mut self.x,
            Axis3::Y => &mut self.y,
            Axis3::Z => &mut self.z,
        }
    }
}

impl<T, U> From<Size2<T, U>> for [T; 2] {
    fn from(size: Size2<T, U>) -> Self {
        size.to_array()
    }
}

impl<T, U> From<Size2<T, U>> for (T, T) {
    fn from(size: Size2<T, U>) -> Self {
        size.to_tuple()
    }
}

impl<T, U> From<[T; 2]> for Size2<T, U> {
    fn from([x, y]: [T; 2]) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<(T, T)> for Size2<T, U> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}

impl<T, U> From<Vector2<T, U>> for Size2<T, U> {
    fn from(vector: Vector2<T, U>) -> Self {
        vector.to_size()
    }
}

impl<T, U> From<Size3<T, U>> for [T; 3] {
    fn from(size: Size3<T, U>) -> Self {
        size.to_array()
    }
}

impl<T, U> From<Size3<T, U>> for (T, T, T) {
    fn from(size: Size3<T, U>) -> Self {
        size.to_tuple()
    }
}

impl<T, U> From<[T; 3]> for Size3<T, U> {
    fn from([x, y, z]: [T; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<(T, T, T)> for Size3<T, U> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self::new(x, y, z)
    }
}

impl<T, U> From<Vector3<T, U>> for Size3<T, U> {
    fn from(vector: Vector3<T, U>) -> Self {
        vector.to_size()
    }
}

impl<T: Floor, U> Floor for Size2<T, U> {
    fn floor(self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }
}

impl<T: Ceil, U> Ceil for Size2<T, U> {
    fn ceil(self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }
}

impl<T: Round, U> Round for Size2<T, U> {
    fn round(self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }
}

impl<T: NumCast, U> Cast for Size2<T, U> {
    type Output<NewT: NumCast> = Size2<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .map(|(x, y)| Size2::new(x, y))
    }
}

impl<T, U> ToPrimitive for Size2<T, U> where Self: Cast {}

impl<T: NumCast, U> Cast for Size3<T, U> {
    type Output<NewT: NumCast> = Size3<NewT, U>;

    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>> {
        NumCast::from(self.x)
            .zip(NumCast::from(self.y))
            .zip(NumCast::from(self.z))
            .map(|((x, y), z)| Size3::new(x, y, z))
    }
}

impl<T, U> ToPrimitive for Size3<T, U> where Self: Cast {}
