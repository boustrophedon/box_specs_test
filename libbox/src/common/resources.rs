use nalgebra::{Isometry3, Point3, Matrix4, PerspectiveMatrix3, ToHomogeneous, Transformation, Vector3, Vector4};

use client::ClientConfig;

#[derive(Clone, Debug)]
pub struct IsRunning(pub bool);

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    view: Matrix4<f32>,
    persp: Matrix4<f32>,
}

const UP: Vector3<f32> = Vector3 {x: 0.0, y: 1.0, z: 0.0};
impl Camera {
    pub fn new(cfg: ClientConfig) -> Camera {
        let aspect = cfg.window_width as f32/cfg.window_height as f32;

        let position = Point3::new(0.0, 0.0, 10.0);
        let target = Point3::new(0.0, 0.0, 0.0);
        let view = Isometry3::look_at_rh(&position, &target, &UP).to_homogeneous();
        let persp = PerspectiveMatrix3::new(aspect, cfg.fov, 1.0, 100.0).to_matrix();

        Camera {
            position: position,
            target: target,
            view: view,
            persp: persp,
        }
    }

    pub fn perspective(&self) -> &Matrix4<f32> {
        &self.persp
    }
    pub fn view(&self) -> &Matrix4<f32> {
        &self.view
    }

    /// Stay at current position and look at target
    pub fn look_at(&mut self, target: Point3<f32>) {
        self.target = target;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &UP).to_homogeneous();
    }

    /// Look at current target and move to position
    pub fn move_to(&mut self, position: Point3<f32>) {
        self.position = position;
        self.view = Isometry3::look_at_rh(&self.position, &self.target, &UP).to_homogeneous();
    }
}
