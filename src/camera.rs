use crate::vec_util::{normalize, get_rotation_quaternion, vec_add, rotate_pos};

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
        get_rotation_quaternion(self.rotation_axis, self.angle)
    }

    pub fn set_rotation_axis(&mut self, axis : [f32;3]) {
        self.rotation_axis = normalize(axis);
    }

    pub fn rotate_around_obj(&mut self, obj_pos : [f32;3], angle : f32) {
        let quat = get_rotation_quaternion(self.rotation_axis, angle);

        let rotated_pos = rotate_pos(vec_add(self.pos, obj_pos, -1.0), quat);

        self.pos = vec_add(rotated_pos, obj_pos, 1.0);
        self.angle -= angle;
    }
}