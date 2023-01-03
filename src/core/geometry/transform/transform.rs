use crate::core::{
    geometry::{transform::*, *},
    num::*,
    units::{Angle, Length},
};
use num_traits::{NumOps, real::Real};
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::*,
    fmt,
};

pub struct Transform2<T, Src, Dst> {
    mat: [[T; 2]; 3],
    mat_inv: [[T; 2]; 3],
    _unit: PhantomData<(Src, Dst)>,
}

pub struct Transform3<T, Src, Dst> {
    mat: [[T; 4]; 4],
    mat_inv: [[T; 4]; 4],
    _unit: PhantomData<(Src, Dst)>,
}

macro_rules! common_impls {
    ($($ty:ident),+) => {$(
impl<T: Copy, Src, Dst> Copy for $ty<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for $ty<T, Src, Dst> {
    fn clone(&self) -> Self {
        Self::new_raw(self.mat.clone(), self.mat_inv.clone())
    }
}

impl<T: Eq, Src, Dst> Eq for $ty<T, Src, Dst> {}

impl<T: PartialEq, Src, Dst> PartialEq for $ty<T, Src, Dst> {
    fn eq(&self, other: &Self) -> bool {
        self.mat == other.mat
    }
}

impl<T: Hash, Src, Dst> Hash for $ty<T, Src, Dst> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mat.hash(state);
    }
}
    )+};
}

common_impls![Transform2, Transform3];

impl<T, Src, Dst> Transform2<T, Src, Dst> {
    #[inline]
    #[must_use]
    const fn new_raw(mat: [[T; 2]; 3], mat_inv: [[T; 2]; 3]) -> Self {
        Self {
            mat,
            mat_inv,
            _unit: PhantomData,
        }
    }

    #[inline]
    pub fn identity() -> Self
        where
            T: Copy + Zero + One,
    {
        let (o, l) = (T::zero(), T::one());
        let mat = [
            [l, o],
            [o, l],
            [o, o],
        ];
        Self::new_raw(mat, mat)
    }

    #[inline]
    #[must_use]
    pub fn is_identity(&self) -> bool
    where
        T: Copy + PartialEq + Zero + One,
    {
        self.mat == Self::identity().mat
    }

    #[inline]
    #[must_use]
    pub const fn erase_unit(&self) -> Transform2<T, UnknownUnit, UnknownUnit>
    where
        T: Copy,
    {
        let &Self { mat, mat_inv, .. } = self;
        Transform2 {
            mat,
            mat_inv,
            _unit: PhantomData,
        }
    }
}

