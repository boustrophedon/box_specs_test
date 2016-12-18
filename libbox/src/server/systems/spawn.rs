use specs::{MessageQueue, RunArg, System, World};

use server::{ServerConfig, ServerSystemContext};

use common::Message;
use common::components::*;

pub struct SpawnSystem {
}

impl SpawnSystem {
    pub fn new(_: ServerConfig) -> SpawnSystem {
        SpawnSystem {}
    }
}

impl System<Message, ServerSystemContext> for SpawnSystem {
    fn run(&mut self, arg: RunArg, _: MessageQueue<Message>, _: ServerSystemContext) {
        let _ = arg.fetch(|_| {});
    }

    fn handle_message(&mut self, world: &mut World, msg: &Message) {
        use common::Message::*;
        match *msg {
            SpawnBox(pos, client) => {
                world.create_now()
                    .with(Movement::new_pos(pos))
                    .with(ClientId::new(client))
                    .build();
            }
            _ => (),
        }
    }
}
