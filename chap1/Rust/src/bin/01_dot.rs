use std::ops::{Add, Mul};

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    println!("{}", dot(&a, &b));
    println!("{}", dot_fp(&a, &b));
}

fn dot<T: Default + Add<Output = T> + Mul<Output = T> + Copy>(x: &Vec<T>, y: &Vec<T>) -> T {
    assert_eq!(x.len(), y.len());
    let mut c = T::default();
    for i in 0..x.len() {
        c = c + x[i] * y[i];
    }
    c
}

fn dot_fp<T: Default + Add<Output = T> + Mul<Output = T> + Copy>(x: &Vec<T>, y: &Vec<T>) -> T {
    assert_eq!(x.len(), y.len());
    x.iter().zip(y.iter()).fold(T::default(), |c, (&a, &b)| c + a * b)
}
