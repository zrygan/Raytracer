//! Raytracer - Interactive 2D ray tracing simulation
//!
//! This is the main module of the Raytracer application, which handles window
//! configuration, the main event loop, user input processing, and rendering.
//! It serves as the entry point and orchestrator for the entire application.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 17, 2025

mod globals;
mod objects;
mod user_input;

use globals::*;
use macroquad::prelude::*;
use objects::behavior::*;
use user_input::actions::add_object_to_scene;

/// Configures the application window settings.
///
/// This function defines all window properties including dimensions, title,
/// and rendering options. It uses constants from the globals module to ensure
/// consistent configuration throughout the application.
///
/// # Returns
///
/// A `Conf` struct with all window configuration parameters set
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

/// Main entry point for the Raytracer application.
///
/// This async function initializes the application window and runs the main event loop.
/// The loop handles:
/// 1. Clearing the background for each frame
/// 2. Processing user input for object creation
/// 3. Drawing all objects in the scene
/// 4. Advancing to the next frame
///
/// The function is marked as the application entry point using the `#[macroquad::main]`
/// attribute, which initializes the Macroquad rendering environment.
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        // Clear the screen with the background color
        clear_background(WINDOW_BG_COLOR);

        // Handle user input for object creation
        if is_key_pressed(KEYB_SIMPLE_CIRCLE) {
            println!(
                "Simple circle created at {}, {}",
                mouse_position().0,
                mouse_position().1
            );
            add_object_to_scene("circle_none");
        } else if is_key_pressed(KEYB_EMITTER_ISOTROPIC) {
            println!(
                "Isotropic emitter object created at {}, {}",
                mouse_position().0,
                mouse_position().1
            );
            add_object_to_scene("emitter_isotropic");
        } else if is_key_pressed(KEYB_EMITTER_COLLIMATED) {
            println!(
                "Collimated emitter object created at {}, {}",
                mouse_position().0,
                mouse_position().1
            );
            add_object_to_scene("emitter_collimated");
        } else if is_key_pressed(KEYB_EMITTER_SPOTLIGHT) {
            println!(
                "Spotlight emitter object created at {}, {}",
                mouse_position().0,
                mouse_position().1
            );
            add_object_to_scene("emitter_spotlight");
        }

        // Draw all objects in the global collection
        for r_obj in OBJ_COLLECTION.lock().unwrap().iter() {
            match r_obj {
                RaytracerObjects::ObjectCircle(o) => {
                    o.draw_object();
                }
                RaytracerObjects::Emitters(o) => {
                    o.draw_object();
                }
            }
        }

        // Wait for the next frame
        next_frame().await;
    }
}
