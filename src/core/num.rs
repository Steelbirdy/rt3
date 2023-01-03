use num_traits::NumCast;

#[inline]
#[must_use]
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

#[inline]
#[must_use]
pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x >= y {
        x
    } else {
        y
    }
}

pub trait Trig {
    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn tan(self) -> Self;

    fn fast_atan2(y: Self, x: Self) -> Self;

    fn degrees_to_radians(deg: Self) -> Self;

    fn radians_to_degrees(rad: Self) -> Self;
}

macro_rules! impl_trig {
    ($ty:ident) => {
        impl Trig for $ty {
            #[inline]
            fn sin(self) -> $ty {
                num_traits::Float::sin(self)
            }

            #[inline]
            fn cos(self) -> $ty {
                num_traits::Float::cos(self)
            }

            #[inline]
            fn tan(self) -> $ty {
                num_traits::Float::tan(self)
            }

            #[inline]
            fn fast_atan2(y: $ty, x: $ty) -> $ty {
                // See https://math.stackexchange.com/questions/1098487/atan2-faster-approximation#1105038
                use core::$ty::consts;
                let x_abs = num_traits::Float::abs(x);
                let y_abs = num_traits::Float::abs(y);
                let a = x_abs.min(y_abs) / x_abs.max(y_abs);
                let s = a * a;
                let mut result =
                    ((-0.046_496_474_9 * s + 0.159_314_22) * s - 0.327_622_764) * s * a + a;
                if y_abs > x_abs {
                    result = consts::FRAC_PI_2 - result;
                }
                if x < 0.0 {
                    result = consts::PI - result;
                }
                if y < 0.0 {
                    result = -result;
                }
                result
            }

            #[inline]
            fn degrees_to_radians(deg: $ty) -> $ty {
                deg.to_radians()
            }

            #[inline]
            fn radians_to_degrees(rad: $ty) -> $ty {
                rad.to_degrees()
            }
        }
    };
}

impl_trig!(f32);
impl_trig!(f64);

pub trait ApproxEq<T = Self> {
    #[must_use]
    fn epsilon() -> T;

    #[must_use]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool;

    #[inline]
    #[must_use]
    fn approx_eq(&self, other: &Self) -> bool {
        self.approx_eq_eps(other, &Self::epsilon())
    }
}

impl<Eps, T: ApproxEq<Eps>, const N: usize> ApproxEq<Eps> for [T; N] {
    #[inline]
    fn epsilon() -> Eps {
        T::epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &Eps) -> bool {
        self.iter()
            .zip(other)
            .all(|(x1, x2)| x1.approx_eq_eps(x2, eps))
    }
}

pub trait Cast: Sized {
    type Output<NewT: NumCast>;

    #[must_use]
    fn cast<NewT: NumCast>(self) -> Self::Output<NewT> {
        self.try_cast().unwrap()
    }

    #[must_use]
    fn try_cast<NewT: NumCast>(self) -> Option<Self::Output<NewT>>;
}

pub trait ToPrimitive: Cast {
    #[inline]
    fn to_f32(self) -> Self::Output<f32> {
        self.cast()
    }

    #[inline]
    fn to_f64(self) -> Self::Output<f64> {
        self.cast()
    }

    #[inline]
    fn to_usize(self) -> Self::Output<usize> {
        self.cast()
    }

    #[inline]
    fn to_u32(self) -> Self::Output<u32> {
        self.cast()
    }

    #[inline]
    fn to_i32(self) -> Self::Output<i32> {
        self.cast()
    }

    #[inline]
    fn to_i64(self) -> Self::Output<i64> {
        self.cast()
    }
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Round {
    fn round(self) -> Self;
}

pub trait Ceil {
    fn ceil(self) -> Self;
}

pub trait Floor {
    fn floor(self) -> Self;
}

impl<T: num_traits::Zero> Zero for T {
    #[inline]
    fn zero() -> Self {
        num_traits::Zero::zero()
    }
}

impl<T: num_traits::One> One for T {
    #[inline]
    fn one() -> Self {
        num_traits::One::one()
    }
}

macro_rules! num_int {
    ($($ty:ident)+) => {$(
        impl Round for $ty {
            #[inline]
            fn round(self) -> Self {
                self
            }
        }
        impl Ceil for $ty {
            #[inline]
            fn ceil(self) -> Self {
                self
            }
        }
        impl Floor for $ty {
            #[inline]
            fn floor(self) -> Self {
                self
            }
        }
    )+};
}

macro_rules! num_float {
    ($($ty:ident)+) => {$(
        impl Round for $ty {
            #[inline]
            fn round(self) -> Self {
                (self + 0.5).floor()
            }
        }
        impl Ceil for $ty {
            #[inline]
            fn ceil(self) -> Self {
                num_traits::Float::ceil(self)
            }
        }
        impl Floor for $ty {
            #[inline]
            fn floor(self) -> Self {
                num_traits::Float::floor(self)
            }
        }

        impl ApproxEq<$ty> for $ty {
            fn epsilon() -> $ty {
                1e-6
            }

            fn approx_eq_eps(&self, other: &$ty, eps: &$ty) -> bool {
                num_traits::Float::abs(*self - *other) < *eps
            }
        }
    )+};
}

num_int![i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize];
num_float![f32 f64];
