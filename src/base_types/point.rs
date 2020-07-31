use auto_ops::*;

use super::Vector;

// Point
coordinate_struct!(Point, x, y, z);

// From vector to point
coordinate_struct_convert!(Point, Vector, x, y, z);

impl Point {
    pub fn to(&self, other: &Point) -> Vector {
        other - self
    }
}

// Point and vector addition (move a point)
impl_op_ex_commutative!(+|lhs: &Point, rhs: &Vector| -> Point {
    let lhs = Vector::from(*lhs);

    Point::from(lhs + rhs)
});

// Point and vector subtraction (move a point)
impl_op_ex!(-|lhs: &Point, rhs: &Vector| -> Point {
    let rhs = -rhs;

    lhs + rhs
});

// Point and point subtraction (vector between points)
impl_op_ex!(-|lhs: &Point, rhs: &Point| -> Vector {
    let lhs = Vector::from(*lhs);
    let rhs = Vector::from(*rhs);

    lhs - rhs
});

