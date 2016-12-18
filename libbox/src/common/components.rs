use nalgebra;
use nalgebra::{Eye, Point3, Matrix4, Vector3};

use specs;
use specs::Component;

use common::ClientID;

#[derive(Clone, Copy, Debug)]
pub struct Render {
    pub model_transform: Matrix4<f32>,
    pub color: Vector3<f32>,
}

impl Render {
    pub fn new() -> Render {
        Render {
            model_transform: Matrix4::new_identity(4),
            color: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    // should do a RenderBuilder in the future
    pub fn with_transform(model: Matrix4<f32>) -> Render {
        Render {
            model_transform: model,
            color: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

impl Component for Render {
    type Storage = specs::VecStorage<Render>;
}


#[derive(Clone, Copy, Debug)]
pub struct Movement {
    // In the future we can change these to a Path
    // and let position just be an f32 in [0,1]
    // For now we just set the position to be lerped to target over time
    pub position: Point3<f32>,
    pub speed: f32,
    pub current_path: Option<(Point3<f32>, Point3<f32>, f32)>,
}

const SPEED: f32 = 0.002; // 2 unit per s in units per ms
impl Movement {
    pub fn new() -> Movement {
        Movement {
            position: Point3::new(0.0, 0.0, 0.0),
            speed: 0.0,
            current_path: None,
        }
    }

    pub fn new_pos(position: Point3<f32>) -> Movement {
        Movement {
            position: position,
            speed: 0.0,
            current_path: None,
        }
    }

    pub fn new_pos_target(position: Point3<f32>, target: Point3<f32>) -> Movement {
        Movement {
            position: position,
            speed: SPEED/nalgebra::distance(&position, &target),
            current_path: Some((position, target, 0.0)),
        }
    }

    pub fn set_target(&mut self, target: Point3<f32>) {
        self.speed = SPEED/nalgebra::distance(&self.position, &target);
        self.current_path = Some((self.position, target, 0.0));
    }
}

impl Component for Movement {
    type Storage = specs::VecStorage<Movement>;
}


#[derive(Clone, Copy, Debug)]
pub struct Selection {
    pub hovered: bool,
    pub selected: bool,
}

impl Selection {
    pub fn new() -> Selection {
        Selection {
            hovered: false,
            selected: false,
        }
    }
}

impl Component for Selection {
    type Storage = specs::VecStorage<Selection>;
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Controllable { }

impl Controllable {
    pub fn new() -> Controllable {
        Controllable { }
    }
}

impl Component for Controllable {
    type Storage = specs::NullStorage<Controllable>;
}


#[derive(Clone, Copy, Debug, Default)]
pub struct ClientId(ClientID);

impl ClientId {
    pub fn new(id: ClientID) -> ClientId {
        ClientId(id)
    }
}

impl From<ClientID> for ClientId {
    fn from(id: ClientID) -> ClientId {
        ClientId::new(id)
    }
}

impl Component for ClientId {
    type Storage = specs::HashMapStorage<ClientId>;
}

