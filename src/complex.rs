use std::ops::{Add, Div, Mul};
use std::fmt::Display;
#[derive(Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(a: f64, b:f64) -> Complex {
        Complex {
            real: a,
            imag: b
        }
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -1.0 * self.imag
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.real.abs() != 0.0 {
            write!(f, "{}", self.real).unwrap();
            if self.imag.abs() != 0.0 {
                if self.imag < 0.0 {
                    write!(f, " - {}i", self.imag.abs()).unwrap();
                } else {
                    write!(f, " + {}i", self.imag).unwrap();
                }
            }

        } else {
            return write!(f, "{}i", self.imag);
        }

        Ok(())
    }
}

impl std::fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       Display::fmt(&self, f)
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        let real_part = (self.real * rhs.real) + (-1.0 * self.imag * rhs.imag);
        let imag_part = (self.real * rhs.imag) + (self.imag * rhs.real);
        Complex {
            real: real_part,
            imag: imag_part
        }
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        let denom = (rhs.real * rhs.real) + (rhs.imag * rhs.imag);
        let to_div = self * rhs.conjugate();
        Complex {
            real: to_div.real / denom,
            imag: to_div.imag / denom
        }
    }
}

impl Into<Complex> for i64 {
    fn into(self) -> Complex {
        Complex {
            real: self as f64,
            imag: 0.0
        }
    }
}