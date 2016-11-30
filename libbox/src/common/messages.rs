use specs::Entity;

use common::resources::{CurrentHover};

// will be serializable in the future
#[derive(Clone, Debug)]
pub enum Message {
    SelectEntity,
    InteractWith(Entity, CurrentHover),
    Quit,
}
