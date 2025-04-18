//! Raytracer - Interactive 2D ray tracing simulation
//!
//! This is the main module of the Raytracer application, which handles window
//! configuration, the main event loop, user input processing, and rendering.
//! It serves as the entry point and orchestrator for the entire application.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 18, 2025

mod globals;
mod helpers;
mod objects;
mod user_input;

use globals::*;
use helpers::{
    action_utils::{object_at_cursor, print_all_objects, remove_object_at_index},
    object_utils::init_all_rays,
};
use macroquad::prelude::*;
use objects::{behavior::*, occlusion::check_for_occlusion};
use std::thread::sleep;
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
    // if any object is moved, set this to true this is for occlusion.
    // so that we dont re-initialize all rays per frame, only when an absorber
    // is moved.
    let mut re_init_rays: bool = false;

    let mut cursor_on_object_index: Option<usize> = None;
    let mut mouse_x: f32;
    let mut mouse_y: f32;
    let mut mouse_delta: Vec2 = vec2(0.0, 0.0);

    // print app information
    println!(
        "{} ver. {}\nBy: {}\nSource Available on: {}",
        APP_NAME, APP_VERSION, APP_AUTHOR, APP_GITHUB
    );

    loop {
        // Clear the screen with the background color
        clear_background(WINDOW_BG_COLOR);
        (mouse_x, mouse_y) = mouse_position();

        // Handle user input for object creation
        if OBJC_MAX_OBJ_COUNT as usize > OBJ_COLLECTION.lock().iter().len() {
            // ============================================================
            // =============== EMITTERS
            // ============================================================
            if is_key_pressed(KEYB_SIMPLE_CIRCLE) {
                println!(
                    "Raytracer Upd: Simple circle created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("circle_none");
                re_init_rays = true;
            } else if is_key_pressed(KEYB_EMITTER_ISOTROPIC) {
                println!(
                    "Raytracer Upd: Isotropic emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_isotropic");
                re_init_rays = true;
            } else if is_key_pressed(KEYB_EMITTER_COLLIMATED) {
                println!(
                    "Raytracer Upd: Collimated emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_collimated");
                re_init_rays = true;
            } else if is_key_pressed(KEYB_EMITTER_SPOTLIGHT) {
                println!(
                    "Raytracer Upd: Spotlight emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_spotlight");
                re_init_rays = true;
            // ============================================================
            // =============== ABSORBERS
            // ============================================================
            } else if is_key_pressed(KEYB_ABSORBER_PERFECT) {
                println!(
                    "Raytracer Upd: Perfect absorber object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("absorber_perfect");
                re_init_rays = true;
            // ============================================================
            // =============== DEBUG AND OTHER KEYBINDS
            // ============================================================
            } else if is_key_pressed(KEYB_DELETE) {
                if let Some(i) = object_at_cursor(mouse_x, mouse_y) {
                    remove_object_at_index(i);
                    println!("Raytracer Upd: Deleted object at {}, {}", mouse_x, mouse_y);
                };
                re_init_rays = true;
            } else if is_key_pressed(KEYB_DEBUG_SHOW_ALL_OBJ) {
                println!("Raytracer Debug: Showing all objects inside OBJ_COLLECTION.");
                print_all_objects();
                println!("Raytracer Debug: Done showing all objects in OBJ_COLLECTION.");
            }
        } else {
            eprintln!(
                "Raytracer Err: Too many RaytracerObjects in the scene, you can only have {}",
                OBJ_COLLECTION.lock().iter().len()
            );
        }

        // Check if the user wants to move an object
        if is_mouse_button_down(MouseButton::Left) {
            cursor_on_object_index = object_at_cursor(mouse_x, mouse_y);
        }

        // If the user is not moving an object, remove dragging_index
        if !is_mouse_button_down(MouseButton::Left) && (cursor_on_object_index.is_some()) {
            println!("Raytracer Upd: Stopped moving object.");
            cursor_on_object_index = None;
        }

        // If user is moving the cursor and is dragging an object,
        // move that object
        if mouse_delta != vec2(0.0, 0.0) && cursor_on_object_index.is_some() {
            let mut collection = OBJ_COLLECTION.lock().unwrap();
            if let Some(raytracer_object) = collection.get_mut(cursor_on_object_index.unwrap()) {
                match raytracer_object {
                    RaytracerObjects::ObjectCircle(object) => object.move_object(mouse_x, mouse_y),
                    RaytracerObjects::Emitters(object) => object.move_object(mouse_x, mouse_y),
                    RaytracerObjects::Absorbers(object) => {
                        object.move_object(mouse_x, mouse_y);
                    }
                }
                re_init_rays = true;
            }
        }

        if re_init_rays {
            // re-initialize all rays
            init_all_rays();

            // Check for occlusion
            check_for_occlusion();

            re_init_rays = false;
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
                RaytracerObjects::Absorbers(object) => {
                    object.draw_object();
                }
            }
        }

        mouse_delta = mouse_delta_position();

        if WINDOW_SLEEP_BOOLEAN {
            sleep(WINDOW_SLEEP_DURATION);
        }

        next_frame().await;
    }
}
