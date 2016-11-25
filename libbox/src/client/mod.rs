use time::Duration;

use specs;
use glium;

mod systems;
use self::systems::*;

use common::Message;
use common::resources::*;
use common::components::*;

#[derive(Clone, Copy, Debug)]
pub struct ClientConfig {
    pub timestep: Duration,
    pub window_width: u32,
    pub window_height: u32,
    pub fov: f32,
    // data directories, etc
}

impl ClientConfig {
    pub fn new() -> ClientConfig {
        use std::f32::consts::FRAC_PI_4;
        ClientConfig {
            timestep: Duration::milliseconds(2),
            window_width: 1280,
            window_height: 720,
            fov: FRAC_PI_4,
        }
    }
}

#[derive(Clone)]
pub struct ClientSystemContext {
    pub dt: Duration,
}

impl ClientSystemContext {
    pub fn new(timestep: Duration) -> ClientSystemContext {
        ClientSystemContext {
            dt: timestep,
        }
    }
}

pub fn make_client_world(cfg: ClientConfig) -> specs::Planner<Message, ClientSystemContext> {
    let mut world = specs::World::new();

    world.register::<Render>();
    world.register::<Movement>();
    world.register::<Selection>();

    let e1 = world.create_now().with(Render::new()).with(Movement::new()).with(Selection::new()).build();
    println!("created entity e1: {:?}", e1);

    // start at +5, move to -5
    use nalgebra::Point3;
    let mvmnt = Movement::new_pos_target(Point3::new(5.0, 0.0, 0.0), Point3::new(-5.0, 0.0, 0.0));
    let e2 = world.create_now().with(Render::new()).with(mvmnt).with(Selection::new()).build();
    println!("created entity e2: {:?}", e2);

    world.add_resource(IsRunning(true));
    world.add_resource(Camera::new(cfg));
    world.add_resource(CurrentSelection(None));

    let mut p = specs::Planner::new(world, 4);
    p.add_system(SelectionSystem::new(), "selection", 1);
    p.add_system(MovementSystem::new(), "movement", 2);

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

        let ctx = ClientSystemContext::new(cfg.timestep);

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

    pub fn run(&mut self) {
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
