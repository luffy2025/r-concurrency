use anyhow::Result;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[allow(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
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

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + MulAssign,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("invalid matrix size"));
    }

    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
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
        let r = multiply(&a, &b)?;
        assert_eq!(r.col, 2);
        assert_eq!(r.row, 2);
        assert_eq!(r.data, vec![22, 28, 49, 64]);
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
