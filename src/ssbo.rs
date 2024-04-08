
use gl::types::GLuint;
use std::ffi::CString;

use crate::shapes::{GlSphere, GlTriangle, Sphere, ToGl, Triangle};

pub enum UniformType {
    INT(i32),
    FLOAT(f32),
    VEC2([f32;2]),   
    VEC3([f32;3])
}


pub fn set_sphere_ssbo(shader_program : GLuint, uniform_name : &str, values : Vec<Sphere>) {
    let mut sphere_buffer : gl::types::GLuint = 0;
    let gl_values : Vec<GlSphere> = values.iter().map(|x| x.to_gl()).collect();
    let mem_size = std::mem::size_of::<GlSphere>();

    unsafe {
        gl::GenBuffers(1, &mut sphere_buffer);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, sphere_buffer);

        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (gl_values.len() * mem_size) as gl::types::GLsizeiptr,
            gl_values.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_READ
        );
    }

    let binding_index = 10;
    unsafe {
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_index, sphere_buffer);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    }
}


pub fn set_triangle_ssbo(shader_program : GLuint, uniform_name : &str, values : Vec<Triangle>) {
    let mut sphere_buffer : gl::types::GLuint = 0;

    let gl_values : Vec<GlTriangle> = values.iter().map(|x| x.to_gl()).collect();
    let mem_size = std::mem::size_of::<GlTriangle>();

    unsafe {
        gl::GenBuffers(1, &mut sphere_buffer);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, sphere_buffer);

        gl::BufferData(
            gl::SHADER_STORAGE_BUFFER,
            (gl_values.len() * mem_size) as gl::types::GLsizeiptr,
            gl_values.as_ptr() as *const gl::types::GLvoid,
            gl::DYNAMIC_READ // optimizations preset, if we wish to update this ssbo often
        );
    }

    // Bind buffer to a uniform block in the shader
    let binding_index = 11;
    unsafe {
        gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, binding_index, sphere_buffer);
        gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    }
}