use egui::Ui;

use crate::{shapes::{Cube, Sphere}, ObjectHandeler};



pub struct CreateRenderObjectGui<'a>{
    sphere : Sphere, 
    cube : Cube,
    selected : &'a str


}

impl CreateRenderObjectGui<'_>{

    pub fn new() -> CreateRenderObjectGui<'static>{
        CreateRenderObjectGui{
            sphere : Sphere::new([0.0; 3], [0.5; 3], 1.0),
            cube : Cube::new([0.0; 3], [1.0; 3], [0.5; 3]),
            selected : "Sphere"
        }   
    }

    fn create_new_shell_objects(&mut self){
        self.sphere = Sphere::new([0.0; 3], [0.5; 3], 1.0);
        self.cube = Cube::new([0.0; 3], [1.0; 3], [0.5; 3]);
    }

    pub fn show(&mut self, create_object_gui_active : &mut bool, ui : &mut Ui, object_handeler : &mut ObjectHandeler, mut should_update_objects : &mut bool){

        let items = vec!["Sphere", "Cube"];
        let mut str = String::from("3.0");
        match create_object_gui_active{
            true => {
                ui.label("Creating new object");
                egui::ComboBox::from_label("Select object type!")
                .selected_text(self.selected)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, "Sphere", "Sphere");
                    ui.selectable_value(&mut self.selected, "Cube", "Cube");
                });

                match self.selected {
                    "Sphere" => {

                        // to tweak coordinates/posistion
                        let mut coord_x = self.sphere.pos[0].to_string();
                        let mut coord_y = self.sphere.pos[1].to_string();
                        let mut coord_z = self.sphere.pos[2].to_string();

                        ui.label("x coord");
                        egui::TextEdit::singleline(&mut coord_x).show(ui);
                        ui.label("y coord");
                        egui::TextEdit::singleline(&mut coord_y).show(ui);
                        ui.label("z coord");
                        egui::TextEdit::singleline(&mut coord_z).show(ui);

                        // convert back
                        self.sphere.pos[0] = coord_x.parse::<f32>().unwrap();
                        self.sphere.pos[1] = coord_y.parse::<f32>().unwrap();
                        self.sphere.pos[2] = coord_z.parse::<f32>().unwrap();


                        ui.label("Radius");
                        egui::TextEdit::singleline(&mut str).show(ui);
                        ui.label("Color");
                        egui::color_picker::color_edit_button_rgb(ui, &mut self.sphere.color);

                        if ui.button("Create object").clicked(){
                            *create_object_gui_active = false;
                            object_handeler.add_sphere(self.sphere);
                            *should_update_objects = true; // to make sure that main loop re-uploads objects to scene
                        }
                    }, 
                    &_=> {

                        // to tweak coordinates/position
                        let mut coord_x = self.cube.pos[0].to_string();
                        let mut coord_y = self.cube.pos[1].to_string();
                        let mut coord_z = self.cube.pos[2].to_string();

                        ui.label("x coord");
                        egui::TextEdit::singleline(&mut coord_x).show(ui);
                        ui.label("y coord");
                        egui::TextEdit::singleline(&mut coord_y).show(ui);
                        ui.label("z coord");
                        egui::TextEdit::singleline(&mut coord_z).show(ui);

                        // convert back
                        self.cube.pos[0] = coord_x.parse::<f32>().unwrap();
                        self.cube.pos[1] = coord_y.parse::<f32>().unwrap();
                        self.cube.pos[2] = coord_z.parse::<f32>().unwrap();

                        // to tweak coords/pos
                        let mut dim_x = self.cube.dim[0].to_string();
                        let mut dim_y = self.cube.dim[1].to_string();
                        let mut dim_z = self.cube.dim[2].to_string();

                        ui.label("Dimensions");
                        ui.label("x");
                        egui::TextEdit::singleline(&mut dim_x).show(ui);
                        ui.label("y");
                        egui::TextEdit::singleline(&mut dim_y).show(ui);
                        ui.label("z");
                        egui::TextEdit::singleline(&mut dim_z).show(ui);

                        // convert back
                        self.cube.dim[0] = dim_x.parse::<f32>().unwrap();
                        self.cube.dim[1] = dim_y.parse::<f32>().unwrap();
                        self.cube.dim[2] = dim_z.parse::<f32>().unwrap();

                        ui.label("Color");
                        egui::color_picker::color_edit_button_rgb(ui, &mut self.cube.color);

                        if ui.button("Create object").clicked(){
                            *create_object_gui_active = false;  // changes visibility of this gui code
                            object_handeler.add_cube(self.cube);
                            *should_update_objects = true;  // to make sure that main loop re-uploads objects to scene
                        }
                    }
                }

                if ui.button("Discard object").clicked(){
                    *create_object_gui_active = false; // changes visibility of this gui code
                }

            }, 
            false => {
                if ui.button("Open object creator").clicked(){
                    *create_object_gui_active = true; // changes visibility of this gui code
                }
            }
        }  
    }
}