use std::ops::{Neg, Add, Mul, Sub, Div};

macro_rules! matrix {
    // (Internal match) Matrix definition and implementation
    // - `matrix`: Matrix type name
    // - `D`: Number of rows/columns
    (@matrix_def $matrix:tt, $D:tt) => {
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct $matrix {
            d: usize,
            pub data: [[f64; $D]; $D],
        }

        impl $matrix {
            pub const fn new(data: [[f64; $D]; $D]) -> Self {
                Self {
                    d: $D,
                    data
                }
            }

            pub const fn zero() -> Self {
                Self::new([[0.0; $D]; $D])
            }

            pub const fn one() -> Self {
                Self::new([[1.0; $D]; $D])
            }

            pub fn ident() -> Self {
                let mut m = Self::zero();
                for i in 0..$D {
                    m.data[i][i] = 1.0;
                }

                m
            }

            pub fn iter(&self) -> impl Iterator<Item = &f64> {
                self.data.iter().flat_map(|r| r.iter())
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f64> {
                self.data.iter_mut().flat_map(|r| r.iter_mut())
            }

            pub fn transposed(&self) -> Self {
                let mut res = self.clone();

                for r in 0..self.d {
                    for c in (r+1)..self.d {
                        res.data[c][r] = self.data[r][c];
                        res.data[r][c] = self.data[c][r];
                    }
                }

                res
            }
        }

        impl Neg for $matrix {
            type Output = $matrix;

            fn neg(mut self) -> Self::Output {
                for r in 0..self.d {
                    for c in 0..self.d {
                        self.data[r][c] = -self.data[r][c];
                    }
                }
                self
            }
        }

        impl Add for $matrix {
            type Output = $matrix;

            fn add(mut self, rhs: $matrix) -> Self::Output {
                for r in 0..self.d {
                    for c in 0..self.d {
                        self.data[r][c] += rhs.data[r][c];
                    }
                }
                self
            }
        }

        impl Sub for $matrix {
            type Output = $matrix;

            fn sub(self, rhs: $matrix) -> Self::Output {
                self + (-rhs)
            }
        }

        impl Mul for $matrix {
            type Output = $matrix;

            fn mul(self, rhs: $matrix) -> Self::Output {
                let mut res = $matrix::zero();

                for r in 0..self.d {
                    for c in 0..self.d {
                        for i in 0..self.d {
                            res.data[r][c] += (self.data[r][i] * rhs.data[i][c]);
                        }
                    }
                }

                res
            }
        }

        impl Div<f64> for $matrix {
            type Output = $matrix;

            fn div(mut self, rhs: f64) -> Self::Output {
                self.iter_mut().for_each(|v| *v /= rhs);
                self
            }
        }
    };

    // (Internal match) Matrix additional implementation (requiring the sub-matrix type)
    // - `matrix`: Matrix type name
    // - `sub_matrix`: Sub-matrix type name
    (@matrix_det $matrix:tt, $sub_matrix:tt) => {
        impl $matrix {
            pub fn delete(&self, r: usize, c: usize) -> $sub_matrix {
                let mut res = $sub_matrix::zero();

                for (y, r) in (0..r).chain((r + 1)..self.d).enumerate() {
                    for (x, c) in (0..c).chain((c + 1)..self.d).enumerate() {
                        res.data[y][x] = self.data[r][c];
                    }
                }

                res
            }

            pub fn cofactor(&self, r: usize, c: usize) -> f64 {
                let f = self.delete(r, c).det();
                if (r + c) % 2 == 0 { f } else { -f }
            }

            pub fn det(&self) -> f64 {
                (0..self.d).map(|c| self.data[0][c] * self.cofactor(0, c)).sum()
            }

            pub fn inverse(&self) -> Option<Self> {
                if self.det() == 0.0 {
                    None
                } else {
                    let det = self.det();

                    let mut res = self.clone();
                    for r in 0..self.d {
                        for c in 0..self.d {
                            res.data[r][c] = self.cofactor(r, c);
                        }
                    }
                    let res = res.transposed();

                    Some(res / det)
                }
            }
        }
    };

    // (Internal match) Matrix additional implementation (base case)
    // - `matrix`: Matrix type name
    (@matrix_det $matrix:tt) => {
        impl $matrix {
            pub fn det(&self) -> f64 {
                self.data[0][0]
            }

            // pub fn inverse(&self) -> Option<Self> {
            //     let det = self.det();
            //
            //     (det != 0.0).then_some(self.clone() / det)
            // }
        }
    };

    // Matrix definition and implementation
    // - `matrix`: Matrix type name
    // - `D`: Number of rows/columns
    // - `sub_matrix`: Sub-matrix type name
    ($matrix:tt, $D:tt, $sub_matrix:tt) => {
        matrix!(@matrix_def $matrix, $D);
        matrix!(@matrix_det $matrix, $sub_matrix);
    };

    // Matrix definition and implementation (base case)
    // - `matrix`: Matrix type name
    // - `D`: Number of rows/columns
    ($matrix:tt, $D:tt) => {
        matrix!(@matrix_def $matrix, $D);
        matrix!(@matrix_det $matrix);
    }
}

matrix!(Matrix1, 1);
matrix!(Matrix2, 2, Matrix1);
matrix!(Matrix3, 3, Matrix2);
matrix!(Matrix4, 4, Matrix3);
