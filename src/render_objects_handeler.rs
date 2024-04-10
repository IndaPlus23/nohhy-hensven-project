use crate::{shapes::ToGl, ssbo::Ssbo};
use crate::shapes::{Sphere, Triangle};



pub struct ObjectHandeler{

    // ssbo for gpu storage
    ssbo_triangles : Ssbo,
    ssbo_spheres : Ssbo,

    // for cpu storage
    cpu_triangles : Vec<Triangle>,
    cpu_spheres : Vec<Sphere>,

    // other stuff
    data_is_modified : bool,
}

impl ObjectHandeler{

    pub fn new() -> ObjectHandeler{
        ObjectHandeler{
            ssbo_triangles : Ssbo::new(10),
            ssbo_spheres : Ssbo::new(11),
            cpu_triangles : Vec::new(),
            cpu_spheres : Vec::new(),
            data_is_modified : false
        }
    }

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

    pub fn add_triangles_from(&mut self, render_objects : &mut Vec<Triangle>){
        self.cpu_triangles.append(render_objects);
        self.data_is_modified = true;
    }

    pub fn add_spheres_from(&mut self, render_objects : &mut Vec<Sphere>){
        self.cpu_spheres.append(render_objects);
        self.data_is_modified = true;
    }

    // TODO: remove()


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

    fn transfer_cpu_data_to_gpu(&mut self){
        // TODO: should be optimized in the future, not really neccasary at all times to redraw/resend all data, all data has probobly not been modified
        self.ssbo_triangles.initalize(self.cpu_triangles.clone()); // clone really neccasary?
        self.ssbo_spheres.initalize(self.cpu_spheres.clone());
    }
}