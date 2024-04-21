


use glium::{glutin::surface::WindowSurface, implement_uniform_block};

use crate::shapes::{Sphere, Triangle};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SphereArray { // It seems to be hard to directly input a array of struct as in the older demo, so I temporarily chose to do it this way instead
    number_of_objects: f32,
    _padding: [f32; 3],
    positions: [[f32; 4]; 128],   // all of this should be improved, but it seeems as if that the alignment is tricky, so I was pragmatic
    colors: [[f32; 4]; 128],
    radius: [[f32; 4]; 128], // should be a float array
}

pub struct ObjectHandeler{

    // ssbo for gpu storage
    //ssbo_triangles : Ssbo,
    //ssbo_spheres : Ssbo,

    // for cpu storage
    cpu_triangles : Vec<Triangle>,
    cpu_spheres : Vec<Sphere>,

    // other stuff
    data_is_modified : bool,
}

impl ObjectHandeler{

    pub fn new() -> ObjectHandeler{
        let mut handeler = ObjectHandeler{
            //ssbo_triangles : Ssbo::new(11),
            //ssbo_spheres : Ssbo::new(10),
            cpu_triangles : Vec::new(),
            cpu_spheres : Vec::new(),
            data_is_modified : false
        };
        handeler.initiate();
        return handeler;
    }

    fn initiate(&mut self){
        
        // this is something that glium needs
        implement_uniform_block!(SphereArray, number_of_objects, positions, colors, radius);
    }

    pub fn get_num_of_triangles(&self) -> usize{self.cpu_triangles.len()}
    pub fn get_num_of_spheres(&self) -> usize{self.cpu_spheres.len()}

    /*
    fn add_render_object<T : ToGl>(&mut self, render_object : T){

    }
    */


    pub fn add_triangle(&mut self, render_object : Triangle){
        self.cpu_triangles.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_sphere(&mut self, render_object : Sphere){
        self.cpu_spheres.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_triangles_from(&mut self, mut render_objects : Vec<Triangle>){
        self.cpu_triangles.append(&mut render_objects);
        self.data_is_modified = true;
    }

    pub fn add_spheres_from(&mut self, mut render_objects : Vec<Sphere>){
        self.cpu_spheres.append(&mut render_objects);
        self.data_is_modified = true;
        println!("{:?}", self.cpu_spheres);
    }

    // TODO: remove()

    /* 
    // update the gpu data if neccsary
    pub fn update(&mut self){
        
        match self.data_is_modified{
            true =>{
                self.transfer_cpu_data_to_gpu(); 
                self.data_is_modified = false;
            }, 
            _=> {}
        }
    }
    */


    // solution until I get ssbo to work with glinum
    pub fn get_uniform_buffer_spheres(&mut self, display : &glium::Display<WindowSurface>) -> glium::uniforms::UniformBuffer<SphereArray>{
        
        let mut sphere_array: glium::uniforms::UniformBuffer<SphereArray> = glium::uniforms::UniformBuffer::empty(display).unwrap();

        {
            let mut mapping = sphere_array.map();
            let mut counter : usize = 0;
            self.cpu_spheres.iter_mut().for_each(|sphere|{
                let mut position = [0.0f32; 4];
                position[..3].copy_from_slice(&sphere.pos);
                mapping.positions[counter] = position;

                let mut color = [0.0f32; 4];
                color[..3].copy_from_slice(&sphere.color);
                mapping.colors[counter] = color;

                let mut radius = [0.0f32; 4];
                radius[0] = sphere.radius;
                mapping.radius[counter] = radius;
                counter += 1;
            });
            
        }
        
        return sphere_array;
    }

}

    /* 
}


    fn transfer_cpu_data_to_gpu(&mut self){
        // TODO: should be optimized in the future, not really neccasary at all times to redraw/resend all data, all data has probobly not been modified
        self.ssbo_triangles.initalize(self.cpu_triangles.clone()); // clone really neccasary?
        self.ssbo_spheres.initalize(self.cpu_spheres.clone());
    }
    */
