use specs::{Join, MessageQueue, RunArg, System, World};

use ncollide;
use ncollide::shape::Cuboid;
use ncollide::broad_phase::BroadPhase;

use nalgebra;
use nalgebra::Vector3;

use client::ClientSystemContext;

use common::Message;
use common::components::{Movement, Selection};
use common::resources::{Camera, CurrentHover, CurrentSelection, CursorPosition};

pub struct SelectionSystem { }

impl SelectionSystem {
    pub fn new() -> SelectionSystem {
        SelectionSystem { }
    }
}

impl System<Message, ClientSystemContext> for SelectionSystem {
    fn run(&mut self, args: RunArg, msg: MessageQueue<Message>, ctx: ClientSystemContext) {
        let (entities, movement, mut sel, camera, mut curr_hover, cursor) = args.fetch(|w| {
            (
                w.entities(),
                w.read::<Movement>(),
                w.write::<Selection>(),
                w.read_resource::<Camera>(),
                w.write_resource::<CurrentHover>(),
                w.read_resource::<CursorPosition>(),
            )
        });

        // this should really be in a build_world() function
        // but I can't figure out how to pass this without writing
        // gigantic types in the signature
        let mut bp = ncollide::broad_phase::DBVTBroadPhase::new(0.2, true);
        let square = Cuboid::new(Vector3::new(1f32, 1.0, 0.0));
        for (e, m, s) in (&entities, &movement, &mut sel).iter() {
            s.hovered = false;

            use ncollide::bounding_volume;
            use nalgebra::Isometry3;
            let pos = Isometry3::new(m.position.to_vector(), nalgebra::zero());
            bp.deferred_add(e.get_id() as usize, bounding_volume::aabb(&square, &pos), e);
        }
        bp.update(&mut |a, b| *a != *b, &mut |_, _, _| { });

        let ray = camera.ray_from_screen(cursor.0);
        let mut hits = Vec::new();
        bp.interferences_with_ray(&ray, &mut hits);

        let selected = hits.first().cloned().cloned();

        // set current hover
        match selected {
            Some(e) => {
                let selection = sel.get_mut(e).unwrap();
                selection.hovered = true;
                *curr_hover = CurrentHover::Entity(e);
            }

            None =>  {
                let mut groundpos = ray.origin + (-ray.origin.z/ray.dir.z)*ray.dir;
                groundpos.z = 0.0;
                *curr_hover = CurrentHover::Ground(groundpos);
            }
        }
    }

    fn handle_message(&mut self, world: &mut World, msg: &Message) {
        match *msg {
            Message::SelectEntity => {
                let hover = world.read_resource::<CurrentHover>();
                let mut curr_sel = world.write_resource::<CurrentSelection>();
                let mut sel = world.write::<Selection>();

                // current selection not selected anymore
                if let Some(e) = curr_sel.0 {
                    sel.get_mut(e).map(|s| s.selected = false);
                }

                // set new current selection based on hovered location
                match *hover {
                    CurrentHover::Entity(e) => {
                        curr_sel.0 = Some(e);
                        sel.get_mut(e).map(|s| s.selected = true);
                    }
                    _ => curr_sel.0 = None,
                }
            }
            _ => ()
        }
    }
}
