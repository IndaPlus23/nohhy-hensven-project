pub struct Camera {
    pub pos : [f32; 3],
    pub view_diretion : [f32; 3]
}

impl Camera {
    pub fn new() -> Self {
        Camera { pos: [0.0, 0.0, 0.0], view_diretion: [0.0, 0.0, 1.0] }
    }
}