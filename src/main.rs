use raytracer::math::{Point, Vector};

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

fn main() {
    let mut projectile =
        Projectile::new(Point::new(0.0, 1.0, 0.0), Vector::new(1.0, 1.0, 0.0));

    let environment = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    let mut tick = 0;

    println!("Initial projectile {projectile:?}");

    while projectile.position.y >= 0.0 {
        projectile = environment.update(projectile);
        tick += 1;

        println!("Projectile at tick {tick:?} is {projectile:?}");
    }

    println!("Projectile landed in {tick:?} ticks")
}
