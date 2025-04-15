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
            add_object_to_scene("circle_none");
        }

        // Draw Raytracer Objects
        for r_obj in OBJC_COLLECTION.lock().unwrap().iter(){
            match r_obj {
                RaytracerObjects::ObjectCircle(object_circle) => {
                    object_circle.draw_object();
                }
            }
        }

        next_frame().await;
    }
}
