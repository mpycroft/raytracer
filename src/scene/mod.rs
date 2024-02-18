mod add;

use crate::{Camera, Light};

/// The `Data` struct holds the information for the scene as we parse it.
#[derive(Clone, Debug)]
struct Data {
    pub camera: Option<Camera>,
    pub lights: Vec<Light>,
}

impl Data {
    pub fn new() -> Self {
        Self { camera: None, lights: Vec::new() }
    }
}
