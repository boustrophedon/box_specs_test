use specs::World;

use glium;
use glium::Display;

use client::{BClientContext, IsRunning};

pub struct InputSystem { }

impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem { }
    }

    pub fn run(&mut self, window: &mut Display, world: &mut World, ctx: BClientContext) {
        let mut running = world.write_resource::<IsRunning>();

        for event in window.poll_events() {
            use glium::glutin::Event;
            use glium::glutin::VirtualKeyCode as KC;
            match event {
                Event::Closed => (running.0 = false),
                Event::KeyboardInput(state, _, key) => {
                    if state == glium::glutin::ElementState::Pressed {
                        match key.unwrap() {
                            KC::Escape => {running.0 = false;},
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
        }
    }
}
