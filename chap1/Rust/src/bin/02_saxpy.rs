use std::ops::{Add, Mul};

fn main() {
    let x = vec![1,2,3];
    let mut y = vec![4,5,6];
    saxpy(2, &x, &mut y);
    println!("{:?}", y);
}

fn saxpy<T: Add<Output=T> + Mul<Output=T> + Copy>(a: T, x: &Vec<T>, y: &mut Vec<T>) {
    assert_eq!(x.len(), y.len());
    for i in 0 .. y.len() {
        y[i] = y[i] + a * x[i];
    }
}
