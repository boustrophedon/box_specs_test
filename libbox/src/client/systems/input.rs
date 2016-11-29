use specs::{World, MessageQueue};

use glium;
use glium::Display;

use client::ClientSystemContext;

use common::Message;
use common::resources::{CursorPosition, CurrentHover, CurrentSelection, IsRunning};

pub enum Keybinds {
    InGame,
    Menu,
}

pub struct InputSystem {
    current_keybinds: Keybinds,
}

use glium::glutin::Event;
use glium::glutin::VirtualKeyCode as KC;
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            current_keybinds: Keybinds::InGame,
        }
    }

    pub fn run(&mut self, window: &mut Display, world: &mut World, msg: MessageQueue<Message>, ctx: ClientSystemContext) {
        for event in window.poll_events() {
            match event {
                Event::MouseMoved(x, y) => {let mut pos = world.write_resource::<CursorPosition>(); pos.0.x = x; pos.0.y = y;},
                _ => (),
            }
            match self.current_keybinds {
                Keybinds::InGame => self.handle_ingame(event, world, &msg, &ctx),
                Keybinds::Menu => () // TODO,
            }
        }
    }

    pub fn handle_ingame(&mut self, event: Event, world: &mut World, msg: &MessageQueue<Message>, ctx: &ClientSystemContext) {
        use glium::glutin::{ElementState, MouseButton};
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
                        },
                        _ => ()
                    }
                }
            }

            Event::MouseInput(state, button) => {
                // TODO instead of MouseButton::Left, we should have a mapping for "move" and
                // "action", etc.
                if state == ElementState::Pressed {
                    if button == MouseButton::Left {
                        msg.send(Message::SelectEntity);
                    }
                    else if button == MouseButton::Right {
                        world.read_resource::<CurrentSelection>().0.map(|sel_entity| {
                            msg.send(Message::InteractWith(sel_entity, world.read_resource::<CurrentHover>().clone()));
                        });
                    }
                }
            }
            _ => ()
        }
    }
}
