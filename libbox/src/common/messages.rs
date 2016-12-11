use specs::Entity;

use common::resources::{CurrentHover};

// will be serializable in the future
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum Message {
    SelectEntity,
    InteractWith(Entity, CurrentHover),
    Quit,
}

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Version(pub String);
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct DisconnectReason(pub String);

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum NetworkMessage {
    GameMessage(Message),
    Connect(Version),
    Motd(String),
    Disconnect(DisconnectReason),
}
