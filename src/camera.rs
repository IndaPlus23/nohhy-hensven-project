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

    pub fn rotate_around_obj(&mut self, obj_pos : [f32;3], angle : f32) {
        let x = self.pos[0] - obj_pos[0];
        let y = self.pos[1] - obj_pos[1];
        let z = self.pos[2] - obj_pos[2];

        let r = f32::sqrt(
            x * x + y * y + z * z
        );

        self.pos = [
            obj_pos[0] + r * f32::sin(PI + angle),
            obj_pos[1],
            obj_pos[2] + r * f32::cos(PI + angle),
        ]
    }
}