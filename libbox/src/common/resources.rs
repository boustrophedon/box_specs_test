use specs::Entity;

use nalgebra;
use nalgebra::{Inverse, Isometry3, Point2, Point3, Matrix4, Norm, PerspectiveMatrix3, ToHomogeneous, FromHomogeneous, Vector3, Vector4};

use ncollide::query::Ray;

use client::ClientConfig;

#[derive(Clone, Debug)]
pub struct IsRunning(pub bool);


#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    view: Isometry3<f32>,
    persp: PerspectiveMatrix3<f32>,
    width: f32,
    height: f32,
}

const UP: Vector3<f32> = Vector3 {x: 0.0, y: 1.0, z: 0.0};
impl Camera {
    pub fn new(cfg: ClientConfig) -> Camera {
        let width = cfg.window_width as f32;
        let height = cfg.window_height as f32;
        let aspect = width/height;

        let position = Point3::new(0.0, 0.0, 10.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&position, &target, &UP);
        let persp = PerspectiveMatrix3::new(aspect, cfg.fov, 1.0, 100.0);

        Camera {
            position: position,
            target: target,
            view: view,
            persp: persp,
            width: width,
            height: height,
        }
    }

    pub fn perspective(&self) -> Matrix4<f32> {
        self.persp.to_matrix()
    }
    pub fn view<'a>(&'a self) -> Matrix4<f32> {
        self.view.to_homogeneous()
    }

    /// Stay at current position and look at target
    pub fn look_at(&mut self, target: Point3<f32>) {
        self.target = target;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &UP);
    }

    /// Look at current target and move to position
    pub fn move_to(&mut self, position: Point3<f32>) {
        self.position = position;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &UP);
    }

    pub fn set_screen_size(&mut self, width: u32, height: u32) {
        self.width = width as f32;
        self.height = height as f32;
    }

    /// Returns a normalized ray pointing from the camera's position into the scene
    /// projected from the point clicked on the screen
    pub fn ray_from_screen(&self, p: Point2<i32>) -> Ray<Point3<f32>> {
        let mut v = Vector4::new((2.0*p.x as f32)/self.width - 1.0, (2.0*p.y as f32)/self.height - 1.0, 1.0, 1.0);

        // isometries always have inverses. perspective should as well, I think?
        v = (self.view.inverse().unwrap().to_homogeneous() * self.persp.as_matrix().inverse().unwrap() * v).normalize();

        Ray::new(self.position, nalgebra::from_homogeneous(&v))
    }
}


#[derive(Clone, Debug)]
pub struct CurrentSelection(pub Option<Entity>);
