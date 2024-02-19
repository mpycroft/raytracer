mod add;
mod define;
mod list;

use std::collections::HashMap;

use serde_yaml::Value;

use self::{add::Add, define::Define};
use crate::{Camera, Light};

/// The `Data` struct holds the information for the scene as we parse it.
#[derive(Clone, Debug)]
struct Data {
    shapes: HashMap<String, Value>,
    materials: HashMap<String, Value>,
    transformations: HashMap<String, Vec<Value>>,
    camera: Option<Camera>,
    lights: Vec<Light>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            materials: HashMap::new(),
            transformations: HashMap::new(),
            camera: None,
            lights: Vec::new(),
        }
    }
}
