mod globals;
mod objects;
mod user_input;

use globals::*;
use macroquad::prelude::*;
use objects::behavior::{Drawable, RaytracerObjects};
use user_input::actions::add_object_to_scene;

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
            add_object_to_scene("circle_none");
        }

        // Drawing the RaytracerObjects
        for r_object in OBJC_COLLECTION.lock().unwrap().iter() {
            match r_object {
                RaytracerObjects::ObjectCircle(object_circle) => {
                    object_circle.draw_object();
                }
            }
            
        }

        next_frame().await;
    }
}
