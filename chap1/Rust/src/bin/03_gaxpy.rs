#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

#[allow(non_snake_case)]
fn main() {
    let A = ml_matrix("1 2;3 4");
    let x = c!(1,2);
    let mut y = c!(0,0);
    gaxpy(&A, &x, &mut y);
    y.print();
}

#[allow(non_snake_case)]
fn gaxpy(A: &Matrix, x: &Vec<f64>, y: &mut Vec<f64>) {
    let m = A.row;
    let n = A.col;
    assert!(m == y.len() && n == x.len());

    for i in 0 .. m {
        for j in 0 .. n {
            y[i] += A[(i, j)] * x[j];
        }
    }
}
