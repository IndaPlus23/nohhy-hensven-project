
#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub pos : [f32;3],
    pub color : [f32;3],
    pub radius : f32
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub v1 : [f32;3],
    pub v2 : [f32;3],
    pub v3 : [f32;3],
    pub color : [f32;3]
}

impl Sphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        Sphere { pos, color, radius}
    }

}

impl Triangle {
    pub fn new(v1 : [f32; 3], v2 : [f32; 3], v3 : [f32; 3], color : [f32; 3]) -> Self {
        Triangle { v1, v2, v3, color}
    }
}
