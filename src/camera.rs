use cgmath::{EuclideanSpace, InnerSpace, Matrix4, Point3, Rad, Vector3};

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub vertical_velocity: f32,
}

impl Camera {
    pub fn new(position: Vector3<f32>, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
            speed: 2.5,
            sensitivity: 0.1,
            vertical_velocity: 0.0,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        let front = self.front();
        Matrix4::look_at_rh(
            Point3::from_vec(self.position),
            Point3::from_vec(self.position + front),
            Vector3::new(0.0, 0.0, 1.0),
        )
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, dt: f32) {
        let velocity = self.speed * dt;
        let yaw = Rad(self.yaw.to_radians());
        let forward = Vector3::new(yaw.0.cos(), yaw.0.sin(), 0.0).normalize();
        let right = Vector3::new(-forward.y, forward.x, 0.0);
        match direction {
            CameraMovement::Forward => self.position += forward * velocity,
            CameraMovement::Backward => self.position -= forward * velocity,
            CameraMovement::Left => self.position -= right * velocity,
            CameraMovement::Right => self.position += right * velocity,
        }
    }

    pub fn process_mouse(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * self.sensitivity;
        self.pitch = (self.pitch + dy * self.sensitivity).clamp(-89.0, 89.0);
    }

    pub fn update(&mut self, dt: f32) {
        const GRAVITY: f32 = -9.8;
        self.vertical_velocity += GRAVITY * dt;
        self.position.z += self.vertical_velocity * dt;
    }

    fn front(&self) -> Vector3<f32> {
        let yaw = Rad(self.yaw.to_radians());
        let pitch = Rad(self.pitch.to_radians());
        Vector3::new(
            yaw.0.cos() * pitch.0.cos(),
            yaw.0.sin() * pitch.0.cos(),
            pitch.0.sin(),
        )
        .normalize()
    }
}
