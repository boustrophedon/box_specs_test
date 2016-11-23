use specs::{Join, MessageQueue, World};

use glium;
use glium::{Display, DisplayBuild, Frame, Surface};

use client::ClientSystemContext;
use client::ClientConfig;

use common::Message;
use common::resources::Camera;
use common::components::{Movement, Render};


#[derive(Clone, Copy, Debug)]
struct Vert {
    position: [f32; 3],
}

implement_vertex!(Vert, position);

struct BoxRenderer {
    box_vb: glium::vertex::VertexBuffer<Vert>,
    box_shader: glium::program::Program,
    box_drawparams: glium::DrawParameters<'static>,
}

impl BoxRenderer {
    pub fn new(window: &mut Display) -> BoxRenderer {
        let tr = [1f32, 1.0, 0.0];
        let tl = [-1f32, 1.0, 0.0];
        let bl = [-1f32, -1.0, 0.0];
        let br = [1f32, -1.0, 0.0];

		let data = &[
			Vert { position: tr }, 
			Vert { position: tl }, 
			Vert { position: bl }, 
                   
			Vert { position: tr }, 
			Vert { position: bl }, 
			Vert { position: br },
		];
        let box_vb = glium::vertex::VertexBuffer::new(window, data).unwrap();

        let v_shader = "
            #version 150
            
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            in vec3 position;

            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        ";
        let f_shader = "
            #version 150

            void main() {
                gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
            }
        ";

        let box_shader = glium::program::Program::from_source(window, v_shader, f_shader, None).unwrap();

        let params = glium::DrawParameters {
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        BoxRenderer {
            box_vb: box_vb,
            box_shader: box_shader,
            box_drawparams: params,
        }

    }

    pub fn render(&self, render: &Render, frame: &mut Frame, camera: &Camera) {
        // get transform, etc. probably from physics component actually
        // uv coords, texture ids, and model from render component in a generic "model renderer"
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            perspective: camera.perspective().as_ref().clone(),
            view: camera.view().as_ref().clone(),
            model: render.model_transform.as_ref().clone(),
        };
        frame.draw(&self.box_vb, &indices, &self.box_shader, &uniforms, &self.box_drawparams).unwrap();
    }

}

pub struct RenderSystem {
    box_renderer: BoxRenderer, 
}

impl RenderSystem {
    pub fn new(window: &mut Display) -> RenderSystem {
        RenderSystem {
            box_renderer: BoxRenderer::new(window)
        }
    }

    pub fn new_window(cfg: ClientConfig) -> Display {
        glium::glutin::WindowBuilder::new()
            .with_dimensions(cfg.window_width, cfg.window_height)
            .with_depth_buffer(24)
            .with_title(format!("Hello world"))
            .build_glium()
            .expect("Failed to open window")
    }

    pub fn run(&mut self, window: &mut Display, world: &mut World, msg: MessageQueue<Message>, ctx: ClientSystemContext) {
        let camera = world.read_resource::<Camera>();

        let mut frame = window.draw();
        frame.clear_color_and_depth((0.0, 1.0, 0.0, 1.0), 1.0);

        let movement = world.read::<Movement>();
        let mut render = world.write::<Render>();
        for (m, r) in (&movement, &mut render).iter() {
            // maybe put this in a function
            r.model_transform[(0, 3)] = m.position[0];
            r.model_transform[(1, 3)] = m.position[1];
            r.model_transform[(2, 3)] = m.position[2];
            self.box_renderer.render(r, &mut frame, &camera);
        }

        frame.finish().unwrap();
    }
}