impl<T, Src, Dst> Transform3<T, Src, Dst> {
    #[inline]
    #[must_use]
    const fn new_raw(mat: [[T; 4]; 4], mat_inv: [[T; 4]; 4]) -> Self {
        Self {
            mat,
            mat_inv,
            _unit: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn identity() -> Self
    where
        T: Copy + Zero + One,
    {
        let (o, l) = (T::zero(), T::one());
        let mat = [
            [l, o, o, o],
            [o, l, o, o],
            [o, o, l, o],
            [o, o, o, l],
        ];
        Self::new_raw(mat, mat)
    }

    #[inline]
    #[must_use]
    pub fn is_identity(&self) -> bool
    where
        T: Copy + PartialEq + Zero + One,
    {
        self.mat == Self::identity().mat
    }

    #[inline]
    #[must_use]
    pub const fn erase_unit(&self) -> Transform3<T, UnknownUnit, UnknownUnit>
    where
        T: Copy,
    {
        let &Self { mat, mat_inv, .. } = self;
        Transform3 {
            mat,
            mat_inv,
            _unit: PhantomData,
        }
    }
}

impl<T: Copy + Zero + One + NumOps, Src, Dst> Transform2<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub fn new(mat: [[T; 2]; 3]) -> Self
    where
        T: PartialEq,
    {
        Self::try_new(mat).expect("the given transform is not invertible")
    }

    #[must_use]
    #[rustfmt::skip]
    pub fn try_new(mat: [[T; 2]; 3]) -> Option<Self>
    where
        T: PartialEq,
    {
        let [[m11, m12], [m21, m22], [m31, m32]] = mat;

        let det = m11 * m22 - m21 * m12;
        let o = T::zero();
        if det == o {
            return None;
        }
        let inv_det = T::one() / det;

        let mat_inv = [
            [inv_det * m22, inv_det * (o - m12)],
            [inv_det * (o - m21), inv_det * m11],
            [inv_det * (m21 * m32 - m22 * m31), inv_det * (m12 * m31 - m11 * m32)],
        ];

        Some(Self::new_raw(mat, mat_inv))
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn translation(v: Vector2<T, Src>) -> Self {
        let o = T::zero();
        let l = T::one();
        let [x, y] = v.to_array();
        Self::new_raw(
            [[l, o], [o, l], [x, y]],
            [[l, o], [o, l], [o-x, o-y]],
        )
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn rotation(theta: Angle<T>) -> Self
    where
        T: Trig,
    {
        let o = T::zero();
        let c = theta.radians().cos();
        let s = theta.radians().sin();
        Self::new_raw(
            [[c, s], [o-s, c], [o, o]],
            [[c, o-s], [s, c], [o, o]],
        )
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn scale(x: Scale<T, Src, Dst>, y: Scale<T, Src, Dst>) -> Self {
        let o = T::zero();
        let l = T::one();
        let (x, y) = (x.get(), y.get());
        Self::new_raw(
            [[x, o], [o, y], [o, o]],
            [[l/x, o], [o, l/y], [o, o]],
        )
    }

    #[inline]
    #[must_use]
    pub fn determinant(&self) -> T {
        let [[m11, m12], [m21, m22], _] = self.mat;
        m11 * m22 - m12 * m21
    }

    #[inline]
    #[must_use]
    pub fn is_invertible(&self) -> bool
    where
        T: PartialEq,
    {
        self.determinant() != T::zero()
    }
}

impl<T: Copy + Zero + One + NumOps, Src, Dst> Transform3<T, Src, Dst> {
    #[inline]
    #[must_use]
    pub fn new(mat: [[T; 4]; 4]) -> Self
    where
        T: PartialEq,
    {
        Self::try_new(mat).expect("the given transform is not invertible")
    }

    #[inline]
    #[must_use]
    pub fn try_new(mat: [[T; 4]; 4]) -> Option<Self>
    where
        T: PartialEq,
    {
        let mat_inv = Self::mat_inverse(mat)?;
        Some(Self::new_raw(mat, mat_inv))
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn translation(v: Vector3<T, Src>) -> Self {
        let o = T::zero();
        let l = T::one();
        let [x, y, z] = v.to_array();
        Self::new_raw(
            [[l, o, o, o],
             [o, l, o, o],
             [o, o, l, o],
             [x, y, z, l]],
            [[  l,   o,   o, o],
             [  o,   l,   o, o],
             [  o,   o,   l, o],
             [o-x, o-y, o-z, l]],
        )
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn rotation(axis: Vector3<T, Src>, theta: Angle<T>) -> Self
    where
        T: Trig,
    {
        let (o, l) = (Zero::zero(), One::one());
        let two = l + l;
        let [x, y, z] = axis.to_array();
        let (xx, yy, zz) = (x*x, y*y, z*z);

        let half_theta = theta.0 / two;
        let (sin, cos) = (half_theta.sin(), half_theta.cos());

        let sc = sin * cos;
        let ss = sin * sin;

        let mat = [
            [   l - two * (yy + zz) * ss, two * (x * y * ss + z * sc), two * (x * z * ss - y * sc), o],
            [two * (x * y * ss - z * sc),    l - two * (xx + zz) * ss, two * (y * z * ss + x * sc), o],
            [two * (x * z * ss + y * sc), two * (y * z * ss - x * sc),    l - two * (xx + yy) * ss, o],
            [                           o,                          o,                           o, l],
        ];
        Self::new_raw(
            mat,
            Self::mat_transpose(mat),
        )
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn scale(x: Scale<T, Src, Dst>, y: Scale<T, Src, Dst>, z: Scale<T, Src, Dst>) -> Self {
        let o = T::zero();
        let l = T::one();
        let (x, y, z) = (x.get(), y.get(), z.get());
        Self::new_raw(
            [
                [x, o, o, o],
                [o, y, o, o],
                [o, o, z, o],
                [o, o, o, l],
            ],
            [
                [l/x,   o,   o, o],
                [  o, l/y,   o, o],
                [  o,   o, l/z, o],
                [  o,   o,   o, l],
            ],
        )
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn look_at_lh(eye: Point3<T, Src>, look: Point3<T, Src>, up: Vector3<T, Src>) -> Self
    where
        T: Real,
    {
        Self::look_to_lh(eye, look - eye, up)
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn look_to_lh(eye: Point3<T, Src>, dir: Vector3<T, Src>, up: Vector3<T, Src>) -> Self
    where
        T: Real,
    {
        Self::look_to_rh(eye, -dir, up)
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn look_at_rh(eye: Point3<T, Src>, look: Point3<T, Src>, up: Vector3<T, Src>) -> Self
    where
        T: Real,
    {
        Self::look_to_rh(eye, look - eye, up)
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn look_to_rh(eye: Point3<T, Src>, dir: Vector3<T, Src>, up: Vector3<T, Src>) -> Self
    where
        T: Real,
    {
        let eye = eye.to_vector();
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        let (o, l) = (Zero::zero(), One::one());
        Self::new([
            [              s.x,               u.x,              -f.x, o],
            [              s.y,               u.y,              -f.y, o],
            [              s.z,               u.z,              -f.z, o],
            [-eye.dot(s), -eye.dot(u), -eye.dot(f), l],
        ])
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn perspective_lh(fov_y: Angle<T>, aspect_ratio: T, z_near: Length<T, Src>, z_far: Length<T, Src>) -> Self
    where
        T: PartialEq + Trig,
    {
        let two = T::one() + T::one();
        let (z_near, z_far) = (z_near.get(), z_far.get());
        let fov = fov_y.radians() / two;
        let (sin_fov, cos_fov) = (fov.sin(), fov.cos());
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        let (o, l) = (T::zero(), T::one());
        Self::new([
            [w, o,              o, o],
            [o, h,              o, o],
            [o, o,              r, l],
            [o, o, (o-r) * z_near, o],
        ])
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn perspective_rh(fov_y: Angle<T>, aspect_ratio: T, z_near: T, z_far: T) -> Self
    where
        T: PartialEq + Trig,
    {
        let two = T::one() + T::one();
        let (z_near, z_far) = (z_near, z_far);
        let fov = fov_y.radians() / two;
        let (sin_fov, cos_fov) = (fov.sin(), fov.cos());
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        let (o, l) = (T::zero(), T::one());
        Self::new([
            [w, o,          o,   o],
            [o, h,          o,   o],
            [o, o,          r, o-l],
            [o, o, r * z_near,   o],
        ])
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn orthographic_lh(
        left: T,
        right: T,
        bottom: T,
        top: T,
        near: T,
        far: T,
    ) -> Self
    where
        T: PartialEq,
    {
        let (l, o) = (T::one(), T::zero());
        let w = l / (right - left);
        let h = l / (top - bottom);
        let r = l / (far - near);
        Self::new([
            [                 w + w,                      o,            o, o],
            [                     o,                  h + h,            o, o],
            [                     o,                      o,            r, o],
            [o - (left + right) * w, o - (top + bottom) * h, o - r * near, l],
        ])
    }

    #[inline]
    #[must_use]
    #[rustfmt::skip]
    pub fn orthographic_rh(
        left: T,
        right: T,
        bottom: T,
        top: T,
        near: T,
        far: T,
    ) -> Self
    where
        T: PartialEq,
    {
        let (l, o) = (T::one(), T::zero());
        let w = l / (right - left);
        let h = l / (top - bottom);
        let r = l / (near - far);
        Self::new([
            [                 w + w,                      o,        o, o],
            [                     o,                  h + h,        o, o],
            [                     o,                      o,        r, o],
            [o - (left + right) * w, o - (top + bottom) * h, r * near, l],
        ])
    }

    #[inline]
    #[must_use]
    pub fn determinant(&self) -> T {
        Self::mat_determinant(self.mat)
    }

    #[must_use]
    #[rustfmt::skip]
    fn mat_determinant(m: [[T; 4]; 4]) -> T {
        let [
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ] = m;
        m14 * m23 * m32 * m41
        - m13 * m24 * m32 * m41
        - m14 * m22 * m33 * m41
        + m12 * m24 * m33 * m41
        + m13 * m22 * m34 * m41
        - m12 * m23 * m34 * m41
        - m14 * m23 * m31 * m42
        + m13 * m24 * m31 * m42
        + m14 * m21 * m33 * m42
        - m11 * m24 * m33 * m42
        - m13 * m21 * m34 * m42
        + m11 * m23 * m34 * m42
        + m14 * m22 * m31 * m43
        - m12 * m24 * m31 * m43
        - m14 * m21 * m32 * m43
        + m11 * m24 * m32 * m43
        + m12 * m21 * m34 * m43
        - m11 * m22 * m34 * m43
        - m13 * m22 * m31 * m44
        + m12 * m23 * m31 * m44
        + m13 * m21 * m32 * m44
        - m11 * m23 * m32 * m44
        - m12 * m21 * m33 * m44
        + m11 * m22 * m33 * m44
    }

    #[must_use]
    #[rustfmt::skip]
    fn mat_inverse(m: [[T; 4]; 4]) -> Option<[[T; 4]; 4]>
    where
        T: PartialEq,
    {
        let det = Self::mat_determinant(m);
        if det == T::zero() {
            return None;
        }

        let [
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ] = m;
        let inv_det = T::one() / det;
        let mat = [
            [inv_det * (m23 * m34 * m42 - m24 * m33 * m42
                        + m24 * m32 * m43 - m22 * m34 * m43
                        - m23 * m32 * m44 + m22 * m33 * m44),
             inv_det * (m14 * m33 * m42 - m13 * m34 * m42
                        - m14 * m32 * m43 + m12 * m34 * m43
                        + m13 * m32 * m44 - m12 * m33 * m44),
             inv_det * (m13 * m24 * m42 - m14 * m23 * m42
                        + m14 * m22 * m43 - m12 * m24 * m43
                        - m13 * m22 * m44 + m12 * m23 * m44),
             inv_det * (m14 * m23 * m32 - m13 * m24 * m32
                        - m14 * m22 * m33 + m12 * m24 * m33
                        + m13 * m22 * m34 - m12 * m23 * m34)],
            [inv_det * (m24 * m33 * m41 - m23 * m34 * m41
                        - m24 * m31 * m43 + m21 * m34 * m43
                        + m23 * m31 * m44 - m21 * m33 * m44),
             inv_det * (m13 * m34 * m41 - m14 * m33 * m41
                        + m14 * m31 * m43 - m11 * m34 * m43
                        - m13 * m31 * m44 + m11 * m33 * m44),
             inv_det * (m14 * m23 * m41 - m13 * m24 * m41
                        - m14 * m21 * m43 + m11 * m24 * m43
                        + m13 * m21 * m44 - m11 * m23 * m44),
             inv_det * (m13 * m24 * m31 - m14 * m23 * m31
                        + m14 * m21 * m33 - m11 * m24 * m33
                        - m13 * m21 * m34 + m11 * m23 * m34)],
            [inv_det * (m22 * m34 * m41 - m24 * m32 * m41
                        + m24 * m31 * m42 - m21 * m34 * m42
                        - m22 * m31 * m44 + m21 * m32 * m44),
             inv_det * (m14 * m32 * m41 - m12 * m34 * m41
                        - m14 * m31 * m42 + m11 * m34 * m42
                        + m12 * m31 * m44 - m11 * m32 * m44),
             inv_det * (m12 * m24 * m41 - m14 * m22 * m41
                        + m14 * m21 * m42 - m11 * m24 * m42
                        - m12 * m21 * m44 + m11 * m22 * m44),
             inv_det * (m14 * m22 * m31 - m12 * m24 * m31
                        - m14 * m21 * m32 + m11 * m24 * m32
                        + m12 * m21 * m34 - m11 * m22 * m34)],
            [inv_det * (m23 * m32 * m41 - m22 * m33 * m41
                        - m23 * m31 * m42 + m21 * m33 * m42
                        + m22 * m31 * m43 - m21 * m32 * m43),
             inv_det * (m12 * m33 * m41 - m13 * m32 * m41
                        + m13 * m31 * m42 - m11 * m33 * m42
                        - m12 * m31 * m43 + m11 * m32 * m43),
             inv_det * (m13 * m22 * m41 - m12 * m23 * m41
                        - m13 * m21 * m42 + m11 * m23 * m42
                        + m12 * m21 * m43 - m11 * m22 * m43),
             inv_det * (m12 * m23 * m31 - m13 * m22 * m31
                        + m13 * m21 * m32 - m11 * m23 * m32
                        - m12 * m21 * m33 + m11 * m22 * m33)],
        ];
        Some(mat)
    }
    
    #[inline]
    #[must_use]
    #[rustfmt::skip]
    fn mat_transpose(m: [[T; 4]; 4]) -> [[T; 4]; 4] {
        let [
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ] = m;
        [
            [m11, m21, m31, m41],
            [m12, m22, m32, m42],
            [m13, m23, m33, m43],
            [m14, m24, m34, m44],
        ]
    }

    fn transform_point3(&self, p: Point3<T, Src>) -> Result<Point3<T, Dst>, ()>
    where
        T: Copy + PartialOrd + Zero + One + NumOps,
    {
        Transform::transform(self, p).try_into()
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Transform2<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + One + NumOps,
{
    type Inverse = Transform2<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Self::translation(Vector2::zero())
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.mat == Self::identity().mat
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        let &Self { mat, mat_inv, .. } = self;
        Transform2::new_raw(mat_inv, mat)
    }
}

impl<T, Src, Dst> Transformation<T, Src, Dst> for Transform3<T, Src, Dst>
where
    T: Copy + PartialEq + Zero + One + NumOps,
{
    type Inverse = Transform3<T, Dst, Src>;

    #[inline]
    fn identity() -> Self {
        Self::translation(Vector3::zero())
    }

    #[inline]
    fn is_identity(&self) -> bool {
        self.mat == Self::identity().mat
    }

    #[inline]
    fn inverse(&self) -> Self::Inverse {
        let &Self { mat, mat_inv, .. } = self;
        Transform3::new_raw(mat_inv, mat)
    }
}

impl<T, Src, Dst> Transform<Point2<T, Src>> for Transform2<T, Src, Dst>
where
    T: Copy + NumOps,
{
    type Output = Point2<T, Dst>;

    #[inline]
    fn transform(&self, p: Point2<T, Src>) -> Self::Output {
        let [[m11, m12], [m21, m22], [dx, dy]] = self.mat;
        Point2::new(p.x * m11 + p.y * m21 + dx, p.x * m12 + p.y * m22 + dy)
    }
}

impl<T, Src, Dst> Transform<Vector2<T, Src>> for Transform2<T, Src, Dst>
where
    T: Copy + NumOps,
{
    type Output = Vector2<T, Dst>;

    #[inline]
    fn transform(&self, v: Vector2<T, Src>) -> Self::Output {
        let [[m11, m12], [m21, m22], _] = self.mat;
        Vector2::new(v.x * m11 + v.y * m21, v.x * m12 + v.y * m22)
    }
}

impl<T, Src, Dst> Transform<Vector2<T, Normal<Src>>> for Transform2<T, Src, Dst>
where
    T: Copy + NumOps,
{
    type Output = Vector2<T, Dst>;

    #[inline]
    fn transform(&self, n: Vector2<T, Normal<Src>>) -> Self::Output {
        let [[m11, m21], [m12, m22], _] = self.mat_inv;
        Vector2::new(n.x * m11 + n.y * m21, n.x * m12 + n.y * m22)
    }
}

impl<T, Src, Dst> Transform<Box2<T, Src>> for Transform2<T, Src, Dst> 
where
    T: Copy + PartialOrd + Zero + NumOps,
{
    type Output = Box2<T, Dst>;

    fn transform(&self, b: Box2<T, Src>) -> Self::Output {
        Box2::from_points([
            self.transform(b.min),
            self.transform(b.max),
            self.transform(Point2::new(b.max.x, b.min.y)),
            self.transform(Point2::new(b.min.x, b.max.y)),
        ])
    }
}

impl<T, Src, Dst> Transform<Point3<T, Src>> for Transform3<T, Src, Dst>
where
    T: Copy + PartialOrd + Zero + One + NumOps,
{
    type Output = HomogeneousVector<T, Dst>;

    #[rustfmt::skip]
    fn transform(&self, p: Point3<T, Src>) -> Self::Output {
        let [
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44],
        ] = self.mat;
        let x = p.x * m11 + p.y * m21 + p.z * m31 + m41;
        let y = p.x * m12 + p.y * m22 + p.z * m32 + m42;
        let z = p.x * m13 + p.y * m23 + p.z * m33 + m43;
        let w = p.x * m14 + p.y * m24 + p.z * m34 + m44;
        HomogeneousVector::new(x, y, z, w)
    }
}

impl<T, Src, Dst> Transform<Vector3<T, Src>> for Transform3<T, Src, Dst>
where
    T: Copy + NumOps,
{
    type Output = Vector3<T, Dst>;

    #[rustfmt::skip]
    fn transform(&self, v: Vector3<T, Src>) -> Self::Output {
        let [
            [m11, m12, m13, _],
            [m21, m22, m23, _],
            [m31, m32, m33, _],
            _,
        ] = self.mat;
        let x = v.x * m11 + v.y * m21 + v.z * m31;
        let y = v.x * m12 + v.y * m22 + v.z * m32;
        let z = v.x * m13 + v.y * m23 + v.z * m33;
        Vector3::new(x, y, z)
    }
}

impl<T, Src, Dst> Transform<Vector3<T, Normal<Src>>> for Transform3<T, Src, Dst>
where
    T: Copy + NumOps,
{
    type Output = Vector3<T, Normal<Dst>>;

    #[rustfmt::skip]
    fn transform(&self, n: Vector3<T, Normal<Src>>) -> Self::Output {
        let [
        [m11, m12, m13, _],
        [m21, m22, m23, _],
        [m31, m32, m33, _],
        _,
        ] = self.mat_inv;
        let x = n.x * m11 + n.y * m12 + n.z * m13;
        let y = n.x * m21 + n.y * m22 + n.z * m23;
        let z = n.x * m31 + n.y * m32 + n.z * m33;
        Vector3::new(x, y, z)
    }
}

impl<T, Src, Dst> Transform<Box3<T, Src>> for Transform3<T, Src, Dst>
where
    T: Copy + PartialOrd + Zero + One + NumOps,
{
    type Output = Option<Box3<T, Dst>>;

    fn transform(&self, b: Box3<T, Src>) -> Self::Output {
        let Box3 { min, max } = b;
        Box3::try_from_points([
            self.transform_point3(min),
            self.transform_point3(Point3::new(min.x, min.y, max.z)),
            self.transform_point3(Point3::new(min.x, max.y, min.z)),
            self.transform_point3(Point3::new(min.x, max.y, max.z)),
            self.transform_point3(Point3::new(max.x, min.y, min.z)),
            self.transform_point3(Point3::new(max.x, min.y, max.z)),
            self.transform_point3(Point3::new(max.x, max.y, min.z)),
            self.transform_point3(max),
        ]).ok()
    }
}

impl<'a, T, A, B, C> Mul<Transform2<T, B, C>> for &'a Transform2<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform2<T, A, C>;

    #[inline]
    fn mul(self, rhs: Transform2<T, B, C>) -> Self::Output {
        *self * rhs
    }
}

impl<'b, T, A, B, C> Mul<&'b Transform2<T, B, C>> for Transform2<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform2<T, A, C>;

    #[inline]
    fn mul(self, rhs: &'b Transform2<T, B, C>) -> Self::Output {
        self * *rhs
    }
}

impl<'a, 'b, T, A, B, C> Mul<&'b Transform2<T, B, C>> for &'a Transform2<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform2<T, A, C>;

    #[inline]
    fn mul(self, rhs: &'b Transform2<T, B, C>) -> Self::Output {
        *self * *rhs
    }
}

impl<T, A, B, C> Mul<Transform2<T, B, C>> for Transform2<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform2<T, A, C>;

    fn mul(self, rhs: Transform2<T, B, C>) -> Self::Output {
        fn matmul<T>(a: [[T; 2]; 3], b: [[T; 2]; 3]) -> [[T; 2]; 3]
        where
            T: Copy + Add<Output = T> + Mul<Output = T>,
        {
            let [[a11, a12], [a21, a22], [a31, a32]] = a;
            let [[b11, b12], [b21, b22], [b31, b32]] = b;
            [
                [a11 * b11 + a12 * b21, a11 * b12 + a12 * b22],
                [a21 * b11 + a22 * b21, a21 * b12 + a22 * b22],
                [a31 * b11 + a32 * b21 + b31, a31 * b12 + a32 * b22 + b32],
            ]
        }

        let Transform2 {
            mat: m1,
            mat_inv: m1_inv,
            ..
        } = self;
        let Transform2 {
            mat: m2,
            mat_inv: m2_inv,
            ..
        } = rhs;

        let mat = matmul(m1, m2);
        let mat_inv = matmul(m2_inv, m1_inv);
        Transform2::new_raw(mat, mat_inv)
    }
}

impl<'a, T, A, B, C> Mul<Transform3<T, B, C>> for &'a Transform3<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform3<T, A, C>;

    #[inline]
    fn mul(self, rhs: Transform3<T, B, C>) -> Self::Output {
        *self * rhs
    }
}

impl<'b, T, A, B, C> Mul<&'b Transform3<T, B, C>> for Transform3<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform3<T, A, C>;

    #[inline]
    fn mul(self, rhs: &'b Transform3<T, B, C>) -> Self::Output {
        self * *rhs
    }
}

impl<'a, 'b, T, A, B, C> Mul<&'b Transform3<T, B, C>> for &'a Transform3<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform3<T, A, C>;

    #[inline]
    fn mul(self, rhs: &'b Transform3<T, B, C>) -> Self::Output {
        *self * *rhs
    }
}

impl<T, A, B, C> Mul<Transform3<T, B, C>> for Transform3<T, A, B>
where
    T: Copy + NumOps,
{
    type Output = Transform3<T, A, C>;

    fn mul(self, rhs: Transform3<T, B, C>) -> Self::Output {
        fn matmul<T>(a: [[T; 4]; 4], b: [[T; 4]; 4]) -> [[T; 4]; 4]
        where
            T: Copy + Add<Output = T> + Mul<Output = T>,
        {
            let [
                [a11, a12, a13, a14],
                [a21, a22, a23, a24],
                [a31, a32, a33, a34],
                [a41, a42, a43, a44],
            ] = a;
            let [
                [b11, b12, b13, b14],
                [b21, b22, b23, b24],
                [b31, b32, b33, b34],
                [b41, b42, b43, b44],
            ] = b;
            [
                [
                    a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41,
                    a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42,
                    a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43,
                    a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44,
                ],
                [
                    a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41,
                    a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42,
                    a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43,
                    a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44,
                ],
                [
                    a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41,
                    a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42,
                    a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43,
                    a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44,
                ],
                [
                    a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41,
                    a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42,
                    a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43,
                    a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44,
                ],
            ]
        }

        let Transform3 {
            mat: m1,
            mat_inv: m1_inv,
            ..
        } = self;
        let Transform3 {
            mat: m2,
            mat_inv: m2_inv,
            ..
        } = rhs;

        let mat = matmul(m1, m2);
        let mat_inv = matmul(m2_inv, m1_inv);
        Transform3::new_raw(mat, mat_inv)
    }
}

impl<T, Src, Dst> fmt::Debug for Transform2<T, Src, Dst>
where
    T: fmt::Debug + Copy + PartialEq + Zero + One,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_identity() {
            write!(f, "[I]")
        } else {
            fmt::Debug::fmt(&self.mat, f)
        }
    }
}

impl<T, Src, Dst> fmt::Debug for Transform3<T, Src, Dst>
where
    T: fmt::Debug + Copy + PartialEq + Zero + One,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_identity() {
            write!(f, "[I]")
        } else {
            fmt::Debug::fmt(&self.mat, f)
        }
    }
}

impl<T: ApproxEq, Src, Dst> ApproxEq<T> for Transform2<T, Src, Dst> {
    #[inline]
    fn epsilon() -> T {
        T::epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        self.mat.approx_eq_eps(&other.mat, eps)
    }
}

impl<T: ApproxEq, Src, Dst> ApproxEq<T> for Transform3<T, Src, Dst> {
    #[inline]
    fn epsilon() -> T {
        T::epsilon()
    }

    #[inline]
    fn approx_eq_eps(&self, other: &Self, eps: &T) -> bool {
        self.mat.approx_eq_eps(&other.mat, eps)
    }
}

impl<T, Src, Dst> From<Translation2<T, Src, Dst>> for Transform2<T, Src, Dst>
where
    T: Copy + Zero + One + NumOps,
{
    fn from(t: Translation2<T, Src, Dst>) -> Self {
        Self::translation(t.to_vector())
    }
}

impl<T, Src, Dst> From<Rotation2<T, Src, Dst>> for Transform2<T, Src, Dst>
where
    T: Copy + Zero + One + Trig + NumOps,
{
    fn from(t: Rotation2<T, Src, Dst>) -> Self {
        Self::rotation(t.angle)
    }
}

impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform2<T, Src, Dst>
where
    T: Copy + Zero + One + NumOps,
{
    fn from(t: Scale<T, Src, Dst>) -> Self {
        Self::scale(t, t)
    }
}

impl<T, Src, Dst> From<Translation3<T, Src, Dst>> for Transform3<T, Src, Dst>
where
    T: Copy + Zero + One + NumOps,
{
    fn from(t: Translation3<T, Src, Dst>) -> Self {
        Self::translation(t.to_vector())
    }
}

impl<T, Src, Dst> From<Rotation3<T, Src, Dst>> for Transform3<T, Src, Dst>
where
    T: Copy + Zero + One + Trig + NumOps,
{
    fn from(t: Rotation3<T, Src, Dst>) -> Self {
        let v = t.vector_part();
        let v2 = v + v;
        let [ii, ij, ik] = (v2 * t.i).to_array();
        let [_, jj, jk] = (v2 * t.j).to_array();
        let kk = t.k * v2.z;
        let [ai, aj, ak] = (v2 * t.a).to_array();

        let (l, o) = (T::one(), T::zero());

        let m11 = l - (jj + kk);
        let m12 = ij + ak;
        let m13 = ik - aj;

        let m21 = ij - ak;
        let m22 = l - (ii + kk);
        let m23 = jk + ai;

        let m31 = ik + aj;
        let m32 = jk - ai;
        let m33 = l - (ii + jj);

        let mat = [
            [m11, m12, m13, o],
            [m21, m22, m23, o],
            [m31, m32, m33, o],
            [  o,   o,   o, l],
        ];
        Self::new_raw(mat, Self::mat_transpose(mat))
    }
}

impl<T, Src, Dst> From<Scale<T, Src, Dst>> for Transform3<T, Src, Dst>
where
    T: Copy + Zero + One + NumOps,
{
    fn from(t: Scale<T, Src, Dst>) -> Self {
        Self::scale(t, t, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    type Mf32 = Transform3<f32, UnknownUnit, UnknownUnit>;
    type Vf32 = Vector3<f32, UnknownUnit>;
    type Nf32 = Vector3<f32, Normal<UnknownUnit>>;

    #[test]
    pub fn test_translation() {
        let t1 = &Mf32::translation(Vf32::new(1., 2., 3.));
        let t2 = t1 * Mf32::identity();
        let t3 = Mf32::identity() * t1;
        assert_eq!(t1, &t2);
        assert_eq!(t1, &t3);
        assert_eq!(t1.transform_point3(Point3::splat(1.)), Ok(Point3::new(2., 3., 4.)));
        assert_eq!(Transform::transform(t1, Vf32::splat(2.)), Vf32::splat(2.));
        assert_eq!(Transform::transform(t1, Nf32::splat(1.)), Nf32::splat(1.));
        assert_eq!(t1 * t1, Mf32::translation(Vf32::new(2., 4., 6.)));
    }

    #[test]
    pub fn test_rotation() {

    }
}
