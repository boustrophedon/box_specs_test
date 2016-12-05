use std::net::SocketAddr;

use time::Duration;

use specs;

mod systems;
use self::systems::*;

use common::Message;
use common::resources::*;
use common::components::*;

#[derive(Clone, Copy, Debug)]
pub struct ServerConfig {
    pub timestep: Duration,
    pub sim_rate: Duration,
    pub server_address: SocketAddr,
    // data directories, etc
}

impl ServerConfig {
    pub fn new() -> ServerConfig {
        ServerConfig {
            timestep: Duration::milliseconds(2),
            sim_rate: Duration::milliseconds(33),
            server_address: "127.0.0.1:8844".parse().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct ServerSystemContext {
    pub dt: Duration,
    pub timestep: Duration,
}

impl ServerSystemContext {
    pub fn new(dt: Duration, timestep: Duration) -> ServerSystemContext {
        ServerSystemContext {
            dt: dt,
            timestep: timestep,
        }
    }
}

pub fn make_server_world(cfg: ServerConfig) -> specs::Planner<Message, ServerSystemContext> {
    let mut world = specs::World::new();

    world.register::<Movement>();
    world.register::<Controllable>();

    world.create_now().with(Movement::new()).with(Controllable::new()).build();

    for i in 0..50 {
        let x = ((((17*i+73)%80)-40) as f32)/2.0;
        let y = ((((3*i+45)%50)-25) as f32)/2.0;
        let pos = Movement::new_pos(Point3::new(x, y, 0.0));
        world.create_now().with(pos).with(Controllable::new()).build();
    }

    // start at +5, move to -5
    use nalgebra::Point3;
    let mvmnt = Movement::new_pos_target(Point3::new(5.0, 0.0, 0.0), Point3::new(-5.0, 0.0, 0.0));
    world.create_now().with(mvmnt).build();

    world.add_resource(IsRunning(true));

    let mut p = specs::Planner::new(world, 4);
    p.add_system(MovementSystem::new(), "movement", 2);
    p.add_system(NetworkSystem::new(cfg), "network", 20);

    p
}

pub struct ServerGame {
    planner: specs::Planner<Message, ServerSystemContext>,
    ctx: ServerSystemContext,
    running: bool,
}

impl ServerGame {
    pub fn new(planner: specs::Planner<Message, ServerSystemContext>, cfg: ServerConfig) -> ServerGame {
        let ctx = ServerSystemContext::new(Duration::seconds(0), cfg.timestep);

        ServerGame {
            planner: planner,
		    ctx: ctx,
            running: true,
        }
    }

    pub fn run(&mut self, dt: Duration) {
        self.ctx.dt = dt;
        self.planner.dispatch(self.ctx.clone());
        self.planner.handle_messages();

        self.running = self.planner.mut_world().read_resource::<IsRunning>().0;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests;
