macro_rules! scale_trait_impls {
    (<$(($($before:tt)+),)? $T:ident $(: ($($T_bounds:tt)+))?, $U1:ident, $U2:ident> for $ty:ident <$(($($ty_before:tt)+),)? _, _ $(, ($($ty_after:tt)+))?> { $($field:tt $(($($field_after:tt)+))?),+ }) => {
        impl <$($($before)+,)? $T: $($($T_bounds)+ +)? Mul, $U1, $U2> Mul<Scale<$T, $U1, $U2>> for $ty<$($($ty_before)+,)? $T, $U1 $($($ty_after)+)?> {
            type Output = $ty<$($($ty_before)+,)? $T::Output, $U2 $($($ty_after)+)?>;

            #[inline]
            fn mul(self, scale: Scale<$T, $U1, $U2>) -> Self::Output {
                $ty::new($(self.$field * scale $($($field_after)+)?),+)
            }
        }

        impl <$($($before)+,)? $T: $($($T_bounds)+ +)? MulAssign, $U1> MulAssign<Scale<$T, $U1, $U1>> for $ty<$($($ty_before)+,)? $T, $U1 $($($ty_after)+)?> {
            #[inline]
            fn mul_assign(&mut self, scale: Scale<$T, $U1, $U1>) {
                $( self.$field *= scale $($($field_after)+)?; )+
            }
        }

        impl <$($($before)+,)? $T: $($($T_bounds)+ +)? One + Div, $U1, $U2> Div<Scale<$T, $U1, $U2>> for $ty<$($($ty_before)+,)? $T, $U2 $($($ty_after)+)?> {
            type Output = $ty<$($($ty_before)+,)? $T::Output, $U1 $($($ty_after)+)?>;

            #[inline]
            fn div(self, scale: Scale<$T, $U1, $U2>) -> Self::Output {
                $ty::new($(self.$field / scale $($($field_after)+)?),+)
            }
        }

        impl <$($($before)+,)? $T: $($($T_bounds)+ +)? DivAssign, $U1> DivAssign<Scale<$T, $U1, $U1>> for $ty<$($($ty_before)+,)? $T, $U1 $($($ty_after)+)?> {
            #[inline]
            fn div_assign(&mut self, scale: Scale<$T, $U1, $U1>) {
                $( self.$field /= scale $($($field_after)+)?; )+
            }
        }
    };
}
