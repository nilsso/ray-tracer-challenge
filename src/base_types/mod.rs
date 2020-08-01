/// Base types for the ray tracer

// Utility to define a three coordinate struct
macro_rules! coordinate_struct {
    ($struct:tt, $x:tt, $y:tt, $z:tt) => {
        #[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
        pub struct $struct {
            pub $x: f64,
            pub $y: f64,
            pub $z: f64,
        }

        impl $struct {
            pub const fn new($x: f64, $y: f64, $z: f64) -> Self {
                Self { $x, $y, $z }
            }

            pub const fn zero() -> Self {
                Self::new(0.0, 0.0, 0.0)
            }

            pub const fn one() -> Self {
                Self::new(1.0, 1.0, 1.0)
            }
        }
    };
}

// Utility to implement From trait for one three coordinate structs to another
macro_rules! coordinate_struct_convert {
    ($to:tt, $from:tt, $x:tt, $y:tt, $z:tt) => {
        impl From<$from> for $to {
            fn from(other: $from) -> $to {
                $to::new(other.$x, other.$y, other.$z)
            }
        }
    };
}

mod color;
mod point;
mod vector;

pub use color::*;
pub use point::*;
pub use vector::*;

#[macro_use]
mod tiny_matrix;
use std::ops::{Add, Mul};

// Square matrices
matrix!(Matrix1, 1);
matrix!(Matrix2, 2);
matrix!(Matrix3, 3);
matrix!(Matrix4, 4);

matrix!(Matrix1x4, 4, 1);
matrix!(Matrix2x4, 4, 1);
matrix!(Matrix3x4, 4, 1);

matrix!(Matrix4x1, 4, 1);
matrix!(Matrix4x2, 4, 1);
matrix!(Matrix4x3, 4, 1);

// Matrix multiplication
//matrix_mul!((Matrix2x3, Matrix3x2, 3), (Matrix2, 2));
//matrix_mul!((Matrix1x4, Matrix4x1, 1), (Matrix1, 1));
//matrix_mul!((Matrix2x4, Matrix4x1, 1), (Matrix2x1, 1));

#[cfg(test)]
mod tests {
    macro_rules! assert_eq_commutative {
        ($op:tt, $lhs:expr, $rhs:expr, $res:expr) => {
            assert_eq!($lhs $op $rhs, $res);
            assert_eq!($rhs $op $lhs, $res);
        }
    }

    mod point_vector_color {
        use crate::{Color, Point, Vector};

        use std::f64::{consts::PI, EPSILON};

        #[test]
        fn adding_two_vectors() {
            const A: Vector = Vector::new(3.0, 2.0, 1.0);
            const B: Vector = Vector::new(5.0, 6.0, 7.0);
            const R: Vector = Vector::new(8.0, 8.0, 8.0);

            assert_eq_commutative!(+, A, B, R);
        }

        #[test]
        fn adding_a_point_and_a_vector() {
            const A: Point = Point::new(3.0, 2.0, 1.0);
            const B: Vector = Vector::new(5.0, 6.0, 7.0);
            const R: Point = Point::new(8.0, 8.0, 8.0);

            assert_eq_commutative!(+, A, B, R);
        }

        #[test]
        fn subtracting_two_points() {
            const A: Point = Point::new(3.0, 2.0, 1.0);
            const B: Point = Point::new(5.0, 6.0, 7.0);
            const R: Vector = Vector::new(-2.0, -4.0, -6.0);

            assert_eq!(A - B, R);
        }

        #[test]
        fn subtracting_a_vector_from_a_point() {
            const A: Point = Point::new(3.0, 2.0, 1.0);
            const B: Vector = Vector::new(5.0, 6.0, 7.0);
            const R: Point = Point::new(-2.0, -4.0, -6.0);

            assert_eq!(A - B, R);
        }

