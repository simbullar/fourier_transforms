/* INFO DFT - my implementation of the Discrete Fourier Transform
* NOTE the definition of the DFT is X[k] = Σ^N-1_n=0 x[n] e^(-i 2π/N kn) */
use std::f64::consts::PI;

pub fn e_power_i_pi_x(input: f64) -> (f64, f64) {
    // simple helper function, i guess
    ((input * PI).cos(), (input * PI).sin())
}

// we have an array of vectors, or better said, complex numbers as our inputs and a similar array in the output
// that means that for the input we say that it is a vector *(practically a resizable array) of a tuple of two f32s *(the complex number)

pub fn dft(input: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let mut output: Vec<(f64, f64)> = vec![(0.0, 0.0); input.len()];

    let n_big = input.len() as f64; // sample number

    for k in 0..input.len() {
        let mut sum: (f64, f64) = (0., 0.); // output for this iteration
        let mut n = 0;
        for xn in &input {
            let (cos_term, i_sin_term) = e_power_i_pi_x(-2. * k as f64 * n as f64 * 1. / n_big);
            sum.0 += xn.0 * cos_term - xn.1 * i_sin_term;
            sum.1 += xn.0 * i_sin_term + xn.1 * cos_term;

            n += 1;
        }

        sum.0 = (sum.0 * 32.).round() / 32.;
        sum.1 = (sum.1 * 32.).round() / 32.;

        output[k] = sum;
    }
    output
}

pub fn dft_2d(input: Vec<Vec<(f64, f64)>>) -> Vec<Vec<(f64, f64)>> {
    let rows = input.len();
    let cols = input[0].len(); // assume that all of the collumns are the same size

    let mut output: Vec<Vec<(f64, f64)>> = vec![vec![(0., 0.); cols]; rows];

    for y in 0..rows {
        let row_dft = dft(input[y].clone());
        for x in 0..cols {
            output[y][x] = row_dft[x];
        }
    }

    for x in 0..cols {
        let mut column: Vec<(f64, f64)> = (0..rows).map(|y| output[y][x]).collect();
        let cols_dft = dft(column);
        for y in 0..rows {
            output[x][y] = cols_dft[y];
        }
    }

    output
}

fn main() {
    let input = vec![(1., 0.), (1., 0.), (2., 0.), (1., 0.)];
    let result = dft(input);
    for (_k, (real, imag)) in result.iter().enumerate() {
        let mut real = *real;
        let mut imag = *imag;

        // INFO This is optional and only for output clarity
        if real == -0 as f64 {
            real = 0.;
        }
        if imag == -0 as f64 {
            imag = 0.;
        }

        println!("{} + i{}", real, imag);
    }
}
