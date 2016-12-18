use specs::Entity;

use rustc_serialize::json;

use nalgebra::Point3;

use common::resources::{CurrentHover};
use common::ClientID;

#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub enum Message {
    SpawnBox(Point3<f32>, ClientID),
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
    Connected(ClientID, String),
    Disconnect(DisconnectReason),
}

impl NetworkMessage {
    pub fn encode_as_bytes(&self, buf: &mut Vec<u8>) {
        buf.push(4); buf.push(2);
        let msg_string = json::encode(self).unwrap();
        for &b in msg_string.len().to_string().as_bytes() {
            buf.push(b);
        }
        buf.push(59); // ';'
        buf.append(&mut msg_string.into_bytes());
    }

    pub fn decode_from_str(buf: &str) -> json::DecodeResult<(NetworkMessage, usize)> {
        // check message is long enough. 2 bytes for magic number, at least 1 (though it must be
        // more) for the size, and one for the semicolon
        assert!(buf.len() > 4, "message was not long enough to be recognized");

        // check magic number at beginning
        assert!(buf.as_bytes()[0] == 4 && buf.as_bytes()[1] == 2, "message did not begin with magic number");

        // find colon directly after length
        let colon = buf.find(";").expect("message did not contain leading colon");
        let length = buf[2..colon].parse::<usize>().expect("message did not contain length before colon");

        assert!(buf.len() > length+colon, "message is not as long as it claims to be");

        // we return the length of the decoded message so we know how much to advance in the buffer
        return json::decode(&buf[colon+1..colon+length+1]).map(|msg| (msg, colon+length+1));
    }
}

pub fn decode_messages(messages_buf: &str) -> json::DecodeResult<Vec<NetworkMessage>> {
    let mut current_index = 0usize;
    let mut results = Vec::new();
    loop {
        if current_index == messages_buf.len() {
            return Ok(results);
        }
        let decoded = NetworkMessage::decode_from_str(&messages_buf[current_index..]);
        match decoded {
            Ok((m, index)) => {
                results.push(m);
                current_index += index;
            },
            Err(error) => {
                return Err(error);
            }
        }
    }
}

