use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Clone, Copy)]
pub enum CalcResult {
    Ok(f64),
    SyntaxError,
    MathError,
}

impl Add for CalcResult {
    type Output = CalcResult;
    fn add(self, rhs: Self) -> Self::Output {
        if let CalcResult::Ok(n1) = self {
            if let CalcResult::Ok(n2) = rhs {
                Self::Ok(n1 + n2)
            } else {
                rhs
            }
        } else {
            self
        }
    }
}

impl Sub for CalcResult {
    type Output = CalcResult;
    fn sub(self, rhs: Self) -> Self::Output {
        if let CalcResult::Ok(n1) = self {
            if let CalcResult::Ok(n2) = rhs {
                Self::Ok(n1 - n2)
            } else {
                rhs
            }
        } else {
            self
        }
    }
}

impl Mul for CalcResult {
    type Output = CalcResult;
    fn mul(self, rhs: Self) -> Self::Output {
        if let CalcResult::Ok(n1) = self {
            if let CalcResult::Ok(n2) = rhs {
                Self::Ok(n1 * n2)
            } else {
                rhs
            }
        } else {
            self
        }
    }
}

impl Div for CalcResult {
    type Output = CalcResult;
    fn div(self, rhs: Self) -> Self::Output {
        if let CalcResult::Ok(n1) = self {
            if let CalcResult::Ok(n2) = rhs {
                Self::Ok(n1 / n2)
            } else {
                rhs
            }
        } else {
            self
        }
    }
}

impl CalcResult {
    pub fn pow(self, rhs: Self) -> Self {
        if let CalcResult::Ok(n1) = self {
            if let CalcResult::Ok(n2) = rhs {
                Self::Ok(n1.powf(n2))
            } else {
                rhs
            }
        } else {
            self
        }
    }
}
