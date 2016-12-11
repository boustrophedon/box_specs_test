use specs::Entity;

use common::resources::{CurrentHover};

// will be serializable in the future
#[derive(Clone, Debug)]
pub enum Message {
    SelectEntity,
    InteractWith(Entity, CurrentHover),
    Quit,
}

#[derive(Clone, Debug)]
pub struct Version(pub String);
#[derive(Clone, Debug)]
pub struct DisconnectReason(pub String);

#[derive(Clone, Debug)]
pub enum NetworkMessage {
    GameMessage(Message),
    Connect(Version),
    Motd(String),
    Disconnect(DisconnectReason),
}
