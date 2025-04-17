mod globals;
mod objects;
mod user_input;

use globals::*;
use macroquad::prelude::*;
use user_input::actions::add_object_to_scene;
use objects::behavior::*;

fn window_conf() -> Conf {
    Conf {
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_title: format!("{} [{}]", APP_NAME, APP_VERSION),
        high_dpi: MACROQUAD_HIGH_DPI,
        fullscreen: MACROQUAD_FULLSCREEN,
        sample_count: MACROQUAD_SAMPLE_COUNT,
        window_resizable: MACROQUAD_RESIZEABLE,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(WINDOW_BG_COLOR);

        // User input handling
        if is_key_pressed(KEYB_SIMPLE_CIRCLE) {
            println!("Simple circle created at {}, {}", mouse_position().0, mouse_position().1);
            add_object_to_scene("circle_none");
        } else if is_key_pressed(KEYB_EMITTER_ISOTROPIC){
            println!("Isotropic emitter object created at {}, {}", mouse_position().0, mouse_position().1);
            add_object_to_scene("emitter_isotropic");
        } else if is_key_pressed(KEYB_EMITTER_COLLIMATED){
            println!("Collimated emitter object created at {}, {}", mouse_position().0, mouse_position().1);
            add_object_to_scene("emitter_collimated");
        }

        // Draw Raytracer Objects
        for r_obj in OBJ_COLLECTION.lock().unwrap().iter(){
            match r_obj {
                RaytracerObjects::ObjectCircle(o) => {
                    o.draw_object();
                }, 
                RaytracerObjects::Emitter(o) => {
                    o.draw_object();
                },
                RaytracerObjects::EmitterCollimated(o) => {
                    o.base_emitter.draw_object();
                }
            }
        }

        next_frame().await;
    }
}
