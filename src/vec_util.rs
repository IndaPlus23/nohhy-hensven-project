pub fn q_mul(r: [f32; 4], s: [f32; 4]) -> [f32; 4] {
    let x = r[0] * s[0] - r[1] * s[1] - r[2] * s[2] - r[3] * s[3];
    let y = r[0] * s[1] + r[1] * s[0] - r[2] * s[3] + r[3] * s[2];
    let z = r[0] * s[2] + r[1] * s[3] + r[2] * s[0] - r[3] * s[1];
    let w = r[0] * s[3] - r[1] * s[2] + r[2] * s[1] + r[3] * s[0];

    [x, y, z, w]
}

pub fn rotate_pos(pos: [f32; 3], camera_rotation_quaternion: [f32; 4]) -> [f32; 3] {
    let p = [0.0, pos[0], pos[1], pos[2]];
    let q = camera_rotation_quaternion;
    let q_inv = [q[0], -q[1], -q[2], -q[3]];

    let res = q_mul(q_mul(q, p), q_inv);

    [res[1], res[2], res[3]]
}

pub fn get_rotation_quaternion(axis : [f32;3], angle : f32) -> [f32; 4] {
    [
        f32::cos(angle / 2.0),
        axis[0] * f32::sin(angle / 2.0),
        axis[1] * f32::sin(angle / 2.0),
        axis[2] * f32::sin(angle / 2.0)
    ]
}

/// x + ky
pub fn vec_add(x : [f32;3], y : [f32;3], k : f32) -> [f32; 3] {
    [
        x[0] + k * y[0],
        x[1] + k * y[1],
        x[2] + k * y[2]
    ]
}

pub fn vec2_add(x : [f32;2], y : [f32;2], k : f32) -> [f32; 2] {
    [
        x[0] + k * y[0],
        x[1] + k * y[1]
    ]
}



pub fn vec_mul(v : [f32;2], k : f32) -> [f32;2] {
    [
        v[0] * k,
        v[1] * k
    ]
}

pub fn normalize(v : [f32; 3]) -> [f32; 3] {
    let norm = f32::sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);

    [
        v[0] / norm,
        v[1] / norm,
        v[2] / norm
    ]
}

pub fn cross_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

pub fn rotate_vec2(v : [f32; 2], angle : f32) -> [f32; 2] {
    let new_x = v[0] * angle.cos() - v[1] * angle.sin();
    let new_y = v[0] * angle.sin() + v[1] * angle.cos();

    [new_x, new_y]
}
