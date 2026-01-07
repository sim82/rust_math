use ::num_complex::Complex;
use std::f64::consts::PI;

fn tailor(n: u64, s: Complex<f64>, t: f64) -> Complex<f64> {
    let mut acc = Complex::new(1.0, 0.0);
    let mut fac = 1.0f64;
    let st = s * Complex::new(t, 0.0);
    let mut pow = st;
    for i in 1..n {
        match i {
            1 => acc += st,
            _ => {
                fac *= i as f64;
                pow *= st;
                acc += (1.0 / fac) * pow;
            }
        }
        println!("t{i}: {fac} {acc}");
    }
    acc
}

fn main() {
    let x = Complex::new(0.0, 2.0 * PI);

    println!("{}", x.exp());
    let t1 = Complex::new(PI, 0.0) * Complex::new(0.0, 1.0);
    println!("{}", t1);

    let t = tailor(20, Complex::new(1.0, 0.0), 1.0);
    println!("{}", t);
}
