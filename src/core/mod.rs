#[macro_use]
mod macros;

pub mod geometry;
pub mod num;
pub mod units;

pub mod prelude {
    use super::geometry::Normal;
    pub use super::{
        geometry::{
            transform::{Rotation2, Rotation3, Scale, Transformation, Translation2, Translation3},
            Axis2, Axis3, Box2, Box3, Mask2, Mask3, Point2, Point3, Ray, Size2, Size3, Vector2,
            Vector3,
        },
        num::{Cast, Ceil, Floor, One, Round, ToPrimitive, Zero},
        units::{Angle, Length, Time},
    };

    pub type Normal2<T, U> = Vector2<T, Normal<U>>;
    pub type Normal3<T, U> = Vector3<T, Normal<U>>;
}
