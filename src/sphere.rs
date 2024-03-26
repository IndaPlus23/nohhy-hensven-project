use glm;

#[derive(Clone, Copy)]
pub struct Sphere {
    pub pos : [f32;3],
    pub color : [f32;3],
    pub radius : f32
}

// Sphere struct padded to 3 * sizeof(vec4)
// Used for setting the Uniform Buffer Object
pub struct GlSphere {
    pub pos : glm::Vec4,
    pub color : glm::Vec4,
    pub radius : glm::Vec4,
}

impl GlSphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        let new_pos = glm::vec4(pos[0], pos[1], pos[2], 1.0);
        let new_color = glm::vec4(color[0], color[1], color[2], 1.0);
        let new_radius = glm::vec4(radius, 0.0, 0.0, 0.0);

        GlSphere { pos : new_pos, color : new_color, radius : new_radius}
    }
}

impl Sphere {
    pub fn new(pos : [f32; 3], color : [f32; 3], radius : f32) -> Self {
        Sphere { pos, color, radius}
    }

    pub fn to_gl_sphere(&self) -> GlSphere {
        GlSphere::new(self.pos, self.color, self.radius)
    }
}
