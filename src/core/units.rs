use crate::core::{geometry::transform::Scale, num::*};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
};

macro_rules! impl_ops {
    (for $ty:ident $((+ $neg:ident))?) => {
        impl<T: Zero> Zero for $ty<T> {
            #[inline]
            fn zero() -> Self {
                Self(T::zero())
            }
        }

        impl<T: ApproxEq> ApproxEq<T> for $ty<T> {
            fn epsilon() -> T {
                T::epsilon()
            }

            fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
                T::approx_eq_eps(&self.0, &other.0, eps)
            }
        }

        impl_ops!(@impl Add { fn add }, AddAssign { fn add_assign } for $ty);
        impl_ops!(@impl Sub { fn sub }, SubAssign { fn sub_assign } for $ty);

        impl<T: Zero + Add<Output = T>> std::iter::Sum for $ty<T> {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), Add::add)
            }
        }

        impl<'a, T> std::iter::Sum<&'a Self> for $ty<T>
        where
            T: 'a + Copy + Zero + Add<Output = T>,
        {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.copied().fold(Self::zero(), Add::add)
            }
        }

        impl<T: Mul> Mul<T> for $ty<T> {
            type Output = $ty<T::Output>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $ty(self.0 * rhs)
            }
        }

        impl<T: MulAssign> MulAssign<T> for $ty<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                self.0.mul_assign(rhs);
            }
        }

        impl<T: Div> Div<Self> for $ty<T> {
            type Output = T::Output;

            #[inline]
            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl<T: Div> Div<T> for $ty<T> {
            type Output = $ty<T::Output>;

            #[inline]
            fn div(self, rhs: T) -> Self::Output {
                $ty(self.0 / rhs)
            }
        }

        impl<T: DivAssign> DivAssign<T> for $ty<T> {
            #[inline]
            fn div_assign(&mut self, rhs: T) {
                self.0.div_assign(rhs);
            }
        }

        $(
        impl<T: $neg> $neg for $ty<T> {
            type Output = $ty<T::Output>;

            #[inline]
            fn neg(self) -> Self::Output {
                $ty(-self.0)
            }
        }
        )?
    };
    (@impl $Trait:ident { fn $func:ident }, $TraitAssign:ident { fn $func_assign:ident } for $ty:ident) => {
        impl<T: $Trait> $Trait<Self> for $ty<T> {
            type Output = $ty<T::Output>;

            #[inline]
            fn $func(self, rhs: Self) -> Self::Output {
                $ty(self.0.$func(rhs.0))
            }
        }

        impl<T: $TraitAssign> $TraitAssign<Self> for $ty<T> {
            #[inline]
            fn $func_assign(&mut self, rhs: Self) {
                self.0.$func_assign(rhs.0);
            }
        }
    };
}

pub struct Length<T, U>(pub T, PhantomData<U>);

impl<T: Default, U> Default for Length<T, U> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: fmt::Debug, U> fmt::Debug for Length<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Copy, U> Copy for Length<T, U> {}

impl<T: Clone, U> Clone for Length<T, U> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T: Eq, U> Eq for Length<T, U> {}

impl<T: PartialEq, U> PartialEq for Length<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Ord, U> Ord for Length<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: PartialOrd, U> PartialOrd for Length<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Hash, U> Hash for Length<T, U> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T, U> Length<T, U> {
    #[inline]
    pub const fn new(v: T) -> Self {
        Self(v, PhantomData)
    }

    #[inline]
    pub fn get(self) -> T {
        self.0
    }

    #[inline]
    pub fn lerp(self, other: Self, t: T) -> Self
    where
        T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let one_minus_t = T::one() - t;
        Length::new(one_minus_t * self.0 + t * other.0)
    }

    #[inline]
    pub fn min(self, other: Self) -> Self
    where
        T: PartialOrd,
    {
        min(self, other)
    }

    #[inline]
    pub fn max(self, other: Self) -> Self
    where
        T: PartialOrd,
    {
        max(self, other)
    }
}

scale_trait_impls!(<T, U1, U2> for Length<_, _> { 0 (.0) });

impl<T: Zero, U> Zero for Length<T, U> {
    #[inline]
    fn zero() -> Self {
        Self::new(T::zero())
    }
}

impl<T: One, U> One for Length<T, U> {
    #[inline]
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: Neg, U> Neg for Length<T, U> {
    type Output = Length<T::Output, U>;

    #[inline]
    fn neg(self) -> Self::Output {
        Length::new(-self.0)
    }
}

impl<T: Add, U> Add<Self> for Length<T, U> {
    type Output = Length<T::Output, U>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Length::new(self.0 + rhs.0)
    }
}

impl<T: AddAssign, U> AddAssign<Self> for Length<T, U> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<T: Zero + Add<Output = T>, U> std::iter::Sum for Length<T, U> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Add::add)
    }
}

impl<'a, T, U> std::iter::Sum<&'a Self> for Length<T, U>
where
    T: 'a + Copy + Zero + Add<Output = T>,
    U: 'a,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().fold(Self::zero(), Add::add)
    }
}

impl<T: Sub, U> Sub<Self> for Length<T, U> {
    type Output = Length<T::Output, U>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Length::new(self.0 - rhs.0)
    }
}

impl<T: SubAssign, U> SubAssign<Self> for Length<T, U> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<T: Mul, U> Mul<T> for Length<T, U> {
    type Output = Length<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        Length::new(self.0 * scale)
    }
}

impl<T: MulAssign, U> MulAssign<T> for Length<T, U> {
    #[inline]
    fn mul_assign(&mut self, scale: T) {
        self.0 *= scale;
    }
}

impl<T: Div, U> Div<T> for Length<T, U> {
    type Output = Length<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        Length::new(self.0 / scale)
    }
}

impl<T: DivAssign, U> DivAssign<T> for Length<T, U> {
    #[inline]
    fn div_assign(&mut self, scale: T) {
        self.0 /= scale;
    }
}

impl<T: Div, Src, Dst> Div<Length<T, Src>> for Length<T, Dst> {
    type Output = Scale<T::Output, Src, Dst>;

    #[inline]
    fn div(self, rhs: Length<T, Src>) -> Self::Output {
        Scale::new(self.0 / rhs.0)
    }
}

impl<T: ApproxEq, U> ApproxEq<T> for Length<T, U> {
    fn epsilon() -> T {
        T::epsilon()
    }

    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        T::approx_eq_eps(&self.0, &other.0, eps)
    }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Time<T>(pub T);

impl_ops!(for Time);

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Angle<T>(pub(in crate::core) T);

impl_ops!(for Angle (+ Neg));

impl<T> Angle<T> {
    #[inline]
    #[must_use]
    pub fn from_radians(rad: T) -> Self {
        Self(rad)
    }

    #[inline]
    #[must_use]
    pub fn radians(self) -> T {
        self.0
    }
}

impl<T: Trig> Angle<T> {
    #[inline]
    #[must_use]
    pub fn from_degrees(deg: T) -> Self {
        Self(T::degrees_to_radians(deg))
    }

    #[inline]
    #[must_use]
    pub fn degrees(self) -> T {
        T::radians_to_degrees(self.0)
    }
}
