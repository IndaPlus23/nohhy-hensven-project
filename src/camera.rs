use std::f32::consts::PI;

pub struct Camera {
    pub pos : [f32; 3],
    pub rotation_axis : [f32; 3],
    pub angle : f32,
    pub fov : f32
}

impl Camera {
    pub fn new() -> Self {
        Camera { pos: [0.0, 0.0, 0.0], rotation_axis : [0., 1., 0.], angle : 0.0, fov : 80.0}
    }

    pub fn get_rotation_quaternion(&self) -> [f32;4] {
        [
            f32::cos(self.angle / 2.0),
            self.rotation_axis[0] * f32::sin(self.angle / 2.0),
            self.rotation_axis[1] * f32::sin(self.angle / 2.0),
            self.rotation_axis[2] * f32::sin(self.angle / 2.0)
        ]
    }
}