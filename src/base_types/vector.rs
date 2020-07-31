use auto_ops::*;

use super::Point;

// Vector
coordinate_struct!(Vector, x, y, z);

// From point to vector
coordinate_struct_convert!(Vector, Point, x, y, z);

impl Vector {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Self::new(x, y, z)
    }

    pub fn hadamard(&self, other: &Self) -> Self {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        Self::new(x, y, z)
    }
}

// Inverse
impl_op_ex!(-|v: &Vector| -> Vector {
    let x = -v.x;
    let y = -v.y;
    let z = -v.z;

    Vector::new(x, y, z)
});

// Vector and vector addition
impl_op_ex!(+|lhs: &Vector, rhs: &Vector| -> Vector {
    let x = lhs.x + rhs.x;
    let y = lhs.y + rhs.y;
    let z = lhs.z + rhs.z;

    Vector::new(x, y, z)
});

// Vector and vector subtraction
impl_op_ex!(-|lhs: &Vector, rhs: &Vector| -> Vector {
    let rhs = -rhs;

    lhs + rhs
});

// Vector and scalar addition
impl_op_ex_commutative!(+|lhs: &Vector, rhs: &f64| -> Vector {
    let rhs = Vector::new(*rhs, *rhs, *rhs);

    lhs + rhs
});

// Vector and scalar multiplication
impl_op_ex_commutative!(*|lhs: &Vector, rhs: &f64| -> Vector {
    let x = lhs.x * rhs;
    let y = lhs.y * rhs;
    let z = lhs.z * rhs;

    Vector::new(x, y, z)
});

// Vector and scalar division
impl_op_ex_commutative!(/|lhs: &Vector, rhs: &f64| -> Vector {
    let x = lhs.x / rhs;
    let y = lhs.y / rhs;
    let z = lhs.z / rhs;

    Vector::new(x, y, z)
});

