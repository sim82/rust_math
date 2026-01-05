use std::f64::consts::PI;

use ::num_complex::Complex;

fn tailor(i: u64, t: f64) -> Complex<f64> {
    let pi_ifac = Complex::new(t, 0.0) * Complex::new(0.0, 1.0);

    match i {
        0 => Complex::new(1.0, 0.0),
        1 => pi_ifac,
        _ => {
            let ifac = faculty(i) as f64;
            let t1 = Complex::new(1.0 / ifac, 0.0);
            t1 * ipow(pi_ifac, i)
        }
    }
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

    let mut t = Complex::new(0.0, 0.0);
    for i in 0..20 {
        t += tailor(i, PI / 1.0);
        println!("t{i}: {t}");
    }
    println!("{}", t);
}
