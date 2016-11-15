use specs::World;

use glium;
use glium::{Display, DisplayBuild, Surface};

use client::BClientContext;


pub struct RenderSystem { }

impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem {}
    }

    // maybe pass in some config struct
    pub fn new_window() -> Display {
        const WIDTH: u32 = 1280;
        const HEIGHT: u32 = 720;

        glium::glutin::WindowBuilder::new()
            .with_dimensions(WIDTH, HEIGHT)
            .with_depth_buffer(24)
            .with_title(format!("Hello world"))
            .build_glium()
            .expect("Failed to open window")
    }

    pub fn run(&mut self, window: &mut Display, world: &mut World, ctx: BClientContext) {
        let mut frame = window.draw();
        frame.clear_color_and_depth((0.0, 1.0, 0.0, 1.0), 1.0);
        frame.finish().unwrap();
        window.swap_buffers().unwrap();
    }
}
