use std::ops::RangeInclusive;

use egui::{Context, Ui};
use egui_glium::*;
use glium::glutin::surface::WindowSurface;
use glutin::{event::{Event, WindowEvent}, event_loop::EventLoopWindowTarget};
use winit::window::Window;

use crate::{objectHandeler, ObjectHandeler};

pub struct GuiHandeler{
    egui_glium : EguiGlium
}

impl GuiHandeler{

    pub fn new(egui_glium_src : EguiGlium) -> GuiHandeler{
        
        GuiHandeler{
            egui_glium : egui_glium_src,
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
            

            //self.collapsing_objects_tree(egui_ctx, &mut objectHandeler);
            
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                
                // Temporary: This should really be inside a functions for simplicty - but the borrow checker goes mad with two mutable borrow with self
                ui.collapsing("Spheres", |ui_inside| { 
                    let spheres = objectHandeler.get_spheres_reference();
                    let mut id_counter = 0;
                    spheres.iter_mut().for_each(|sphere|{
                        ui_inside.collapsing(id_counter.to_string(), |ui_inside_inside|{
                            ui_inside_inside.label("Radius");
                            if ui_inside_inside.add(egui::widgets::Slider::new(&mut sphere.radius, RangeInclusive::new(0.0f32, 3.0f32)).min_decimals(2)).enabled(){
                                *should_update_objects = true;
                            };
                            ui_inside_inside.label("Color");
                            if egui::color_picker::color_edit_button_rgb(ui_inside_inside, &mut sphere.color).enabled(){
                                *should_update_objects = true;
                            };
                            ui_inside_inside.label("Position X");
                            ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[0], 0.0..=5.0).min_decimals(1));
                            ui_inside_inside.label("Position Y");
                            ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[1], 0.0..=5.0).min_decimals(1));
                            ui_inside_inside.label("Position Z");
                            ui_inside_inside.add(egui::Slider::new(&mut sphere.pos[2], 0.0..=5.0).min_decimals(1));
                            //response.on_hover_text("Drag me!");
        
                        });
                        id_counter += 1;
                    });
                });
            });
            

            /* 
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello World!");

                if ui.button("Quit").clicked() {
                    *should_quit = true;
                }

                //ui.button(RichText::new("delete").color(ui.visuals().warn_fg_color)).clicked();
                ui.heading("Not implemented logic yet - just for show!");
                ui.collapsing("Objects", |ui| { 

                    
                    ui.collapsing("Camera - TODO", |ui| { 
                        let mut custom_value = 10;
                        ui.label("Angle 1");
                        ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 360)));
                        ui.label("Angle 2");
                        ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 360)));
                        ui.label("Fov");
                        ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 1)));
   

                    });

                    // show objects
                    

                    /* 
                    ui.collapsing("Spheres", |ui| { 
                        ui.collapsing("Sphere1", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
                            

                        });

                        ui.collapsing("Sphere2", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
       

                        });

                        ui.collapsing("Sphere3", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
                            

                        });

                        ui.collapsing("Sphere4", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
       

                        });

                    });

                    ui.collapsing("Other type of object", |ui| { 
                        
                        ui.collapsing("Obj1", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
                            

                        });

                        ui.collapsing("Obj2", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
       

                        });

                        ui.collapsing("Obj3", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
                            

                        });

                        ui.collapsing("Obj", |ui| { 
                            let mut custom_value = 10;
                            ui.label("Radius");
                            ui.add(egui::widgets::Slider::new(&mut custom_value, RangeInclusive::new(0, 100)));
                            ui.label("Color");
                            egui::color_picker::color_edit_button_rgb(ui, &mut [0.0f32, 0.0f32, 0.0f32]);
                        });
                    });
                    */
                });
                */
            

            
            egui::TopBottomPanel::bottom("myawdwad_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello Worldawdwd!");

            });
            
        });
    }


    fn collapsing_objects_tree(&self, egui_ctx : &Context, objectHandeler : &mut ObjectHandeler){

        egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
            ui.collapsing("Spheres", |ui_inside| { 
                let spheres = objectHandeler.get_spheres_reference();
                let id_counter = 0;
                spheres.iter_mut().for_each(|sphere|{
                    ui_inside.collapsing(id_counter.to_string(), |ui_inside_inside|{
                        ui_inside_inside.label("Radius");
                        ui_inside_inside.add(egui::widgets::Slider::new(&mut sphere.radius, RangeInclusive::new(0f32, 100f32)));
                        ui_inside_inside.label("Color");
                        egui::color_picker::color_edit_button_rgb(ui_inside_inside, &mut sphere.color)
    
                    });
                });
            });
        });     
    }

}



struct CollapsingObjectsTree{

}

impl CollapsingObjectsTree{

}