        #[test]
        fn subtracting_two_vectors() {
            const A: Vector = Vector::new(3.0, 2.0, 1.0);
            const B: Vector = Vector::new(5.0, 6.0, 7.0);
            const R: Vector = Vector::new(-2.0, -4.0, -6.0);

            assert_eq!(A - B, R);
        }

        #[test]
        fn inverse_of_a_vector() {
            const A: Vector = Vector::new(3.0, 2.0, 1.0);
            const R: Vector = Vector::new(-3.0, -2.0, -1.0);

            assert_eq!(-A, R);
        }

        #[test]
        fn multiplying_a_vector_by_a_scalar() {
            const A: Vector = Vector::new(1.0, -2.0, 3.0);
            const R: Vector = Vector::new(3.5, -7.0, 10.5);

            assert_eq!(A * 3.5, R);
        }

        #[test]
        fn dividing_a_vector_by_a_scalar() {
            const A: Vector = Vector::new(1.0, -2.0, 3.0);
            const R: Vector = Vector::new(0.5, -1.0, 1.5);

            assert_eq!(A / 2.0, R);
        }

        #[test]
        fn length_of_a_vector() {
            const A: Vector = Vector::new(-1.0, 2.0, -3.0);

            assert_eq!(A.length(), 14f64.sqrt());
        }

        #[test]
        fn normalization_of_a_vector() {
            const A: Vector = Vector::new(1.0, 2.0, 3.0);
            let q = 14f64.sqrt();
            let r = Vector::new(1.0 / q, 2.0 / q, 3.0 / q);

            assert_eq!(A.normalize(), r);
        }

        #[test]
        fn dot_product_of_orthogonal_vectors_is_zero() {
            const A: Vector = Vector::new(1.0, 2.0, 0.0);
            const B: Vector = Vector::new(0.0, 0.0, 3.0);

            assert_eq!(A.dot(&B), 0.0);
        }

        #[test]
        fn dot_product_of_a_unit_vector_and_its_inverse_is_minus_one() {
            let a = Vector::new(1.0, 2.0, 3.0).normalize();

            assert_eq!(a.dot(&(-a)), -1.0);
        }

        #[test]
        fn dot_product_of_unit_vectors_is_cos_of_their_angle() {
            assert!({
                // 10:30 and 9:30, theta is 90 degrees
                let a = Vector::new(1.0, 1.0, 0.0).normalize();
                let b = Vector::new(-1.0, 1.0, 0.0).normalize();
                let theta = (PI / 2.0).cos();

                a.dot(&b) - theta < EPSILON
            });

            assert!({
                // 9 o-clock and 1:30, theta is 135 degrees
                let a = Vector::new(1.0, 1.0, 0.0).normalize();
                let b = Vector::new(-1.0, 0.0, 0.0);
                let theta = (PI * 3.0 / 2.0).cos();

                a.dot(&b) - theta < EPSILON
            });
        }

        #[test]
        fn cross_product_is_orthogonal() {
            const A: Vector = Vector::new(1.0, 2.0, 3.0);
            const B: Vector = Vector::new(2.0, 3.0, 4.0);
            const R: Vector = Vector::new(-1.0, 2.0, -1.0);

            // Order matters!
            assert_eq!(A.cross(&B), R);
            assert_eq!(B.cross(&A), -R);

            assert!(A.dot(&R) - EPSILON < 0.0);
            assert!(B.dot(&R) - EPSILON < 0.0);
        }

        #[test]
        fn color_operations() {
            const A: Color = Color::new(1.0, 2.0, 3.0);

            assert_eq_commutative!(+, A, A, Color::new(2.0, 4.0, 6.0));
            assert_eq_commutative!(*, A, A, Color::new(1.0, 4.0, 9.0));
            assert_eq_commutative!(*, A, 3.0, Color::new(3.0, 6.0, 9.0));
            assert_eq!(A - 0.5 * A, Color::new(0.5, 1.0, 1.5));
        }
    }

    mod matrix {}
}
