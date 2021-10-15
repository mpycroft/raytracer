use raytracer::math::{Matrix, Point};

fn main() {
    let mut id = Matrix::identity();
    println!("{:?}", id.invert().unwrap());

    let m = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [2.5, 0.5, 3.5, 1.5],
        [2.0, -1.0, -2.0, -3.0],
        [5.0, 6.0, 7.0, 8.0],
    ]);
    println!("{:?}", m * m.invert().unwrap());

    println!("{:?}", m.transpose().invert().unwrap());
    println!("{:?}", m.invert().unwrap().transpose());

    id[1][2] = 3.0;

    println!("{:?}", id * Point::new(1.0, 2.0, 3.0));
}
