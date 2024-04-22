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
pub struct Box {
    pub pos : [f32;3],
    pub dim : [f32;3],
    pub color : [f32;3]
}

// Sphere struct padded to 3 * sizeof(vec4)
// Used for setting the Uniform Buffer Object
pub struct GlSphere {
    pub pos : glm::Vec4,
    pub color : glm::Vec4,
    pub radius : glm::Vec4,
}

pub struct GlBox {
    pos : glm::Vec4,
    dim : glm::Vec4,
    color : glm::Vec4,
}

impl GlSphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        let new_pos = glm::vec4(pos[0], pos[1], pos[2], 1.0);
        let new_color = glm::vec4(color[0], color[1], color[2], 1.0);
        let new_radius = glm::vec4(radius, 0.0, 0.0, 0.0);

        GlSphere { pos : new_pos, color : new_color, radius : new_radius}
    }
}

impl GlBox {
    pub fn new(pos : [f32; 3], dim : [f32; 3], color : [f32;3]) -> Self {
        let new_pos = glm::vec4(pos[0], pos[1], pos[2], 1.0);
        let new_dim = glm::vec4(dim[0], dim[1], dim[2], 1.0);

        let new_color = glm::vec4(color[0], color[1], color[2], 1.0);

        GlBox { pos: new_pos, dim: new_dim, color: new_color }
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

impl ToGl for Box {
    type Output = GlBox;

    fn to_gl(&self) -> Self::Output {
        GlBox::new(self.pos, self.dim,  self.color)
    }
    fn mem_size(&self) -> usize {
        std::mem::size_of::<GlBox>()
    }
}

impl Sphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        Sphere { pos, color, radius}
    }

}

impl Box {
    pub fn new(pos : [f32; 3], dim : [f32; 3],  color : [f32; 3]) -> Self {
        Box { pos, dim, color}
    }
}
