use std::{fs::write, io::Error};

use raytracer::{
    math::{Point, Vector},
    Canvas, Colour,
};

#[derive(Clone, Copy, Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }

    pub fn update(&self, projectile: Projectile) -> Projectile {
        Projectile::new(
            projectile.position + projectile.velocity,
            projectile.velocity + self.gravity + self.wind,
        )
    }
}

fn main() -> Result<(), Error> {
    let width = 900;
    let height = 550;
    let mut canvas = Canvas::new(width, height);

    let mut projectile = Projectile::new(
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 1.8, 0.0).normalise() * 11.25,
    );
    let environment = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    while projectile.position.y >= 0.0 {
        let x = projectile.position.x as i32;
        let y = height as i32 - projectile.position.y as i32;

        if (0..width as i32).contains(&x) && (0..height as i32).contains(&y) {
            canvas.write_pixel(x as usize, y as usize, Colour::white());
        }

        projectile = environment.update(projectile);
    }

    write("image.ppm", canvas.to_ppm())
}
