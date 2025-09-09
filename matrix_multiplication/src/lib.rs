#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
    todo!()
}

#[cfg(test)]
mod tests;
