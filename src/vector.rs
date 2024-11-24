use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul, MulAssign};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + MulAssign,
{
    let mut r = T::default();
    for i in 0..a.len() {
        r += a[i] * b[i];
    }
    Ok(r)
}
