use egui;
use crate::camera::Camera;
use crate::vec_util::*;

fn get_basis_vectors(rotation_quat : [f32;4]) -> ([f32;3], [f32;3]) {
    let mut v1 = [1., 0., 0.];
    let mut v2 = [0., 1., 0.];

    v1 = rotate_pos(v1, rotation_quat);
    v2 = rotate_pos(v2, rotation_quat);

    (normalize(v2), normalize(v1))
}

/// v : mouse velocity
/// p : camera position
/// o : object position
fn get_rotation_axis(v : [f32;2], center_pos : [f32;3], camera : &Camera) -> [f32;3]{
    let n = vec_add(center_pos, camera.pos, -1.);
    let (v1, v2) = get_basis_vectors(camera.get_rotation_quaternion());

    let v_b = vec_add(
        [v[0] * v1[0], v[0] * v1[1], v[0] * v1[2]], 
        [v[1] * v2[0], v[1] * v2[1], v[1] * v2[2]], 
        1.
    );

    return v1;
}

pub struct MouseHandler {
    rotation_axis : [f32;3],
    update_axis : bool
}

impl MouseHandler {
    pub fn new() -> Self {
        MouseHandler {
            rotation_axis : [0., 1., 0.],
            update_axis : true
        }
    }
    pub fn move_camera(&mut self, ctx : &egui::Context, camera : &mut Camera, sensitivity : f32) {
        ctx.input(|i| {
            let ptr_state = &i.pointer;

            let vel = ptr_state.velocity();
            
            if !(vel.x == 0. && vel.y == 0.) {
                if ptr_state.any_down() {
                    if self.update_axis {
                        self.rotation_axis = get_rotation_axis([vel.x, vel.y], [0., 1., 0.], &camera);
                        self.update_axis = false;
                    
                    }
                    // camera.rotate_around_obj(&[0., 0., 0.], &[0., 1., 0.], -sensitivity * vel.x);
                    camera.rotation_axis = self.rotation_axis;
                    // println!("{:?}", camera.rotation_axis);
                    camera.rotate_around_obj([0., 0., 0.], -sensitivity * vel.x);
                } else {
                    self.update_axis = true;
                }
            }

        })
    }
}