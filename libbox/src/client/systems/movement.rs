use time::Duration;

use specs::{Join, MessageQueue, RunArg, System, World};

use client::ClientSystemContext;

use common::Message;
use common::components::{Controllable, Movement};
use common::resources::CurrentHover;

use nalgebra::Point3;


pub struct MovementSystem { }

impl MovementSystem {
    pub fn new() -> MovementSystem {
        MovementSystem { }
    }

    fn do_movement(&mut self, m: &mut Movement, timestep: Duration) {
        let (begin, end, mut t) = m.current_path.unwrap();

        let progress = m.speed*(timestep.num_milliseconds() as f32);
        t += progress;

        if t >= 1.0 {
            m.position = end;
            m.current_path = None;
        }
        else {
            let new_position = lerpf32(&begin, &end, &t);
            m.position = new_position;
            m.current_path = Some((begin, end, t));
        }
    }
}

impl System<Message, ClientSystemContext> for MovementSystem {
    fn run(&mut self, arg: RunArg, _: MessageQueue<Message>, ctx: ClientSystemContext) {
        let mut mvt = arg.fetch(|w| w.write::<Movement>());

        let mut accum = ctx.dt;
        while accum > ctx.timestep {
            for m in (&mut mvt).iter() {
                if m.current_path.is_some() {
                    self.do_movement(m, ctx.timestep);
                }
            }
            accum = accum - ctx.timestep;
        }
    }

    fn handle_message(&mut self, world: &mut World, msg: &Message) {
        match *msg {
            Message::InteractWith(e, ref interact) => {
                let control = world.read::<Controllable>();
                let mut movement = world.write::<Movement>();
                control.get(e)
                .and_then(|_| movement.get_mut(e))
                .map(|m| match *interact {
                    CurrentHover::Ground(target) => m.set_target(target),
                    _ => (),
                });
            },
            _ => (),
        }
    }
}

pub fn lerpf32(begin: &Point3<f32>, end: &Point3<f32>, t: &f32) -> Point3<f32> {
    let x = (1.0 - t)*begin.x + t*end.x;
    let y = (1.0 - t)*begin.y + t*end.y;
    let z = (1.0 - t)*begin.z + t*end.z;

    Point3::new(x, y, z)
}
