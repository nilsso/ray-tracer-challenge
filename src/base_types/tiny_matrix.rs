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
        // D: Number of columns/rows in O
        ($A:tt, $B:tt, $N:tt),
        ($O:tt, $D:tt)
    ) => {
        impl Mul<$B> for $A {
            type Output = $O;

            fn mul(self, rhs: $B) -> Self::Output {
                let mut res = $O::zero();

                for r in 0..$D {
                    for c in 0..$D {
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
    ($A:tt, $D:tt) => {
        matrix_mul!(($A, $A, $D), ($A, $D));
    };
}
