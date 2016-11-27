use specs::{Join, MessageQueue, RunArg, System};

use ncollide;
use ncollide::shape::Cuboid;
use ncollide::broad_phase::BroadPhase;

use nalgebra;
use nalgebra::{Point2, Vector3};

use client::ClientSystemContext;

use common::Message;
use common::components::{Movement, Selection};
use common::resources::{Camera, CurrentSelection};

pub struct SelectionSystem {
    last_pos: Point2<i32>, // could make this an Option but not worth it
    set_selection: bool,
}

impl SelectionSystem {
    pub fn new() -> SelectionSystem {
        SelectionSystem {
            last_pos: Point2::new(0, 0),
            set_selection: false,
        }
    }

    // fn clear_world(&mut self) {
    //     // I wish there were a "clear" function
    //     self.collision_bp = ncollide::broad_phase::DBVTBroadPhase::new(0.05, true);
    // }

    // fn add_to_world(&mut self, e: Entity, m: &Movement) {
    //     use ncollide::bounding_volume::aabb;
    //     let square = Cuboid::new(Vector3::new(0.5, 0.5, 0.0));
    //     self.collision_bp.deferred_add(e.get_id() as usize, aabb(square, m.position), e);
    // }

    // fn query_world(&self, camera: &Camera) -> Option<Entity> {
    //     //let ray = camera.ray_to_coords(self.last_pos);
    //     let ray = ncollide::query::Ray::new(Point3::new(0.0, 0.0, 10.0), Vector3::new(0.0, 0.0, 1.0));
    //     let mut hits = Vec::new();
    //     self.collision_bp.interferences_with_ray(&ray, &mut hits);
    //     return hits.first();
    // }

    // fn finalize_world(&mut self) {
    //     // magic incantation that does all the adds
    //     // to understand the closures see reference for DBVTBroadPhase
    //     self.collision_bp.update(&mut |a, b| *a != *b, &mut |_, _, _| { });
    // }
}

impl System<Message, ClientSystemContext> for SelectionSystem {
    fn run(&mut self, args: RunArg, msg: MessageQueue<Message>, ctx: ClientSystemContext) {
        let (entities, movement, mut sel, camera, mut curr_sel) = args.fetch(|w| {
            (
                w.entities(),
                w.read::<Movement>(),
                w.write::<Selection>(),
                w.read_resource::<Camera>(),
                w.write_resource::<CurrentSelection>()
            )
        });

        // this should really be in a build_world() function
        // but I can't figure out how to pass this without writing
        // gigantic types in the signature
        let mut bp = ncollide::broad_phase::DBVTBroadPhase::new(0.2, true);
        let square = Cuboid::new(Vector3::new(1f32, 1.0, 0.0));
        for (e, m, s) in (&entities, &movement, &mut sel).iter() {
            // clear hover
            s.hovered = false;

            use ncollide::bounding_volume;
            use nalgebra::Isometry3;
            let pos = Isometry3::new(m.position.to_vector(), nalgebra::zero());
            bp.deferred_add(e.get_id() as usize, bounding_volume::aabb(&square, &pos), e);
        }
        bp.update(&mut |a, b| *a != *b, &mut |_, _, _| { });

        //let selected = self.query_world(camera.deref());
        let ray = camera.ray_from_screen(self.last_pos);
        //let ray = ncollide::query::Ray::new(Point3::new(0.0f32, 0.0, 10.0), Vector3::new(0.0f32, 0.0, -1.0));
        let mut hits = Vec::new();
        bp.interferences_with_ray(&ray, &mut hits);

        let selected = hits.first().cloned().cloned();

        if self.set_selection {
            if curr_sel.0.is_some() {
                match sel.get_mut(curr_sel.0.unwrap()) {
                    Some(s) => s.selected = false,
                    None => (),
                }
            }
            curr_sel.0 = selected;
        }
        match selected {
            Some(e) => {
                let selection = sel.get_mut(e).unwrap();
                selection.hovered = true;
                if self.set_selection { selection.selected = true; }
            },
            None => ()
        }

        if self.set_selection {
            self.set_selection = false;
        }
    }

    fn handle_message(&mut self, msg: &Message) {
        match *msg {
            Message::MouseMoved(x, y) => self.last_pos = Point2::new(x, y),
            Message::MouseInput(state, button) => {
                use glium::glutin::{ElementState, MouseButton};
                if state == ElementState::Pressed && button == MouseButton::Left {
                    self.set_selection = true;
                }
            },
            _ => ()
        }
    }
}
