use std::ops::RangeInclusive;

use egui_glium::*;
use glium::glutin::surface::WindowSurface;
use glutin::{event::{Event, WindowEvent}, event_loop::EventLoopWindowTarget};
use winit::window::Window;

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


    pub fn update_gui(&mut self, window : &Window, should_quit : &mut bool){


        self.egui_glium.run(&window, |egui_ctx| {

            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello World!");

                if ui.button("Quit").clicked() {
                    *should_quit = true;
                }

                //ui.button(RichText::new("delete").color(ui.visuals().warn_fg_color)).clicked();
                ui.heading("Not implemented logic yet - just for show!");
                ui.collapsing("Objects", |ui| { 
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
                });
            });

            egui::TopBottomPanel::bottom("myawdwad_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello Worldawdwd!");

            });
        });
    }

}