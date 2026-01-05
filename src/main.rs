use ::num_complex::Complex;
use std::{f64::consts::PI, mem::zeroed};

fn tailor_n(i: u64, s: Complex<f64>, t: f64) -> Complex<f64> {
    // let pi_ifac = Complex::new(t, 0.0) * Complex::new(0.0, 1.0);

    let st = s * Complex::new(t, 0.0);
    match i {
        0 => Complex::new(1.0, 0.0),
        1 => st,
        _ => {
            let ifac = faculty(i) as f64;
            let t1 = Complex::new(1.0 / ifac, 0.0);
            t1 * ipow(st, i)
        }
    }
}

fn tailor(n: u64, s: Complex<f64>, t: f64) -> Complex<f64> {
    let mut acc = Complex::<f64>::ZERO;
    for i in 0..n {
        acc += tailor_n(i, s, t);
        println!("t{i}: {acc}");
    }
    acc
}

fn ipow(a: Complex<f64>, i: u64) -> Complex<f64> {
    match i {
        0 => Complex::new(1.0, 0.0),
        1 => a,
        _ => {
            let mut b = a;
            for _ in 1..i {
                b *= a
            }
            b
        }
    }
}
fn faculty(i: u64) -> u64 {
    match i {
        0 | 1 => 1,
        _ => {
            let mut f = 1;
            for j in 1..=i {
                f *= j
            }
            f
        }
    }
}
fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("{}", x.exp());
    let t1 = Complex::new(PI, 0.0) * Complex::new(0.0, 1.0);
    println!("{}", t1);

    let t = tailor(20, Complex::new(0.0, 1.0), PI);
    println!("{}", t);
}
