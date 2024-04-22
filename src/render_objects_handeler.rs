use crate::{shapes::ToGl, ssbo::Ssbo};
use crate::shapes::{Sphere, Box};



pub struct ObjectHandeler{

    // ssbo for gpu storage
    ssbo_boxes : Ssbo,
    ssbo_spheres : Ssbo,

    // for cpu storage
    cpu_boxes : Vec<Box>,
    cpu_spheres : Vec<Sphere>,

    // other stuff
    data_is_modified : bool,
}

impl ObjectHandeler{

    pub fn new() -> ObjectHandeler{
        ObjectHandeler{
            ssbo_boxes : Ssbo::new(11),
            ssbo_spheres : Ssbo::new(10),
            cpu_boxes : Vec::new(),
            cpu_spheres : Vec::new(),
            data_is_modified : false
        }
    }

    /*
    fn add_render_object<T : ToGl>(&mut self, render_object : T){

    }
    */


    pub fn add_box(&mut self, render_object : Box){
        self.cpu_boxes.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_sphere(&mut self, render_object : Sphere){
        self.cpu_spheres.push(render_object);
        self.data_is_modified = true;
    }

    pub fn add_boxes_from(&mut self, render_objects : &mut Vec<Box>){
        self.cpu_boxes.append(render_objects);
        self.data_is_modified = true;
    }

    pub fn add_spheres_from(&mut self, render_objects : &mut Vec<Sphere>){
        self.cpu_spheres.append(render_objects);
        self.data_is_modified = true;
    }

    pub fn set_boxes(&mut self, render_objects : Vec<Box>){
        self.cpu_boxes= render_objects;
        self.data_is_modified = true;
    }

    pub fn set_spheres(&mut self, render_objects : Vec<Sphere>){
        self.cpu_spheres = render_objects;
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
        self.ssbo_boxes.initalize(self.cpu_boxes.clone()); // clone really neccasary?
        self.ssbo_spheres.initalize(self.cpu_spheres.clone());
    }
}