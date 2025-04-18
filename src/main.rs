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
use macroquad::prelude::{camera::mouse, *};
use objects::{behavior::*, emitters::*};
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
        ..Default::default() // the rest are left to be the default values
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
    // main function variables
    let mut dragged_index: Option<usize> = None;
    let mut mouse_x: f32;
    let mut mouse_y: f32;
    let mut mouse_delta: Vec2 = vec2(0.0, 0.0);

    // print app information
    println!(
        "{} ver. {}\nBy: {}\nSource Available on: {}",
        APP_NAME, APP_VERSION, APP_AUTHOR, APP_GITHUB
    );

    // Clear the screen with the background color
    clear_background(WINDOW_BG_COLOR);

    loop {
        (mouse_x, mouse_y) = mouse_position();

        // Handle user input for object creation
        if OBJC_MAX_OBJ_COUNT as usize > OBJ_COLLECTION.lock().iter().len() {
            if is_key_pressed(KEYB_SIMPLE_CIRCLE) {
                println!(
                    "Raytracer Upd: Simple circle created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("circle_none");
            } else if is_key_pressed(KEYB_EMITTER_ISOTROPIC) {
                println!(
                    "Raytracer Upd: Isotropic emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_isotropic");
            } else if is_key_pressed(KEYB_EMITTER_COLLIMATED) {
                println!(
                    "Raytracer Upd: Collimated emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_collimated");
            } else if is_key_pressed(KEYB_EMITTER_SPOTLIGHT) {
                println!(
                    "Raytracer Upd: Spotlight emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_spotlight");
            }

            // Draw all objects in the global collection
            for r_obj in OBJ_COLLECTION.lock().unwrap().iter() {
                match r_obj {
                    RaytracerObjects::ObjectCircle(object) => {
                        object.draw_object();
                    }
                    RaytracerObjects::Emitters(object) => {
                        object.draw_object();
                    }
                }
            }
        } else {
            println!(
                "Raytracer Err: Too many RaytracerObjects in the scene, you can only have {}",
                OBJ_COLLECTION.lock().iter().len()
            );
        }

        // Check if the user wants to move an object
        if is_mouse_button_down(MouseButton::Left) {
            let temp = OBJ_COLLECTION.lock().unwrap();

            for (index, object) in temp.iter().enumerate() {
                let (x, y) = object.get_pos();
                if (mouse_x - x).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
                    && (mouse_y - y).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
                {
                    dragged_index = Some(index);
                    println!("Raytracer Upd: Moving object {:#?}", object);
                    break;
                }
            }
        }

        // If the user is not moving an object, remove dragging_index
        if !is_mouse_button_down(MouseButton::Left) & (dragged_index.is_some()) {
            println!("Raytracer Upd: Stopped moving object.");
            dragged_index = None;
        }

        // If user is moving the cursor and is dragging an object,
        // move that object
        if mouse_delta != vec2(0.0, 0.0) && dragged_index.is_some() {
            let mut raytracer_object = {
                let collection = OBJ_COLLECTION.lock().unwrap();
                collection[dragged_index.unwrap()].clone()
            };

            match &mut raytracer_object {
                RaytracerObjects::ObjectCircle(object) => object.move_object(mouse_x, mouse_y),
                RaytracerObjects::Emitters(emitter) => match emitter {
                    Emitters::Emitter(object) => object.move_object(mouse_x, mouse_y),
                    Emitters::EmitterCollimated(object) => object
                        .base_emitter
                        .base_object
                        .move_object(mouse_x, mouse_y),
                    Emitters::EmitterSpotlight(object) => object
                        .base_emitter
                        .base_object
                        .move_object(mouse_x, mouse_y),
                },
            }
        }

        mouse_delta = mouse_delta_position();

        // Wait for the next frame
        next_frame().await;
    }
}
