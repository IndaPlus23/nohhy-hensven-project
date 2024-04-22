use gl::types::GLuint;
use std::ffi::CString;

use crate::shapes::{GlSphere, GlBox, Sphere, ToGl, Box};

pub enum UniformType {
    INT(i32),
    FLOAT(f32),
    VEC2([f32;2]),   
    VEC3([f32;3]),
    VEC4([f32;4])
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
                UniformType::VEC2(x) => gl::Uniform2f(uniform_location, x[0], x[1]),
                UniformType::VEC3(x) => gl::Uniform3f(uniform_location, x[0], x[1], x[2]),
                UniformType::VEC4(x) => gl::Uniform4f(uniform_location, x[0], x[1], x[2], x[3]),
                UniformType::FLOAT(x) => gl::Uniform1f(uniform_location, x),
                UniformType::INT(x) => gl::Uniform1i(uniform_location, x),
            }
            
        }
    } else {
        println!("Uniform location {} not found", uniform_name);
    } 
}


pub fn set_sphere_buffer_object(shader_program : GLuint, uniform_name : &str, values : Vec<Sphere>) {
    let mut sphere_buffer : gl::types::GLuint = 0;

    let gl_values : Vec<GlSphere> = values.iter().map(|x| x.to_gl()).collect();
    let mem_size = std::mem::size_of::<GlSphere>();

    unsafe {
        gl::GenBuffers(1, &mut sphere_buffer);
        gl::BindBuffer(gl::UNIFORM_BUFFER, sphere_buffer);

        gl::BufferData(
            gl::UNIFORM_BUFFER,
            (gl_values.len() * mem_size) as gl::types::GLsizeiptr,
            gl_values.as_ptr() as *const gl::types::GLvoid,
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

pub fn set_triangle_buffer_object(shader_program : GLuint, uniform_name : &str, values : Vec<Box>) {
    let mut sphere_buffer : gl::types::GLuint = 0;

    let gl_values : Vec<GlBox> = values.iter().map(|x| x.to_gl()).collect();
    let mem_size = std::mem::size_of::<GlBox>();

    unsafe {
        gl::GenBuffers(1, &mut sphere_buffer);
        gl::BindBuffer(gl::UNIFORM_BUFFER, sphere_buffer);

        gl::BufferData(
            gl::UNIFORM_BUFFER,
            (gl_values.len() * mem_size) as gl::types::GLsizeiptr,
            gl_values.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
    }

    // Bind buffer to a uniform block in the shader
    let binding_index = 1;
    unsafe {
        let block_index = gl::GetUniformBlockIndex(shader_program, CString::new(uniform_name).unwrap().as_ptr());

        gl::UniformBlockBinding(shader_program, block_index, binding_index);
        gl::BindBufferBase(gl::UNIFORM_BUFFER, binding_index, sphere_buffer);
    }
}

/* 


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

    let gl_values : Vec<GlBox> = values.iter().map(|x| x.to_gl()).collect();
    let mem_size = std::mem::size_of::<GlBox>();

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
*/