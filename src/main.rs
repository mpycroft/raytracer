use raytracer::math::{Point, Vector};

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
    let mut proj =
        Projectile::new(Point::new(0.0, 1.0, 0.0), Vector::new(1.0, 1.0, 0.0));

    let env = Environment::new(
        Vector::new(0.0, -0.1, 0.0),
        Vector::new(-0.01, 0.0, 0.0),
    );

    let mut tick: u32 = 0;

    println!("Initial projectile {:?}, environment {:?}", proj, env);

    while proj.position.y >= 0.0 {
        proj.update(&env);

        tick += 1;

        println!("Tick {}, projectile {:?}", tick, proj);
    }

    println!("\nProjectile landed in {} ticks", tick);
}
