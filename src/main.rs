mod globals;
mod objects;
mod user_input;

use globals::*;
use macroquad::prelude::*;
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

        if is_key_pressed(KEYB_SIMPLE_CIRCLE) {
            add_object_to_scene("circle_none");
        }

        next_frame().await;
    }
}
