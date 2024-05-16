


use glium::{glutin::surface::WindowSurface, implement_uniform_block};

use crate::shapes::{Sphere, Triangle, Cube, MengerSponge};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SphereArray { // It seems to be hard to directly input a array of struct as in the older demo, so I temporarily chose to do it this way instead
    number_of_objects: f32,
    _padding: [f32; 3],
    positions: [[f32; 4]; 128],   // all of this should be improved, but it seeems as if that the alignment is tricky, so I was pragmatic
    colors: [[f32; 4]; 128],
    radius: [[f32; 4]; 128], // should be a float array
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TriangleArray { 
    v1: [[f32; 4]; 128],   
    v2: [[f32; 4]; 128], 
    v3: [[f32; 4]; 128], 
    norm: [[f32; 4]; 128],
    color_triangles: [[f32; 4]; 128],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CubesArray { 
    pos_cubes: [[f32; 4]; 128],   
    dim_cubes: [[f32; 4]; 128], 
    color_cubes: [[f32; 4]; 128],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MengerSpongeArray { 
    pos_menger_sponges: [[f32; 4]; 128],   
    iterations_menger_sponges: [[f32; 4]; 128], 
    color_menger_sponges: [[f32; 4]; 128],
}

pub struct ObjectHandeler{

    // ssbo for gpu storage
    //ssbo_triangles : Ssbo,
    //ssbo_spheres : Ssbo,

    // for cpu storage
    cpu_triangles : Vec<Triangle>,
    cpu_spheres : Vec<Sphere>,
    cpu_cubes : Vec<Cube>,
    cpu_menger_sponges : Vec<MengerSponge>,

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
            cpu_cubes : Vec::new(),
            cpu_menger_sponges : Vec::new(),
            data_is_modified : false
        };
        handeler.initiate(); // initiate sphere and triangle struct for glinum
        return handeler;
    }

    fn initiate(&mut self){
        
        // this is something that glium needs, so that we can pass these resources as uniforms
        implement_uniform_block!(SphereArray, number_of_objects, positions, colors, radius);
        implement_uniform_block!(TriangleArray, v1, v2, v3, norm, color_triangles);
        implement_uniform_block!(CubesArray, pos_cubes, dim_cubes, color_cubes);
        implement_uniform_block!(MengerSpongeArray, pos_menger_sponges, iterations_menger_sponges, color_menger_sponges);
    }

    pub fn get_num_of_triangles(&self) -> usize{self.cpu_triangles.len()}
    pub fn get_num_of_spheres(&self) -> usize{self.cpu_spheres.len()}
    pub fn get_num_of_cubes(&self) -> usize{self.cpu_cubes.len()}
    pub fn get_num_of_menger_sponges(&self) -> usize{self.cpu_menger_sponges.len()}

    pub fn get_spheres_reference(&mut self) -> &mut Vec<Sphere>{
        &mut self.cpu_spheres
    }

    pub fn get_cubes_reference(&mut self) -> &mut Vec<Cube>{
        &mut self.cpu_cubes
    }

    pub fn get_menger_sponges_reference(&mut self) -> &mut Vec<MengerSponge>{
        &mut self.cpu_menger_sponges
    }

    /*
    fn add_render_object<T : ToGl>(&mut self, render_object : T){

    }
    */

    pub fn remove_sphere(&mut self, render_object : Sphere){
        let pos = self.cpu_spheres.iter().position(|object|{object == &render_object});
        if let Some(pos_unwrapped) = pos{
            self.cpu_spheres.remove(pos_unwrapped);
        }
    }

    pub fn add_triangle(&mut self, render_object : Triangle){
        self.cpu_triangles.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_sphere(&mut self, render_object : Sphere){
        self.cpu_spheres.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_cube(&mut self, render_object : Cube){
        self.cpu_cubes.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_menger_sponge(&mut self, render_object : MengerSponge){
        self.cpu_menger_sponges.push(render_object);
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

    pub fn add_cubes_from(&mut self, mut render_objects : Vec<Cube>){
        self.cpu_cubes.append(&mut render_objects);
        self.data_is_modified = true;
    }

    pub fn add_menger_sponges_from(&mut self, mut render_objects : Vec<MengerSponge>){
        self.cpu_menger_sponges.append(&mut render_objects);
        self.data_is_modified = true;
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


    // temp solution until I get ssbo to work with glinum
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

    // temp solution until I get ssbo to work with glinum
    pub fn get_uniform_buffer_triangles(&mut self, display : &glium::Display<WindowSurface>) -> glium::uniforms::UniformBuffer<TriangleArray>{
        
        let mut triangle_array: glium::uniforms::UniformBuffer<TriangleArray> = glium::uniforms::UniformBuffer::empty(display).unwrap();

        {
            let mut mapping = triangle_array.map();
            let mut counter : usize = 0;
            self.cpu_triangles.iter_mut().for_each(|triangle|{
                let mut v1 = [0.0f32; 4];
                v1[..3].copy_from_slice(&triangle.v1);
                mapping.v1[counter] = v1;

                let mut v2 = [0.0f32; 4];
                v2[..3].copy_from_slice(&triangle.v2);
                mapping.v2[counter] = v2;

                let mut v3 = [0.0f32; 4];
                v3[..3].copy_from_slice(&triangle.v3);
                mapping.v3[counter] = v1;

                // seems as if this field is redundant in older dev_0.1??
                let mut norm = [0.0f32; 4];
                //norm[..3].copy_from_slice(&triangle.);
                mapping.norm[counter] = norm;

                let mut color = [0.0f32; 4];
                color[..3].copy_from_slice(&triangle.color);
                mapping.color_triangles[counter] = color;

                counter += 1;
            });
            
        }
        return triangle_array;
    }

    // temp solution until I get ssbo to work with glinum
    pub fn get_uniform_buffer_cubes(&mut self, display : &glium::Display<WindowSurface>) -> glium::uniforms::UniformBuffer<CubesArray>{
        
        let mut cube_array: glium::uniforms::UniformBuffer<CubesArray> = glium::uniforms::UniformBuffer::empty(display).unwrap();

        {
            let mut mapping = cube_array.map();
            let mut counter : usize = 0;
            self.cpu_cubes.iter_mut().for_each(|cube|{
                let mut pos = [0.0f32; 4];
                pos[..3].copy_from_slice(&cube.pos);
                mapping.pos_cubes[counter] = pos;

                let mut dim = [0.0f32; 4];
                dim[..3].copy_from_slice(&cube.dim);
                mapping.dim_cubes[counter] = dim;

                let mut color = [0.0f32; 4];
                color[..3].copy_from_slice(&cube.color);
                mapping.color_cubes[counter] = color;

                counter += 1;
            });
            
        }
        return cube_array;
    }

    pub fn get_uniform_buffer_menger_sponges(&mut self, display : &glium::Display<WindowSurface>) -> glium::uniforms::UniformBuffer<MengerSpongeArray>{
        
        let mut menger_sponge_array: glium::uniforms::UniformBuffer<MengerSpongeArray> = glium::uniforms::UniformBuffer::empty(display).unwrap();

        {
            let mut mapping = menger_sponge_array.map();
            let mut counter : usize = 0;
            self.cpu_menger_sponges.iter_mut().for_each(|menger_sponge|{
                let mut pos = [0.0f32; 4];
                pos[..3].copy_from_slice(&menger_sponge.pos);
                mapping.pos_menger_sponges[counter] = pos;

                let mut iterations = [0.0f32; 4];
                iterations[0] = menger_sponge.iterations;
                mapping.iterations_menger_sponges[counter] = iterations;

                let mut color = [0.0f32; 4];
                color[..3].copy_from_slice(&menger_sponge.color);
                mapping.color_menger_sponges[counter] = color;

                counter += 1;
            });
            
        }
        return menger_sponge_array;
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
