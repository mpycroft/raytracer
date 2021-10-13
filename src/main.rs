use raytracer::{
    math::{Point, Vector},
    Canvas, Colour,
};
use std::fs::write;

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    pub fn update(&mut self, environment: &Environment) {
        self.position += self.velocity;
        self.velocity += environment.gravity + environment.wind;
    }
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }
}

fn main() {
    let width: i32 = 900;
    let height: i32 = 500;
    let mut canvas = Canvas::new(width as usize, height as usize);

    let mut proj = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalise() * 11.25,
    );

    let env = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    println!("Initial projectile {:?}, environment {:?}", proj, env);

    let mut tick: u32 = 0;

    while proj.position.y >= 0.0 {
        let x = proj.position.x as i32;
        let y = height - proj.position.y as i32;

        if (0..width).contains(&x) && (0..height).contains(&y) {
            canvas.write_pixel(
                x as usize,
                y as usize,
                Colour::new(1.0, 1.0, 1.0),
            );
        }

        proj.update(&env);

        tick += 1;
    }

    println!("\nProjectile landed in {} ticks", tick);

    write("image.ppm", canvas.to_ppm()).unwrap()
}
