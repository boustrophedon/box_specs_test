use specs::{MessageQueue, RunArg, System, World};

use client::{ClientConfig, ClientSystemContext};

use common::Message;
use common::components::*;
use common::resources::MyClientId;

pub struct SpawnSystem {
}

impl SpawnSystem {
    pub fn new(_: ClientConfig) -> SpawnSystem {
        SpawnSystem {}
    }
}

impl System<Message, ClientSystemContext> for SpawnSystem {
    fn run(&mut self, arg: RunArg, _: MessageQueue<Message>, _: ClientSystemContext) {
        let _ = arg.fetch(|_| {});
    }

    fn handle_message(&mut self, world: &mut World, msg: &Message) {
        use common::Message::*;
        match *msg {
            SpawnBox(pos, client) => {
                let controllable = {
                    world.read_resource::<MyClientId>().0 == Some(client)
                };
                let mut newbox = world.create_now()
                    .with(Render::new())
                    .with(Movement::new_pos(pos))
                    .with(Selection::new())
                    .with(ClientId::new(client));

                if controllable {
                    newbox = newbox.with(Controllable::new());
                }
                newbox.build();

            }
            _ => (),
        }
    }
}
