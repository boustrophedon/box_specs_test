use time;

use specs::{World, MessageQueue};

use glium;
use glium::Display;

use client::ClientSystemContext;

use common::Message;
use common::resources::IsRunning;

pub struct InputSystem { }

impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem { }
    }

    pub fn run(&mut self, window: &mut Display, world: &mut World, msg: MessageQueue<Message>, ctx: ClientSystemContext) {
        for event in window.poll_events() {
            use glium::glutin::Event;
            use glium::glutin::VirtualKeyCode as KC;
            match event {
                Event::Closed => {
                    msg.send(Message::Quit);
                    world.write_resource::<IsRunning>().0 = false;
                },
                Event::KeyboardInput(state, _, key) => {
                    if state == glium::glutin::ElementState::Pressed {
                        match key.unwrap() {
                            KC::Escape => { 
                                msg.send(Message::Quit);
                                world.write_resource::<IsRunning>().0 = false;
                                println!("{:?}", time::now());
                            },
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
        }
    }
}
