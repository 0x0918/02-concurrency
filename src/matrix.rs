use core::fmt;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use anyhow::{anyhow, Error, Result};
use oneshot::{channel, Sender};

const NUM_THREADS: usize = 4;

use crate::vector::{dot_product, Vector};
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: Sender<MsgOutput<T>>,
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Add<Output = T> + Copy + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.col"));
    }
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let index = i * b.col + j;
            let input = MsgInput::new(index, row, col);
            let (tx, rx) = channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[index % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + AddAssign + Add<Output = T> + Mul<Output = T> + Send + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}
impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix(row = {}, col = {}, {})",
            self.row, self.col, self
        )
    }
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::Matrix;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(c.col, 2);
        assert_eq!(c.row, 2);
        assert_eq!(c.data, vec![22, 28, 49, 64]);
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row = 2, col = 2, {22 28, 49 64})"
        );
        Ok(())
    }
}
