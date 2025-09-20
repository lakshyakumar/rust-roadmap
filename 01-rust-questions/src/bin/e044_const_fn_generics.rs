// 44. How do you use const fn and const generics to implement a fixed-size matrix type FixedMatrix<T, const N: usize>?
// Implement an identity() method as a const fn. Why are const generics powerful?

// Const generics allow us to parameterize types not just by types but by values known at compile time.
// For example, instead of a Vec<Vec<T>>, we can say:
// struct FixedMatrix<T, const N: usize> {
//     data: [[T; N]; N], // N×N matrix
// }
// This means FixedMatrix<f64, 4> is a 4×4 matrix of f64, and its size is guaranteed at compile time.

// #![feature(const_mut_refs)] // needed for mutation inside const fn

#[derive(Debug, Clone, Copy)]
struct FixedMatrix<T, const N: usize> {
    data: [[T; N]; N],
}

impl<const N: usize> FixedMatrix<i32, N> {
    /// Create an identity matrix as a const fn
    pub const fn identity() -> Self {
        let mut data = [[0; N]; N]; // initialize with zeros
        let mut i = 0;
        while i < N {
            data[i][i] = 1; // set diagonal elements
            i += 1;
        }
        Self { data }
    }
}

fn main() {
    // Identity matrix created at compile time
    const I3: FixedMatrix<i32, 3> = FixedMatrix::identity();
    const I4: FixedMatrix<i32, 4> = FixedMatrix::identity();

    println!("3x3 Identity Matrix: {:?}", I3);
    println!("4x4 Identity Matrix: {:?}", I4);
}
