use std::f32::consts::PI;

pub struct Camera {
    pub pos : [f32; 3],
    pub rotation_axis : [f32; 3],
    pub angle : f32,
    pub fov : f32
}

fn q_mul(r: &[f32; 4], s: &[f32; 4]) -> [f32; 4] {
    let x = r[0] * s[0] - r[1] * s[1] - r[2] * s[2] - r[3] * s[3];
    let y = r[0] * s[1] + r[1] * s[0] - r[2] * s[3] + r[3] * s[2];
    let z = r[0] * s[2] + r[1] * s[3] + r[2] * s[0] - r[3] * s[1];
    let w = r[0] * s[3] - r[1] * s[2] + r[2] * s[1] + r[3] * s[0];

    [x, y, z, w]
}

fn rotate_pos(pos: &[f32; 3], camera_rotation_quaternion: &[f32; 4]) -> [f32; 3] {
    let p = [0.0, pos[0], pos[1], pos[2]];
    let q = *camera_rotation_quaternion;
    let q_inv = [q[0], -q[1], -q[2], -q[3]];

    let res = q_mul(&q_mul(&q, &p), &q_inv);

    [res[1], res[2], res[3]]
}

fn get_rotation_quaternion(axis : &[f32;3], angle : f32) -> [f32; 4] {
    [
        f32::cos(angle / 2.0),
        axis[0] * f32::sin(angle / 2.0),
        axis[1] * f32::sin(angle / 2.0),
        axis[2] * f32::sin(angle / 2.0)
    ]
}

/// x + ky
fn vec_add(x : &[f32;3], y : &[f32;3], k : f32) -> [f32; 3] {
    [
        x[0] + k * y[0],
        x[1] + k * y[1],
        x[2] + k * y[2]
    ]
}

fn normalize(v : &[f32; 3]) -> [f32; 3] {
    let norm = f32::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);

    [
        v[0] / norm,
        v[1] / norm,
        v[2] / norm
    ]
}

impl Camera {
    pub fn new() -> Self {
        Camera { pos: [0.0, 0.0, 0.0], rotation_axis : [0., 1., 0.], angle : 0.0, fov : 80.0}
    }

    pub fn get_rotation_quaternion(&self) -> [f32;4] {
        get_rotation_quaternion(&self.rotation_axis, self.angle)
    }

    pub fn set_rotation_axis(&mut self, axis : &[f32;3]) {
        self.rotation_axis = normalize(axis);
    }

    pub fn rotate_around_obj(&mut self, obj_pos : &[f32;3], angle : f32) {
        let quat = get_rotation_quaternion(&self.rotation_axis, angle);

        let rotated_pos = rotate_pos(&vec_add(&self.pos, obj_pos, -1.0), &quat);

        self.pos = vec_add(&rotated_pos, obj_pos, 1.0);
        self.angle -= angle;
    }
}