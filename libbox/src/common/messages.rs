use specs::Entity;

use nalgebra::Point2;

use glium::glutin::{ElementState, MouseButton};

// will be serializable in the future
#[derive(Clone, Debug)]
pub enum Message {
    GroundSelected(Point2<f32>),
    EntitySelected(Entity),
    MouseMoved(i32, i32), // same as glutin::Event::MouseMoved, x, y relative to top-left corner
    MouseInput(ElementState, MouseButton), // same as glutin::Event::MouseInput
    Quit,
}
