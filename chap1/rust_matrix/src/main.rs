#![feature(trait_alias)]
extern crate matrixmultiply;
use matrixmultiply::dgemm;

use std::ops::{Add, Index, IndexMut, Mul, Sub, Div};
use std::env::args;

pub trait AddNum = Copy + Clone + Add<Output = Self>;
pub trait SubNum = Copy + Clone + Sub<Output = Self>;
pub trait Num = Copy 
    + Clone
    + Default
    + Add<Output = Self> 
    + Sub<Output = Self> 
    + Mul<Output = Self>
    + Div<Output = Self>;

fn main() {
    let args: Vec<String> = args().collect();
    let row: usize = args[1].parse().unwrap();
    let col: usize = args[2].parse().unwrap();

    let a: Matrix<f64> = zeros(row, col);
    let b: Matrix<f64> = zeros(row, col);
    //let c = a * b;
    let c = matmul(&a, &b);
    println!("{}", c[(row/2, col/2)]);
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix { data }
    }

    pub fn nrow(&self) -> usize {
        self.data.len()
    }

    pub fn ncol(&self) -> usize {
        self.data[0].len()
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i][j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[i][j]
    }
}

impl<T: Copy + Clone + Add<Output = T>> Add<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.nrow();
        let c = self.ncol();
        assert_eq!(r, rhs.nrow());
        assert_eq!(c, rhs.ncol());

        let mut result = self.clone();
        for i in 0..r {
            for j in 0..c {
                result[(i, j)] = result[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

impl<T: SubNum> Sub<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.nrow();
        let c = self.ncol();
        assert_eq!(r, rhs.nrow());
        assert_eq!(c, rhs.ncol());

        let mut result = self.clone();
        for i in 0..r {
            for j in 0..c {
                result[(i, j)] = result[(i, j)] - rhs[(i, j)];
            }
        }
        result
    }
}

/// Basic matrix multiplication
impl<T: Num> Mul<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.nrow();
        let c = rhs.ncol();
        let n = self.ncol();
        assert_eq!(n, rhs.nrow());

        let mut result = Matrix::new(vec![vec![T::default(); c]; r]);

        for i in 0..r {
            for j in 0..c {
                let mut s = T::default();
                for k in 0..n {
                    s = s + self[(i, k)] * rhs[(k, j)];
                }
                result[(i, j)] = s;
            }
        }
        result
    }
}

/// Zeros
pub fn zeros<T: Default + Copy + Clone>(r: usize, c: usize) -> Matrix<T> {
    Matrix::new(vec![vec![T::default(); c]; r])
}

pub fn matmul(a: &Matrix<f64>, b: &Matrix<f64>) -> Matrix<f64> {
    let m = a.nrow();
    let k = a.ncol();
    let n = b.ncol();
    let (rsa, csa) = (k as isize, 1isize);
    let (rsb, csb) = (n as isize, 1isize);
    let (rsc, csc) = (n as isize, 1isize);

    let a_flat: Vec<f64> = a.data.clone().into_iter().flatten().collect();
    let b_flat: Vec<f64> = b.data.clone().into_iter().flatten().collect();
    let mut c_flat: Vec<f64> = vec![0f64; m * n];

    unsafe {
        matrixmultiply::dgemm(
            m,
            k,
            n,
            1f64,
            a_flat.as_ptr(),
            rsa,
            csa,
            b_flat.as_ptr(),
            rsb,
            csb,
            0f64,
            c_flat.as_mut_ptr(),
            rsc,
            csc,
        )
    }

    let mut c_data: Vec<Vec<f64>> = vec![vec![0f64; n]; m];
    for i in 0 .. m {
        c_data[i] = c_flat[n * i .. n * (i+1)].to_vec();
    }
    Matrix::new(c_data)
}
