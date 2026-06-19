#[derive(PartialEq)]
pub enum CalcResult{
    Ok(f64),
    SyntaxError,
    MathError
}