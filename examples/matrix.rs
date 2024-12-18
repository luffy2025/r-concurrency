use anyhow::Result;
use r_concurrency::Matrix;

fn main() -> Result<()> {
    let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    let r = a * b;
    println!("{:?}", r);
    Ok(())
}
