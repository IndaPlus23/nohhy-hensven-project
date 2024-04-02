use glm;

pub trait ToGl {
    type Output;

    fn to_gl(&self) -> Self::Output;
    fn mem_size(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub pos : [f32;3],
    pub color : [f32;3],
    pub radius : f32
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v1 : [f32;3],
    pub v2 : [f32;3],
    pub v3 : [f32;3],
    pub color : [f32;3]
}

// Sphere struct padded to 3 * sizeof(vec4)
// Used for setting the Uniform Buffer Object
pub struct GlSphere {
    pub pos : glm::Vec4,
    pub color : glm::Vec4,
    pub radius : glm::Vec4,
}

pub struct GlTriangle {
    pub v1 : glm::Vec4,
    pub v2 : glm::Vec4,
    pub v3 : glm::Vec4,
    pub color : glm::Vec4
}

impl GlSphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        let new_pos = glm::vec4(pos[0], pos[1], pos[2], 1.0);
        let new_color = glm::vec4(color[0], color[1], color[2], 1.0);
        let new_radius = glm::vec4(radius, 0.0, 0.0, 0.0);

        GlSphere { pos : new_pos, color : new_color, radius : new_radius}
    }
}

impl GlTriangle {
    pub fn new(v1 : [f32; 3], v2 : [f32; 3], v3 : [f32; 3], color : [f32;3]) -> Self {
        let new_v1 = glm::vec4(v1[0], v1[1], v1[2], 1.0);
        let new_v2 = glm::vec4(v2[0], v2[1], v2[2], 1.0);
        let new_v3 = glm::vec4(v3[0], v3[1], v3[2], 1.0);

        let new_color = glm::vec4(color[0], color[1], color[2], 1.0);

        GlTriangle { v1: new_v1, v2: new_v2, v3: new_v3, color: new_color }
    }
}

impl ToGl for Sphere {
    type Output = GlSphere;

    fn to_gl(&self) -> Self::Output {
        GlSphere::new(self.pos, self.color, self.radius)
    }
    fn mem_size(&self) -> usize {
        std::mem::size_of::<GlSphere>()
    }
}

impl ToGl for Triangle {
    type Output = GlTriangle;

    fn to_gl(&self) -> Self::Output {
        GlTriangle::new(self.v1, self.v2, self.v3, self.color)
    }
    fn mem_size(&self) -> usize {
        std::mem::size_of::<GlTriangle>()
    }
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
