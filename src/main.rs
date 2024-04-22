extern crate glutin;
extern crate gl;

use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event::{Event, WindowEvent};
use std::ffi::CString;
use std::fs;
use std::time::{Duration, Instant};

mod camera;
mod shapes;
mod set_uniform;
mod ssbo;
mod render_objects_handeler;

use set_uniform::*;
use camera::Camera;
use shapes::{Sphere, Box};
use set_uniform::{set_sphere_buffer_object, set_triangle_buffer_object};
use ssbo::{Ssbo};
use render_objects_handeler::{ObjectHandeler};

fn load_shader(source_path: &str, shader_type: u32) -> u32 {
    let source = fs::read_to_string(source_path).expect("Failed to read shader file");
    let shader = unsafe {gl::CreateShader(shader_type)};
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }
    
    let mut success = gl::FALSE as i32;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }
    if success != gl::TRUE as i32 {
        let mut log_length = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let log = vec![0u8; log_length as usize];
            gl::GetShaderInfoLog(shader, log_length, std::ptr::null_mut(), log.as_ptr() as *mut i8);
            let log_string = String::from_utf8_lossy(&log);
            panic!("Failed to compile shader: {}", log_string);
        }
    } else {
        println!("Shader compiled successfully!")
    }

    shader
}

fn init_spheres() -> Vec<Sphere> {
    vec![
        Sphere::new([0.0, 1.0, 0.0], [0.8078, 0.1647, 0.3569], 0.7)
    ]
}

fn main() {
    // Define the size of the viewport (width and height in pixels)
    let mut width = 1000;   
    let mut height = 700; 

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("OpenGL Window");
    let context = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    let context = unsafe { context.make_current().unwrap() };
    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    // Load and compile shaders
    let vertex_shader = load_shader("shaders/vertex.glsl", gl::VERTEX_SHADER);
    let fragment_shader = load_shader("shaders/fragment.glsl", gl::FRAGMENT_SHADER);

    // Create shader program
    let shader_program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        gl::UseProgram(shader_program);
    }

    // Define vertex data (positions)
    let vertices: [f32; 18] = [
        -1.0, -1.0, 0.0,  // Bottom-left
         1.0, -1.0, 0.0,  // Bottom-right
        -1.0,  1.0, 0.0,  // Top-left
        1.0, -1.0, 0.0,  // Bottom-right
        1.0,  1.0, 0.0,  // Top-right
        -1.0,  1.0, 0.0,  // Top-left
    ];

    // Create and bind vertex buffer object (VBO)
    let mut vbo: u32 = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // Specify vertex attribute pointers
    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, std::ptr::null());
        gl::EnableVertexAttribArray(0);
    }

    // Setup scene
    let mut spheres = init_spheres();
    let mut boxes : Vec<Box> = vec![Box::new([0.0, 1.0, 0.0], [0.3, 1.5, 0.2], [0.8078, 0.1647, 0.3569])];
    let mut light_pos = [5.0, 5.0, -3.0];
    let mut t = 0.0;

    let mut camera = Camera::new();
    camera.pos = [0.0, 1.0, -3.0];
    camera.set_rotation_axis(&[0.0, 1.0, 0.0]);

    // New way to add objects to render
    // setup scene with objectHandeler
    
    let mut object_handeler = ObjectHandeler::new();

    /*
    //// New way to add objects to render, which works fine
    let mut sphere_buffer = Ssbo::new(10);
    let mut triangle_buffer = Ssbo::new(11);
    sphere_buffer.initalize(spheres.clone());
    triangle_buffer.initalize(triangles.clone());
    */

    let mut t : f32 = 0.0;

    
    camera.rotate_around_obj(&[0., 0., 0.], 1.0);

    //Set uinform values
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        t += 0.001;

        match event {
            
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(new_size) => {
                    (width, height) = new_size.into();
                    unsafe {
                        gl::Viewport(0, 0, width as i32, height as i32);
                    }
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                let start = Instant::now();

                object_handeler.set_boxes(boxes.clone()); // should instead be a move, I belive
                object_handeler.set_spheres(spheres.clone()); // should instead be a move, I belive
            
                // transfer data to gpu memory
                object_handeler.update();

                set_uniform(shader_program, "u_resolution", UniformType::VEC2([width as f32, height as f32]));
                set_uniform(shader_program, "lightPos", UniformType::VEC3(light_pos));
                set_uniform(shader_program, "numOfSpheres", UniformType::INT(spheres.len() as i32));
                set_uniform(shader_program, "numOfBoxes", UniformType::INT(boxes.len() as i32));
                set_uniform(shader_program, "cameraPos", UniformType::VEC3(camera.pos));
                set_uniform(shader_program, "cameraRotationQuaternion", UniformType::VEC4(camera.get_rotation_quaternion()));
                set_uniform(shader_program, "cameraFOV", UniformType::FLOAT(camera.fov));
                // Render modes
                // 0 : Normal
                // 1 : Intersect
                set_uniform(shader_program, "renderMode", UniformType::INT(2));
                set_uniform(shader_program, "smoothness", UniformType::FLOAT(0.3));

                // Clear the color buffer
                unsafe { 
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    // Draw the triangle
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                }

                boxes[0].pos[2] = f32::sin(t);
        
                // Swap buffers if using double buffering
                context.swap_buffers().unwrap();

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();

                println!("fps: {fps}");
            }
            _ => (),
        }

    });    
}
