use cgmath::InnerSpace;
use cgmath::Rotation3;
use cgmath::Vector3;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug, Clone)]
pub struct CameraPos {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
}

impl Default for CameraPos {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraPos {
    pub fn new() -> Self {
        Self {
            eye: (0.0, 5.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
        }
    }

    pub fn rotate(&mut self, dx: f32, dy: f32) {
        let dx = cgmath::Rad(dx * 0.005);
        let dy = cgmath::Rad(dy * 0.005);

        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(cgmath::Vector3::unit_y()).normalize();
        let up = right.cross(forward).normalize();

        let rotation = cgmath::Quaternion::from_axis_angle(right, dy) * cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_y(), dx);

        let new_target = rotation * forward;
        self.target = self.eye + new_target;
    }

    pub fn move_eye(&mut self, amount: Vector3<f32>) {
        let forward = (self.target - self.eye).normalize();
        let right = forward.cross(cgmath::Vector3::unit_y()).normalize();
        let up = right.cross(forward).normalize();

        self.eye += right * amount.x;
        self.eye += up * amount.y;
        self.eye += forward * amount.z;
        self.target += right * amount.x;
        self.target += up * amount.y;
        self.target += forward * amount.z;
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: (0.0, 5.0, 10.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.1,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn update_pos(&mut self, pos: &CameraPos) {
        self.eye = pos.eye;
        self.target = pos.target;
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn build_uniform(&self) -> CameraUniform {
        CameraUniform::from_camera(self.clone())
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        proj * view
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn from_camera(camera: Camera) -> Self {
        Self {
            view_proj: (OPENGL_TO_WGPU_MATRIX * camera.build_view_projection_matrix()).into(),
        }
    }
}
