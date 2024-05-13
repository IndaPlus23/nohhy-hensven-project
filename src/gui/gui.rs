


use std::ops::RangeInclusive;

use egui::{popup, ComboBox, Context, Ui};
use egui_glium::*;
use glium::glutin::{api::egl::display, surface::WindowSurface};
use glutin::{event::{Event, WindowEvent}, event_loop::EventLoopWindowTarget};
use winit::window::Window;

use crate::{objectHandeler, shapes::Sphere, ObjectHandeler, specificGuiFunctionality::*};

use egui_snarl::{ui::{SnarlStyle, SnarlViewer}, *};
struct StateHandeler{

    pub create_object : bool,
    /*...... */
}

impl StateHandeler{
    fn new() -> StateHandeler{
        return StateHandeler{
            create_object : false
        };
    }
}

pub struct GuiHandeler<'a>{
    egui_glium : EguiGlium,
    state_handeler : StateHandeler,
    create_object_gui : CreateRenderObjectGui<'a>
}

impl GuiHandeler<'_>{

    pub fn new(egui_glium_src : EguiGlium) -> GuiHandeler<'static>{
        
        GuiHandeler{
            egui_glium : egui_glium_src,
            state_handeler : StateHandeler::new(),
            create_object_gui: CreateRenderObjectGui::new()
        }
    }

    pub fn render<T : glium::Surface>(&mut self, display : &glium::Display<WindowSurface>, target : &mut T){
        self.egui_glium.paint(display, target);
    }

    pub fn get_responce(&mut self, window : &Window, event : &winit::event::WindowEvent) -> EventResponse{
        return self.egui_glium.on_event(&window, &event);
    }

    pub fn update_gui(&mut self, window : &Window, should_quit : &mut bool, mut objectHandeler : &mut ObjectHandeler, mut should_update_objects : &mut bool){

        *should_update_objects = false;

        self.egui_glium.run(&window, |egui_ctx| {
            
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {

                // objects present in scene
                Self::collapsing_objects_tree(objectHandeler, should_update_objects, ui);

                // Adding space
                ui.add_space(15.0);
                ui.separator();

                // gui to create object
                self.create_object_gui.show(&mut self.state_handeler.create_object, ui, objectHandeler, should_update_objects);

            });
            egui::TopBottomPanel::bottom("myawdwad_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello Worldawdwd!");

            });
            
        });
    }


    fn collapsing_objects_tree(objectHandeler : &mut ObjectHandeler, mut should_update_objects : &mut bool, ui : &mut Ui){

        ui.collapsing("Spheres", |ui_inside| { 
            let spheres = objectHandeler.get_spheres_reference();
            let mut id_counter = 0;

            for mut i in 0..spheres.len() {
                let mut break_ = false;
                ui_inside.collapsing(id_counter.to_string(), |ui_inside_inside|{

                    {
                        let sphere = spheres.get_mut(i).unwrap();
                        ui_inside_inside.label("Radius");
                        if ui_inside_inside.add(egui::widgets::Slider::new(&mut sphere.radius, RangeInclusive::new(0.0f32, 3.0f32)).min_decimals(2)).enabled(){
                            *should_update_objects = true;
                        };
                        ui_inside_inside.label("Color");
                        if egui::color_picker::color_edit_button_rgb(ui_inside_inside, &mut sphere.color).enabled(){
                            *should_update_objects = true;
                        };
                        ui_inside_inside.label("Position X");
                        ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[0], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Position Y");
                        ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[1], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Position Z");
                        ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[2], -5.0..=5.0).min_decimals(1));

                    }
                    
                    if ui_inside_inside.button("Remove").clicked(){
                        spheres.remove(i); // thus remove this sphere object
                        break_ = true;
                    }
                });

                if break_{break;}
                id_counter += 1;

            }
            
        });

        ui.collapsing("Cubes", |ui_inside| { 
            let cubes = objectHandeler.get_cubes_reference();
            let mut id_counter = 0;

            for i in 0..cubes.len() {
                ui_inside.collapsing(id_counter.to_string(), |ui_inside_inside|{

                    {
                        let cube = cubes.get_mut(i).unwrap();
                        ui_inside_inside.label("Color");
                        if egui::color_picker::color_edit_button_rgb(ui_inside_inside, &mut cube.color).enabled(){
                            *should_update_objects = true;
                        };
                        ui_inside_inside.label("Position X");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.pos[0], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Position Y");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.pos[1], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Position Z");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.pos[2], -5.0..=5.0).min_decimals(1));

                        ui_inside_inside.label("Dim X");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.dim[0], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Dim Y");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.dim[1], -5.0..=5.0).min_decimals(1));
                        ui_inside_inside.label("Dim Z");
                        ui_inside_inside.add(egui::Slider::new(&mut cube.dim[2], -5.0..=5.0).min_decimals(1));

                    }
                    
                    if ui_inside_inside.button("Remove").clicked(){
                        cubes.remove(i); // thus remove this sphere object
                    }
                });
                id_counter += 1;

            }
        });

        /* 
        ui.collapsing("Groups", |ui_inside| { 

            ui_inside.collapsing("Group 1", |ui_inside_inside|{

                ui_inside_inside.label("Relations");
                ui_inside_inside.collapsing("Relation 1", |ui_inside_inside_inside|{
                    ui_inside_inside_inside.label("Type: Union");
                    if ui_inside_inside_inside.button("Tweak relation").clicked(){
            
                    }
                });

                ui_inside_inside.collapsing("Relation 2", |ui_inside_inside_inside|{
                    ui_inside_inside_inside.label("Type: Intersection");
                    if ui_inside_inside_inside.button("Tweak relation").clicked(){
            
                    }
                });

                ui_inside_inside.separator();
                ui_inside_inside.label("Objects");


            });

            ui_inside.collapsing("Group 2", |ui_inside_inside|{

            });

        });

        if ui.button("Create group").clicked(){
            
        }
        */
    }

}
