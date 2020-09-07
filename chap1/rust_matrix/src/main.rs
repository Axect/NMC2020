#![feature(trait_alias)]
use std::ops::{Add, Sub, Mul, Index, IndexMut};

fn main() {
    let mut a = Matrix::new(vec![vec![1,2],vec![3,4]]);
    println!("{:?}", a);
    println!("{}", a[(1, 0)]);
    a[(1, 0)] = 5;
    println!("{:?}", a);
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Matrix {
            data
        }
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
        for i in 0 .. r {
            for j in 0 .. c {
                result[(i, j)] = result[(i, j)] + rhs[(i, j)];
            }
        }
        result
    }
}

pub trait AddNum = Copy + Clone + Add<Output = Self>;
pub trait SubNum = Copy + Clone + Sub<Output = Self>;
pub trait MulNum = Copy + Clone + Mul<Output = Self>;

impl<T: SubNum> Sub<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let r = self.nrow();
        let c = self.ncol();
        assert_eq!(r, rhs.nrow());
        assert_eq!(c, rhs.ncol());

        let mut result = self.clone();
        for i in 0 .. r {
            for j in 0 .. c {
                result[(i, j)] = result[(i, j)] - rhs[(i, j)];
            }
        }
        result
    }
}

impl<T: MulNum> Mul<Matrix<T>> for Matrix<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
