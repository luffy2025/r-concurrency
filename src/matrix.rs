use crate::vector::{dot_product, Vector};
use anyhow::Result;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::sync::mpsc;
use std::{fmt, thread};

const N: usize = 4;

#[allow(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    v: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Mul for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + MulAssign + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("matrix multiply error")
    }
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j < self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + MulAssign + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("invalid matrix size"));
    }

    let senders = (0..N)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let v = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        v,
                    }) {
                        eprintln!("send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
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
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            receivers.push(rx);
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % N].send(msg) {
                eprintln!("send error: {:?}", e);
            }
        }
    }
    for rx in receivers {
        let r = rx.recv()?;
        data[r.idx] = r.v;
    }

    Ok(Matrix::new(data, a.row, b.col))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let r = a * b;
        assert_eq!(r.col, 2);
        assert_eq!(r.row, 2);
        assert_eq!(r.data, vec![22, 28, 49, 64]);
        Ok(())
    }

    #[test]
    fn test_a_can_not_multiply_b() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let r = multiply(&a, &b);
        assert!(r.is_err());
        Ok(())
    }

    #[test]
    fn test_matrix_display() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(format!("{}", b), "{1 2, 3 4, 5 6}");
    }
}
