/* INFO DFT - my implementation of the Discrete Fourier Transform
* NOTE the definition of the DFT is X[k] = Σ^N-1_n=0 x[n] e^(-i 2π/N kn) */
use std::{f64::consts::PI, task::RawWakerVTable};

fn e_power_i_pi_x(input: f64) -> (f64, f64) {
    ((input * PI).cos(), (input * PI).sin())
}

// we have an array of vectors, or better said, complex numbers as our inputs and a similar array in the output
// that means that for the input we say that it is a vector *(practically a resizable array) of a tuple of two f32s *(the complex number)

fn dft(input: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut output: Vec<(f64, f64)> = vec![(0.0, 0.0); input.len()];

    let n_big = input.len() as f64; // sample num

    for k in 0..input.len() {
        let mut sum: (f64, f64) = (0., 0.);
        let mut n = 0;
        for xn in &input {
            let (cos_term, i_sin_term) = e_power_i_pi_x(-2. * k as f64 * n as f64 * 1. / n_big);
            sum.0 += xn.0 * cos_term - xn.1 * i_sin_term;
            sum.1 += xn.0 * i_sin_term + xn.1 * cos_term;

            n += 1;
        }
        output[k] = sum;
    }
    output
}

fn main() {
    let result = dft(vec![(1., 0.), (1., 0.), (1., 0.), (1., 0.)]);
    for (k, (real, imag)) in result.iter().enumerate() {
        // INFO fixing artifacts
        let mut real = (real * 32.).round() / 32.;
        let mut imag = (imag * 32.).round() / 32.;

        if real == -0 as f64 {
            real = 0.;
        } else if imag == -0 as f64 {
            imag = 0.;
        }

        println!("{} + i{}", real, imag);
    }
}
