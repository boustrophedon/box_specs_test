use time::Duration;

use specs;
use specs::{System, RunArg};

use glium;

mod systems;
use self::systems::*;


#[derive(Clone)]
pub struct BClientContext {
    pub message_queue: (),
    pub dt: Duration,
}

impl BClientContext {
    pub fn new(timestep: Duration) -> BClientContext {
        BClientContext {
            message_queue: (),
            dt: timestep,
        }
    }
}

struct IsRunning(bool);

pub fn make_world() -> specs::Planner<BClientContext> {
    let mut world = specs::World::new();
    let testsys = TestSystem::new("hello");

    world.add_resource(IsRunning(true));

    let mut p = specs::Planner::new(world, 4);
    p.add_system(testsys, "test", 1);

    p
}

pub struct ClientGame {
    input: InputSystem,
    planner: specs::Planner<BClientContext>,
    render: RenderSystem,
    window: glium::Display,
    running: bool,
}

impl ClientGame {
    pub fn new() -> ClientGame {
        // probably read some config file here, pass config struct into the news
        // or maybe just use the ctx

        let input = InputSystem::new();
        let render = RenderSystem::new();
        let planner = make_world();
        let window = RenderSystem::new_window();
        ClientGame {
            input: input,
            planner: planner,
            render: render,
            window: window,
            running: true,
        }
    }

    pub fn run(&mut self, ctx: BClientContext) {
        self.input.run(&mut self.window, self.planner.mut_world(), ctx.clone());
        self.planner.dispatch(ctx.clone());
        self.planner.wait();
        self.render.run(&mut self.window, self.planner.mut_world(), ctx.clone());

        if !self.planner.mut_world().read_resource::<IsRunning>().0 {
            self.running = false;
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.running
    }
}
