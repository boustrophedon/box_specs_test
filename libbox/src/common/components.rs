use nalgebra::{Eye, Matrix4};

use specs;
use specs::Component;

#[derive(Clone, Copy, Debug)]
pub struct Render {
    pub model_transform: Matrix4<f32>,
}

impl Render {
    pub fn new() -> Render {
        Render {
            model_transform: Matrix4::new_identity(4),
        }
    }

    pub fn with_transform(model: Matrix4<f32>) -> Render {
        Render {
            model_transform: model,
        }
    }
}

impl Component for Render {
    type Storage = specs::VecStorage<Render>;
}
