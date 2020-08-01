macro_rules! matrix {
    (@matrix_mn $name:tt, $R:tt, $C:tt, $N:expr) => {
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct $name {
            data: [f64; $N],
        }

        impl $name {
            pub const fn new(data: [f64; $N]) -> Self {
                Self { data }
            }

            pub const fn zero() -> Self {
                Self::new([0.0; $N])
            }

            pub const fn one() -> Self {
                Self::new([1.0; $N])
            }

            const fn index(&self, r: usize, c: usize) -> usize {
                c + r * $C
            }

            pub fn ident() -> Self {
                let mut m = Self::zero();
                for i in 0..$R.min($C) {
                    m.data[m.index(i, i)] = 1.0;
                }

                m
            }

            pub fn dim(&self) -> (usize, usize) {
                ($R, $C)
            }

            pub fn len(&self) -> usize {
                $N
            }

            pub fn row_sums(&self) -> [f64; $R] {
                let mut res = [0.0; $R];
                for r in 0..$R {
                    for c in 0..$C {
                        res[r] += self.data[self.index(r, c)];
                    }
                }
                res
            }

            pub fn col_sums(&self) -> [f64; $C] {
                let mut res = [0.0; $C];
                for r in 0..$R {
                    for c in 0..$C {
                        res[c] += self.data[self.index(r, c)];
                    }
                }
                res
            }
        }

        impl Add for $name {
            type Output = $name;

            fn add(self, mut rhs: $name) -> Self::Output {
                for i in 0..$N {
                    rhs.data[i] += self.data[i];
                }
                rhs
            }
        }
    };

    ($name:tt, $R:tt, $C:tt) => {
        matrix!(@matrix_mn $name, $R, $C, $R * $C);
    };

    // name: Matrix item
    // D: Number of rows/columns for the matrix
    ($name:tt, $D:tt) => {
        matrix!($name, $D, $D);
        matrix_mul!($name, $D);
    };
}

macro_rules! matrix_mul {
    (
        // A: Left matrix
        // B: Right matrix
        // N: Number of columns in A/number of rows in B
        // O: Output matrix
        // R: Number of rows in O
        // C: Number of rows in C
        ($A:tt, $B:tt, $N:tt),
        ($O:tt, $R:tt, $C:tt)
    ) => {
        impl Mul<$B> for $A {
            type Output = $O;

            fn mul(self, rhs: $B) -> Self::Output {
                let mut res = $O::zero();

                for r in 0..$R {
                    for c in 0..$C {
                        let i = res.index(r, c);

                        for j in 0..$N {
                            let a_i = self.index(r, j);
                            let b_i = rhs.index(j, c);

                            res.data[i] += (self.data[a_i] * rhs.data[b_i]);
                        }
                    }
                }

                res
            }
        }
    };
    (
        ($A:tt, $B:tt, $N:tt),
        ($O:tt, $D:tt)
    ) => {
        matrix_mul!(($A, $B, $N), ($O, $D, $D));
    };
    ($A:tt, $D:tt) => {
        matrix_mul!(($A, $A, $D), ($A, $D));
    };
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Mul};

    matrix!(Matrix1, 1);
    matrix!(Matrix2, 2);
    matrix!(Matrix3, 3);
    matrix!(Matrix4, 4);

    matrix!(Matrix1x2, 1, 2);
    matrix!(Matrix1x3, 1, 3);
    matrix!(Matrix1x4, 1, 4);

    matrix!(Matrix2x1, 2, 1);
    matrix!(Matrix2x3, 2, 3);
    matrix!(Matrix2x4, 2, 4);

    matrix!(Matrix3x1, 3, 1);
    matrix!(Matrix3x2, 3, 2);
    matrix!(Matrix3x4, 3, 4);

    matrix!(Matrix4x1, 4, 1);
    matrix!(Matrix4x2, 4, 2);
    matrix!(Matrix4x3, 4, 3);

    matrix_mul!((Matrix1x4, Matrix4x1, 4), (Matrix1, 1));
    matrix_mul!((Matrix2x4, Matrix4x1, 4), (Matrix2x1, 2, 1));
    matrix_mul!((Matrix3x4, Matrix4x1, 4), (Matrix3x1, 3, 1));

    matrix_mul!((Matrix1x4, Matrix4x2, 4), (Matrix1x2, 1, 2));

    //#[test]
    //fn matrix_1x1_operations() {
    //const A: Matrix1 = Matrix1::new([2.0]);
    //const B: Matrix1 = Matrix1::new([3.0]);
    //{
    //const R: Matrix1 = Matrix1::new([6.0]);
    //assert_eq!(A * B, R);
    //}
    //{
    //const R: Matrix1 = Matrix1::new([5.0]);
    //assert_eq!(A + B, R);
    //}
    //}

    fn matrix_4x4_operations() {}

    #[test]
    fn multiply_1x4_and_4x1_matrices() {
        const A: Matrix1x4 = Matrix1x4::new([
            1.0, 2.0, 3.0, 4.0, //
        ]);
        const B: Matrix4x1 = Matrix4x1::new([
            1.0, //
            2.0, //
            3.0, //
            4.0, //
        ]);
        const R: Matrix1 = Matrix1::new([
            1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0, //
        ]);

        assert_eq!(A * B, R);
    }

    #[test]
    fn multiply_2x4_and_4x1_matrices() {
        const A: Matrix2x4 = Matrix2x4::new([
            1.0, 2.0, 3.0, 4.0, //
            5.0, 6.0, 7.0, 8.0, //
        ]);
        const B: Matrix4x1 = Matrix4x1::new([
            1.0, //
            2.0, //
            3.0, //
            4.0, //
        ]);
        const R: Matrix2x1 = Matrix2x1::new([
            1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0, //
            5.0 * 1.0 + 6.0 * 2.0 + 7.0 * 3.0 + 8.0 * 4.0, //
        ]);

        assert_eq!(A * B, R);
    }

    #[test]
    fn multiply_3x4_and_4x1_matrices() {
        const A: Matrix3x4 = Matrix3x4::new([
            1.0, 2.0, 3.0, 4.0, //
            5.0, 6.0, 7.0, 8.0, //
            9.0, 10.0, 11.0, 12.0, //
        ]);
        const B: Matrix4x1 = Matrix4x1::new([
            1.0, //
            2.0, //
            3.0, //
            4.0, //
        ]);
        const R: Matrix3x1 = Matrix3x1::new([
            1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0,    //
            5.0 * 1.0 + 6.0 * 2.0 + 7.0 * 3.0 + 8.0 * 4.0,    //
            9.0 * 1.0 + 10.0 * 2.0 + 11.0 * 3.0 + 12.0 * 4.0, //
        ]);

        assert_eq!(A * B, R);
    }

    #[test]
    fn multiply_1x4_and_4x2_matrices() {
        const A: Matrix1x4 = Matrix1x4::new([
            1.0, 2.0, 3.0, 4.0, //
        ]);
        const B: Matrix4x2 = Matrix4x2::new([
            1.0, 2.0, //
            3.0, 4.0, //
            5.0, 6.0, //
            7.0, 8.0, //
        ]);
        const R: Matrix1x2 = Matrix1x2::new([
            1.0 * 1.0 + 2.0 * 3.0 + 3.0 * 5.0 + 4.0 * 7.0, //
            1.0 * 2.0 + 2.0 * 4.0 + 3.0 * 6.0 + 4.0 * 8.0, //
        ]);

        assert_eq!(A * B, R);
    }
}
