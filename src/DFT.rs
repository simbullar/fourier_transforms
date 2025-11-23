/* INFO DFT - my implementation of the Discrete Fourier Transform
* the definition of the DFT is X[k] = Σ^N-1_n=0 x[n] e^(-i 2π/N kn) */

// we have an array of vectors, or better said, complex numbers as our inputs and a similar array in the output
// that means that for the input we say that it is a vector *(practically a resizable array) of a tuple of two f32s *(the complex number)
fn dft(input: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut output: Vec<(f32, f32)> = vec![];
    let mut i = 0;
    for xn in input {
        output[i] = xn; //TODO * e^(-i 2π )^(1/N kn)
        i += 1;
    }
    output
}

fn main() {
    dft(vec![(1., 0.), (1., 0.), (1., 0.), (1., 0.)]);
}
