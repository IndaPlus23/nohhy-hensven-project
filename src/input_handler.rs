use egui;
use crate::camera::Camera;
use crate::{vec_util::*, ObjectHandeler};

pub struct InputHandler {
    rotation_pos : [f32;3]
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            rotation_pos : [0., 1., 0.]
        }
    }

    pub fn handle(&mut self, ctx : &egui::Context, camera : &mut Camera, object_handler : &mut ObjectHandeler) {
        self.rotate_camera(ctx, camera, 0.00009);
        self.move_camera(ctx, camera, 0.00015);
        self.zoom(ctx, camera, 0.05);
        self.keyboard_inputs(ctx, object_handler);
    }

    fn zoom(&mut self, ctx : &egui::Context, camera : &mut Camera, sensitivity : f32) {
        ctx.input(|i| {
            let scroll_amount = -i.raw_scroll_delta[1];

            let new_fov = camera.fov + scroll_amount * sensitivity;

            if new_fov >= 0.001 && new_fov <= 100000.0 {
                camera.fov = new_fov;
            }
        })
    }

    fn rotate_camera(&mut self, ctx : &egui::Context, camera : &mut Camera, sensitivity : f32) {
        ctx.input(|i| {
            let ptr_state = &i.pointer;

            let vel = ptr_state.velocity();
            
            if !(vel.x == 0. && vel.y == 0.) {
                if ptr_state.primary_down() {
                    camera.rotate_around_obj(self.rotation_pos, -sensitivity * vel.x);
                }
            }

        })
    }

    fn move_camera(&mut self, ctx : &egui::Context, camera : &mut Camera, sensitivity : f32) {
        ctx.input(|i| {
            let ptr_state = &i.pointer;

            if ptr_state.middle_down() {
                let vel = ptr_state.velocity();

                let x_dir = rotate_vec2([1., 0.], camera.angle);
                let z_dir = rotate_vec2([0., 1.], camera.angle);

                let x_move = vec_mul(x_dir, vel.x);
                let z_move = vec_mul(z_dir, vel.y);

                let tot_move = vec2_add(x_move, z_move, 1.);

                camera.pos[0] += tot_move[0] * -sensitivity;
                camera.pos[2] += tot_move[1] * sensitivity;
            }
        })
    }

    fn keyboard_inputs(&mut self, ctx : &egui::Context, object_handler : &mut ObjectHandeler) {
        ctx.input(|i| {
            let keys_down = &i.keys_down;

            if keys_down.contains(&egui::Key::Num1) {
                object_handler.set_render_mode(0);
            }
            if keys_down.contains(&egui::Key::Num2) {
                object_handler.set_render_mode(1);
            }
            if keys_down.contains(&egui::Key::Num3) {
                object_handler.set_render_mode(2);
            }
        })
    }
}