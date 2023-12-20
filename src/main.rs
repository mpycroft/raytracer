// Ignore pedantic lints in our temp binary code until we actually start writing
// real raytracer code here.
#![allow(clippy::pedantic)]

use raytracer::math::{matrix::Matrix, Vector};

fn main() {
    println!("{:?}", Matrix::<4>::identity().invert().unwrap());

    let m = Matrix([
        [1.0, 2.0, 3.0, 4.0],
        [0.0, -2.0, -3.0, 0.0],
        [-2.0, 1.0, 0.0, 1.0],
        [-1.0, 2.0, 0.0, 1.0],
    ]);

    println!("{:?}", m * m.invert().unwrap());

    println!("{:?}", m.transpose().invert().unwrap());
    println!("{:?}", m.invert().unwrap().transpose());

    let mut id = Matrix::identity();
    id[0][1] = 2.0;

    println!("{:?}", id * Vector::new(1.0, 2.0, 3.0));
}
