mod complex;
use complex::Complex;

/// calculate the inner product of two complex vectors
/// the calculation is the standard inner product over the complex field
fn inner(a: &[Complex], b: &[Complex]) -> Result<Complex, &'static str> {
    if a.len() != b.len() {
        return Err("Lengths do not match");
    }

    let mut result = Complex::new(0.0, 0.0);
    for i in 0..a.len() {
        result = result + (a[i].conjugate() * b[i]);
    }
    Ok(result)
}

/// Perform a * b, where a is a scalar complex and b is a complex vector
fn vmuls(a: Complex, b: Vec<Complex>) -> Vec<Complex> {
    b.into_iter()
    .map(|c| c * a)
    .collect()
}

/// Lanewise vector addition
fn vaddv(a: Vec<Complex>, b: Vec<Complex>) -> Vec<Complex> {
    let mut res = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        res.push(a[i] + b[i]);
    }
    res
}

fn main() {
    let u1 = vec![
        3.into(),
        Complex::new(0.0, 2.0),
        Complex::new(0.0, 1.0),
        1.into()
    ];

    let u2 = vec! [
        Complex::new(0.0, 2.0),
        1.into(),
        1.into(),
        Complex::new(0.0, -3.0)
    ];

    let v = vec![
        2.into(),
        Complex::new(0.0, -7.0),
        Complex::new(0.0, -1.0),
        Complex::new(-6.0, 0.0)
    ];

    println!("<u1, u1>: {}", inner(&u1, &u1).unwrap());
    println!("<u2, u2>: {}", inner(&u2, &u2).unwrap());
    println!("<u1, v>: {}", inner(&u1, &v).unwrap());
    println!("<v, u1>: {}", inner(&v, &u1).unwrap());
    println!("<u2, v>: {}", inner(&u2, &v).unwrap());
    println!("<v, u2>: {}", inner(&v, &u2).unwrap());

    let _u1 = vmuls(Complex::new(-1.0, 0.0), u1);
    let _u2 = vmuls(Complex::new(0.0, -2.0), u2);
    let vprime = vaddv(_u1, _u2);
    println!("v': {:?}", vprime);

}
