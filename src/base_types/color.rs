use std::fmt::{Display, Error as FmtError, Formatter};

use auto_ops::*;

use super::{Point, Vector};

coordinate_struct!(Color, r, g, b);

coordinate_struct_convert!(Color, Point, x, y, z);
coordinate_struct_convert!(Color, Vector, x, y, z);

pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

impl Color {
    pub fn gray(value: f64) -> Self {
        Self::new(value, value, value)
    }
}

impl Default for Color {
    fn default() -> Self {
        BLACK
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let r = ((255.0 * self.r) as u64).clamp(0, 255);
        let g = ((255.0 * self.g) as u64).clamp(0, 255);
        let b = ((255.0 * self.b) as u64).clamp(0, 255);

        Ok(write!(f, "{} {} {}", r, g, b)?)
    }
}

// Color and color addition
impl_op_ex!(+|lhs: &Color, rhs: &Color| -> Color {
    let r = lhs.r + rhs.r;
    let g = lhs.g + rhs.g;
    let b = lhs.b + rhs.b;

    Color::new(r, g, b)
});

// Color and color subtraction
impl_op_ex!(-|lhs: &Color, rhs: &Color| -> Color {
    let r = lhs.r - rhs.r;
    let g = lhs.g - rhs.g;
    let b = lhs.b - rhs.b;

    Color::new(r, g, b)
});

// Color and color multiplication (Hadamard/element-wise product)
impl_op_ex!(*|lhs: &Color, rhs: &Color| -> Color {
    let r = lhs.r * rhs.r;
    let g = lhs.g * rhs.g;
    let b = lhs.b * rhs.b;

    Color::new(r, g, b)
});

// Color and scalar multiplication
impl_op_ex_commutative!(*|lhs: &Color, rhs: &f64| -> Color {
    let rhs = Color::new(*rhs, *rhs, *rhs);

    lhs * rhs
});
