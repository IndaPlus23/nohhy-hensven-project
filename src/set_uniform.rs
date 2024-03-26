use gl::types::GLuint;
use std::ffi::CString;

use crate::sphere::{Sphere, GlSphere};

pub enum UniformType {
    INT(i32),
    FLOAT(f32),
    VEC2([f32;2]),   
    VEC3([f32;3])
}

pub fn set_uniform(shader_program : GLuint, uniform_name : &str, uniform_value : UniformType) {
    // Get the location of the uniform variable in the shader program
    let uniform_location = unsafe {
        gl::GetUniformLocation(shader_program, CString::new(uniform_name).unwrap().as_ptr())
    };

    // Check if the uniform location is valid (-1 means not found)
    if uniform_location != -1 {
        unsafe {
            match uniform_value {
                UniformType::VEC3(x) => gl::Uniform3f(uniform_location, x[0], x[1], x[2]),
                UniformType::VEC2(x) => gl::Uniform2f(uniform_location, x[0], x[1]),
                UniformType::FLOAT(x) => gl::Uniform1f(uniform_location, x),
                UniformType::INT(x) => gl::Uniform1i(uniform_location, x),
            }
            
        }
    } else {
        println!("Uniform location {} not found", uniform_name);
    } 
}


pub fn set_sphere_buffer(shader_program : GLuint, uniform_name : &str, spheres : Vec<Sphere>) {
    let mut sphere_buffer : gl::types::GLuint = 0;

    let gl_spheres : Vec<GlSphere> = spheres.iter().map(|x| x.to_gl_sphere()).collect();

    unsafe {
        gl::GenBuffers(1, &mut sphere_buffer);
        gl::BindBuffer(gl::UNIFORM_BUFFER, sphere_buffer);

        gl::BufferData(
            gl::UNIFORM_BUFFER,
            (gl_spheres.len() * std::mem::size_of::<GlSphere>()) as gl::types::GLsizeiptr,
            gl_spheres.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
    }

    // Bind buffer to a uniform block in the shader
    let binding_index = 0;
    unsafe {
        let block_index = gl::GetUniformBlockIndex(shader_program, CString::new(uniform_name).unwrap().as_ptr());

        gl::UniformBlockBinding(shader_program, block_index, binding_index);
        gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_index, sphere_buffer);
    }
}