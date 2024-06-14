use std::{
    ops::{Add, AddAssign, Deref, Index, Mul},
    slice::Iter,
};

use anyhow::{anyhow, Result};

pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("Dot product error: a.len != b.len"));
    }

    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
