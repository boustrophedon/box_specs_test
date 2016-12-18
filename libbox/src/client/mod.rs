use std::net::SocketAddr;

use time::Duration;

use specs;
use glium;

use nalgebra::Point2;

mod systems;
use self::systems::*;

use common::Message;
use common::resources::*;
use common::components::*;

#[derive(Clone, Copy, Debug)]
pub struct ClientConfig {
    pub timestep: Duration,
    pub sim_rate: Duration,
    pub window_width: u32,
    pub window_height: u32,
    pub fov: f32,
    pub server_address: SocketAddr,
    // data directories, etc
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        use std::f32::consts::FRAC_PI_4;
        ClientConfig {
            timestep: Duration::milliseconds(2),
            sim_rate: Duration::milliseconds(33),
            window_width: 1280,
            window_height: 720,
            fov: FRAC_PI_4,
            server_address: "127.0.0.1:8844".parse().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct ClientSystemContext {
    pub dt: Duration,
    pub timestep: Duration,
}

impl ClientSystemContext {
    pub fn new(dt: Duration, timestep: Duration) -> ClientSystemContext {
        ClientSystemContext {
            dt: dt,
            timestep: timestep,
        }
    }
}

pub fn make_client_world(cfg: ClientConfig) -> specs::Planner<Message, ClientSystemContext> {
    let mut world = specs::World::new();

    world.register::<Render>();
    world.register::<Movement>();
    world.register::<Selection>();
    world.register::<Controllable>();
    world.register::<ClientId>();

    world.add_resource(IsRunning(true));
    world.add_resource(Camera::new(cfg.window_width, cfg.window_height, cfg.fov));
    world.add_resource(CursorPosition(Point2::new(0,0)));
    world.add_resource(CurrentSelection(None));
    world.add_resource(CurrentHover::None);
    world.add_resource(MyClientId(None));

    let mut p = specs::Planner::new(world, 4);
    p.add_system(SelectionSystem::new(), "selection", 1);
    p.add_system(MovementSystem::new(), "movement", 2);
    p.add_system(SpawnSystem::new(cfg), "spawn", 10);
    p.add_system(NetworkSystem::new(cfg), "network", 20);

    p
}

pub struct ClientGame {
    input: InputSystem,
    planner: specs::Planner<Message, ClientSystemContext>,
    render: RenderSystem,
    window: glium::Display,
    ctx: ClientSystemContext,
    running: bool,
}

impl ClientGame {
    pub fn new(planner: specs::Planner<Message, ClientSystemContext>, cfg: ClientConfig) -> ClientGame {
        let input = InputSystem::new();
        let mut window = RenderSystem::new_window(cfg);
        let render = RenderSystem::new(&mut window);

        let ctx = ClientSystemContext::new(Duration::seconds(0), cfg.timestep);

        ClientGame {
            input: input,
            planner: planner,
            render: render,
            window: window,
		    ctx: ctx,
            running: true,
        }
    }

    pub fn get_input(&mut self) {
        let msg = self.planner.message_out.clone();
        let world = self.planner.mut_world();
        self.input.run(&mut self.window, world, msg, self.ctx.clone());
    }

    pub fn run(&mut self, dt: Duration) {
        self.ctx.dt = dt;
        self.planner.dispatch(self.ctx.clone());
        self.planner.handle_messages();

        self.running = self.planner.mut_world().read_resource::<IsRunning>().0;
    }

    pub fn render(&mut self) {
        let msg = self.planner.message_out.clone();
        let world = self.planner.mut_world();
        self.render.run(&mut self.window, world, msg, self.ctx.clone());
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

#[cfg(test)]
mod tests;
