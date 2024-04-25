#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release // from demo

use egui::{Align, RichText, Ui, ViewportId};
use glium::{backend::{glutin::SimpleWindowBuilder, Facade}, glutin::{api::egl::display, surface::WindowSurface}, implement_buffer_content, implement_uniform_block, implement_vertex, index::PrimitiveType, program, uniform, GlObject, Surface};
use shapes::{Cube, Sphere};
use winit::{
    event,
    event_loop::{EventLoop, EventLoopBuilder},
};
use std::{fs, ops::RangeInclusive, time::Instant};

mod gui;
mod object_handeler;
mod shapes;
mod camera;
mod mouse_handler;
mod vec_util;

use gui::*;
use object_handeler::*;
use shapes::Triangle;
use camera::*;


fn init_spheres() -> Vec<Sphere> {
    vec![
        Sphere::new([0.0, 1.0, 0.0], [0.8078, 0.1647, 0.3569], 0.7)
    ]
}

fn main() {
    // setup glinum and window
    let event_loop = EventLoopBuilder::with_user_event().build().unwrap();
    let (window, display) = create_display(&event_loop);

    // setup gui and objects
    let mut gui_handeler = gui::GuiHandeler::new(egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop));
    let mut object_handeler = object_handeler::ObjectHandeler::new();

    // Setup scene
    let mut spheres = init_spheres();
    let mut triangles : Vec<Triangle> = vec![Triangle::new([-1.5, -1.5, 1.0], [-1.5, 1.5, 1.0], [1.5, -1.5, 1.0], [0.8078, 0.1647, 0.3569])];
    let mut cubes : Vec<Cube> = vec![Cube::new([0.0, 1.0, 0.0], [0.3, 1.5, 0.2], [0.8078, 0.1647, 0.3569])];
    //object_handeler.add_spheres_from(spheres);
    object_handeler.add_triangles_from(triangles);
    object_handeler.add_cubes_from(cubes);
    object_handeler.add_spheres_from(spheres);

    
    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {

        // todo remove colors, this code is from a demo
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }
        implement_vertex!(Vertex, position, color);

        // todo remove colors, this code is from a demo
        glium::VertexBuffer::new(&display,
            &[
                Vertex { position: [-1.0, -1.0], color: [0.0, 1.0, 0.0] }, 
                Vertex { position: [ 1.0,  1.0], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], color: [1.0, 0.0, 0.0] },

                Vertex { position: [-1.0, 1.0], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 1.0,  1.0], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ -1.0, -1.0], color: [1.0, 0.0, 0.0] },
            ]
        ).unwrap()
    };

    // building the index buffer - indices
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // compiling shaders and linking them together
    let source_vertex = fs::read_to_string("shaders/vertex.glsl").expect("Failed to read shader file");
    let source_fragment = fs::read_to_string("shaders/fragment.glsl").expect("Failed to read shader file");

    // load shaders
    let program = glium::Program::from_source(&display, &source_vertex, &source_fragment, None).unwrap();

    // from demo code
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let _ptr = program.get_frag_data_location("f_color").unwrap(); // will be zero; internal glium location for f_color that is "out" for fragment shader
    //program.get_shader_storage_blocks().get_key_value(k);

    // load sphere and triangle uniform buffer
    let mut sphere_array = object_handeler.get_uniform_buffer_spheres(&display);
    let mut triangle_array = object_handeler.get_uniform_buffer_triangles(&display);
    let mut cube_array = object_handeler.get_uniform_buffer_cubes(&display);

    // create camera 
    let mut camera = Camera::new();
    camera.pos = [0.0, 1.0, -3.0];
    camera.set_rotation_axis([0.0, 1.0, 0.0]);

    let mut should_quit = false;
    let mut should_update_objects = false;
    let result = event_loop.run(move |event, target| {

        let start = Instant::now();

        
        let mut redraw = |camera : &mut Camera| {

            if should_quit {
                target.exit() // exit program/window
            }

            // change gui
            gui_handeler.update_gui(&window, &mut should_quit, &mut object_handeler, &mut should_update_objects, camera);

            if should_update_objects {
                sphere_array = object_handeler.get_uniform_buffer_spheres(&display);
                triangle_array = object_handeler.get_uniform_buffer_triangles(&display);
            }

            if should_quit {
                target.exit() // exit program/window
            }

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui::Rgba::from_rgb(0.0, 0.0, 0.0);
                target.clear_color(color[0], color[1], color[2], color[3]);
                let data_ = [0.5, 0.4f32];

                // load spheres uniform buffer
                //let sphere_array = object_handeler.get_uniform_buffer_spheres(&display);
                 // load other uniforms
                //let u_resolution = [1500.0f32, 800.0f32]
                let u_resolution = [window.inner_size().width as f32, window.inner_size().height as f32];
                let numOfSpheres = object_handeler.get_num_of_spheres() as i32;
                let numOfTriangles = object_handeler.get_num_of_triangles() as i32;
                let numOfBoxes = object_handeler.get_num_of_cubes() as i32;
                let mut light_pos = [0.0f32, 0.0f32, -300.0f32];

                let renderMode = 2 as i32;
                let smoothness = 0.3 as f32;

                // a bug requires us to have the matrix as a uniform, even when we dont need the matrix in the shader, which is really wierd
                let matrix = [
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ];

                target.draw(
                    &vertex_buffer, 
                    &index_buffer, 
                    &program, 
                    //&uniform! {sphere_array: &*sphere_array, u_resolution : u_resolution, numOfSpheres : numOfSpheres, numOfTriangles : numOfTriangles}, 
                    //&uniform! {sphere_array: &*sphere_array}, 
                    // a bug requires us to have the matrix as a uniform, even when we dont need the matrix in the shader, which is really wierd
                    &uniform! {
                        // Format: name of uniform (in glsl) | resource/data
                        matrix : matrix, 
                        u_resolution : u_resolution, 
                        numOfSpheres : numOfSpheres, 
                        numOfTriangles : numOfTriangles, 
                        numOfBoxes : numOfBoxes,
                        renderMode : renderMode,
                        smoothness : smoothness,
                        lightPos : light_pos,
                        cameraPos : camera.pos,
                        cameraRotationQuaternion : camera.get_rotation_quaternion(), 
                        cameraFOV : camera.fov,
                        sphere_array : &*sphere_array, 
                        triangle_array : &*triangle_array,
                        cube_array : &*cube_array
                    }, 
                    &Default::default()
                ).unwrap();

                // draw things behind egui here
                gui_handeler.render(&display, &mut target);
                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {should_quit = true;}
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                    }
                    WindowEvent::RedrawRequested => redraw(&mut camera),
                    _ => {}
                }

                //let event_response = egui_glium.on_event(&window, &event);
                let event_response = gui_handeler.get_responce(&window, &event);
            

                let dur = Instant::elapsed(&start);
                let fps = 1.0 / dur.as_secs_f64();
                // println!("fps: {fps}");


                if event_response.repaint {
                    window.request_redraw();
                }
                
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            _ => (),
        }

        
        
    });
    result.unwrap()
}

// from demo code
fn create_display(
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, glium::Display<WindowSurface>) {
    SimpleWindowBuilder::new()
        .set_window_builder(winit::window::WindowBuilder::new().with_resizable(true))
        .with_inner_size(1000, 700)
        .with_title("egui_glium example")
        .build(event_loop)
}