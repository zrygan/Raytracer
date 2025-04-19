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
    action_utils::{
        object_at_cursor_index, object_at_cursor_type, print_all_objects, remove_object_at_index,
    },
    object_utils::init_all_rays,
};
use macroquad::prelude::*;
use macroquad::time::draw_fps;
use objects::emitters::*;
use objects::{behavior::*, occlusion::check_for_occlusion};
use std::{thread::sleep, time::Duration};
use user_input::{
    add_to_scene_actions::add_object_to_scene,
    emitter_actions::{object_change_orientation, object_change_size},
};

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
    let mut cursor_on_object_type: &'static str;
    let mut cursor_is_moving_object: bool = false;
    let mut mouse_x: f32;
    let mut mouse_y: f32;
    let mut mouse_delta: Vec2 = vec2(0.0, 0.0);
    let mut collection_size = 0;
    let mut ft;

    // print app information
    println!(
        "{} ver. {}\nBy: {}\nSource Available on: {}",
        APP_NAME, APP_VERSION, APP_AUTHOR, APP_GITHUB
    );

    loop {
        let keybind_increase_rays =
            is_key_pressed(KEYB_EMM_INC_RAYS) || is_key_down(KEYB_EMM_INC_RAYS);
        let keybind_decrease_rays =
            is_key_pressed(KEYB_EMM_DEC_RAYS) || is_key_down(KEYB_EMM_DEC_RAYS);

        let keybind_emitter_secondary_inc =
            is_key_pressed(KEYB_EMM_SEC_INC) || is_key_down(KEYB_EMM_SEC_INC);

        let keybind_emitter_secondary_dec =
            is_key_pressed(KEYB_EMM_SEC_DEC) || is_key_down(KEYB_EMM_SEC_DEC);

        ft = get_frame_time();
        // Clear the screen with the background color
        clear_background(WINDOW_BG_COLOR);
        draw_fps();
        (mouse_x, mouse_y) = mouse_position();

        // Handle user input for object creation
        if OBJC_MAX_OBJ_COUNT as usize > collection_size {
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
                collection_size += 1;
            } else if is_key_pressed(KEYB_EMITTER_ISOTROPIC) {
                println!(
                    "Raytracer Upd: Isotropic emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_isotropic");
                re_init_rays = true;
                collection_size += 1;
            } else if is_key_pressed(KEYB_EMITTER_COLLIMATED) {
                println!(
                    "Raytracer Upd: Collimated emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_collimated");
                re_init_rays = true;
                collection_size += 1;
            } else if is_key_pressed(KEYB_EMITTER_SPOTLIGHT) {
                println!(
                    "Raytracer Upd: Spotlight emitter object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("emitter_spotlight");
                re_init_rays = true;
                collection_size += 1;
            }
            // ============================================================
            // =============== INCREASE/DECREASE EMITTER RAYS
            // ============================================================
            else if keybind_increase_rays || keybind_decrease_rays {
                cursor_on_object_type = object_at_cursor_type(mouse_x, mouse_y, false);
                cursor_on_object_index = object_at_cursor_index(mouse_x, mouse_y);

                if cursor_on_object_type == "Emitter" {
                    if let Some(index) = cursor_on_object_index {
                        let mut collection = OBJ_COLLECTION.write().unwrap();

                        if let Some(RaytracerObjects::Emitters(o)) = collection.get_mut(index) {
                            let ray_delta = if keybind_increase_rays { 1 } else { -1 };
                            o.change_rays_count(ray_delta);

                            println!(
                                "Raytracer Upd: {} rays to Emitter object at {}, {}",
                                if ray_delta > 0 { "Adding" } else { "Reducing" },
                                mouse_x,
                                mouse_y
                            );

                            re_init_rays = true;
                        }
                    }
                }
            }
            // ============================================================
            // =============== INCREASE/DECREASE SECONDARY TRAIT
            // ===== COLLIMATED = Collimated Beam Diameter
            // ===== SPOTLIGHT  = Spotlight Beam Angle
            // ============================================================
            else if keybind_emitter_secondary_inc || keybind_emitter_secondary_dec {
                cursor_on_object_type = object_at_cursor_type(mouse_x, mouse_y, true);
                cursor_on_object_index = object_at_cursor_index(mouse_x, mouse_y);

                if let Some(index) = cursor_on_object_index {
                    let mut collection = OBJ_COLLECTION.write().unwrap();
                    if cursor_on_object_type == "Collimated" {
                        if let Some(RaytracerObjects::Emitters(Emitters::EmitterCollimated(o))) =
                            collection.get_mut(index)
                        {
                            let mut width_delta = if keybind_emitter_secondary_inc {
                                KEYB_EMM_SEC_COLL_WIDTH_DELTA
                            } else {
                                -KEYB_EMM_SEC_COLL_WIDTH_DELTA
                            };

                            if is_key_down(KeyCode::LeftShift) {
                                width_delta *= KEYB_RTC_MULTIPLIER
                            };

                            // Make sure we don't go below minimum width
                            if o.collimated_beam_diameter + width_delta as f32 <= 0.0
                                && width_delta < 0
                            {
                                println!("Raytracer ~Err: Cannot decrease beam diameter below 0");
                                // Skip the update
                            } else {
                                // Apply the width change
                                o.collimated_beam_diameter += width_delta as f32;

                                println!(
                                    "Raytracer Upd: {} collimated beam diameter to Emitter object at {}, {}",
                                    if width_delta > 0 {
                                        "Increasing"
                                    } else {
                                        "Decreasing"
                                    },
                                    mouse_x,
                                    mouse_y
                                );

                                re_init_rays = true;
                            }
                        }
                    } else if cursor_on_object_type == "Spotlight" {
                        if let Some(RaytracerObjects::Emitters(Emitters::EmitterSpotlight(o))) =
                            collection.get_mut(index)
                        {
                            let mut angle_delta = if keybind_emitter_secondary_inc {
                                KEYB_EMM_SEC_SPOT_ANGLE_DELTA
                            } else {
                                -KEYB_EMM_SEC_SPOT_ANGLE_DELTA
                            };

                            if is_key_down(KeyCode::LeftShift) {
                                angle_delta *= KEYB_RTC_MULTIPLIER as f32;
                            };

                            // Define min and max angle bounds (0 to 2π)
                            let min_angle: f32 = 0.0;
                            let max_angle: f32 = std::f32::consts::PI * 2.0; // 360 degrees in radians

                            // Check if new angle would be outside bounds
                            let new_angle = o.spotlight_beam_angle + angle_delta;

                            if new_angle < min_angle && angle_delta < 0.0 {
                                println!(
                                    "Raytracer ~Err: Cannot decrease spotlight beam angle below 0 radians"
                                );
                                // Skip the update
                            } else if new_angle > max_angle && angle_delta > 0.0 {
                                println!(
                                    "Raytracer ~Err: Cannot increase spotlight beam angle above 2π radians (360°)"
                                );
                                // Skip the update
                            } else {
                                // Apply the angle change
                                o.spotlight_beam_angle = new_angle;

                                println!(
                                    "Raytracer Upd: {} spotlight beam angle to Emitter object at {}, {} (current: {:.2} radians)",
                                    if angle_delta > 0.0 {
                                        "Increasing"
                                    } else {
                                        "Decreasing"
                                    },
                                    mouse_x,
                                    mouse_y,
                                    o.spotlight_beam_angle
                                );

                                re_init_rays = true;
                            }
                        }
                    }
                }
            }
            // ============================================================
            // =============== ABSORBERS
            // ============================================================
            else if is_key_pressed(KEYB_ABSORBER_PERFECT) {
                println!(
                    "Raytracer Upd: Perfect absorber object created at {}, {}",
                    mouse_x, mouse_y
                );
                add_object_to_scene("absorber_perfect");
                re_init_rays = true;
                collection_size += 1;
            }
            // ============================================================
            // =============== ENLARGE AND REDUCE
            // ============================================================
            else if is_key_down(KEYB_RTC_ENLARGE) || is_key_down(KEYB_RTC_SHRINK) {
                let cursor_object_type = object_at_cursor_type(mouse_x, mouse_y, false);
                if cursor_object_type != "None" {
                    let mut multiplier = if is_key_down(KEYB_RTC_ENLARGE) {
                        1.
                    } else {
                        -1.
                    };

                    if is_key_down(KeyCode::LeftShift) {
                        multiplier *= KEYB_RTC_MULTIPLIER as f32;
                    }

                    object_change_size(mouse_x, mouse_y, multiplier * OBJD_SIZE_DELTA_FACTOR);

                    println!(
                        "Raytracer Upd: {} object at {}, {}",
                        if multiplier == 1. {
                            "Enlarged"
                        } else {
                            "Shrunk"
                        },
                        mouse_x,
                        mouse_y
                    );
                    re_init_rays = true;
                } else {
                    println!(
                        "Raytracer ~Err: Failed to enlarge or shrink an object, there is no object at {}, {}",
                        mouse_x, mouse_y
                    );
                }
            }
            // ============================================================
            // =============== CHANGE ORIENTATION
            // ============================================================
            else if is_key_down(KEYB_RTC_INC_ORIENTATION) || is_key_down(KEYB_RTC_DEC_ORIENTATION)
            {
                if object_at_cursor_type(mouse_x, mouse_y, false) != "None" {
                    let mut delta = if is_key_down(KEYB_RTC_INC_ORIENTATION) {
                        OBJD_ORIENTATION_DELTA_FACTOR
                    } else {
                        -OBJD_ORIENTATION_DELTA_FACTOR
                    };

                    if is_key_down(KeyCode::LeftShift) {
                        delta *= KEYB_RTC_MULTIPLIER as f32;
                    }

                    object_change_orientation(mouse_x, mouse_y, delta);

                    println!(
                        "Raytracer Upd: {} orientation for object at {}, {}",
                        if delta > 0.0 {
                            "Increased"
                        } else {
                            "Decreased"
                        },
                        mouse_x,
                        mouse_y
                    );

                    re_init_rays = true;
                } else {
                    println!(
                        "Raytracer ~Err: Failed to change orientation, there is no object at {}, {}",
                        mouse_x, mouse_y
                    );
                }
            }
            // ============================================================
            // =============== DEBUG AND OTHER KEYBINDS
            // ============================================================
            else if is_key_pressed(KEYB_DELETE) {
                if collection_size >= 1 {
                    if let Some(i) = object_at_cursor_index(mouse_x, mouse_y) {
                        println!("Raytracer Upd: Deleted object at {}, {}", mouse_x, mouse_y);
                        remove_object_at_index(i);
                        re_init_rays = true;
                        collection_size -= 1;
                    } else {
                        println!(
                            "Raytracer ~Err: Failed to delete object, there is no object at {}, {}",
                            mouse_x, mouse_y
                        );
                    }
                } else {
                    println!(
                        "Raytracer ~Err: Failed to delete object, there is no object on the scene"
                    )
                }
            } else if is_key_pressed(KEYB_DEBUG_SHOW_ALL_OBJ) {
                println!("Raytracer Debug: Showing all objects inside OBJ_COLLECTION.");
                print_all_objects();
                println!("Raytracer Debug: Done showing all objects in OBJ_COLLECTION.");
            }
        } else {
            eprintln!(
                "Raytracer Err: Too many RaytracerObjects in the scene, you can only have {}",
                collection_size
            );
        }

        // Check if the user wants to move an object
        if is_mouse_button_down(MouseButton::Left) {
            cursor_on_object_index = object_at_cursor_index(mouse_x, mouse_y);
            if cursor_on_object_index.is_some() {
                cursor_is_moving_object = true
            }
        }

        // If the user is not moving an object, remove dragging_index
        if !is_mouse_button_down(MouseButton::Left) && cursor_is_moving_object == true {
            println!("Raytracer Upd: Stopped moving object.");
            cursor_is_moving_object = false;
        }

        // If user is moving the cursor and is dragging an object,
        // move that object
        if mouse_delta != vec2(0.0, 0.0) && cursor_is_moving_object {
            if let Some(index) = cursor_on_object_index {
                let mut collection = OBJ_COLLECTION.write().unwrap();
                if let Some(object) = collection.get_mut(index) {
                    match object {
                        RaytracerObjects::ObjectCircle(o) => {
                            o.move_object(mouse_x, mouse_y);
                        }
                        RaytracerObjects::Emitters(o) => {
                            o.move_object(mouse_x, mouse_y);
                        }
                        RaytracerObjects::Absorbers(o) => {
                            o.move_object(mouse_x, mouse_y);
                        }
                    }
                    re_init_rays = true;
                }
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
        for r_obj in OBJ_COLLECTION.read().unwrap().iter() {
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

        if (ft < WINDOW_FRAME_RATE) && WINDOW_USE_FRAME_RATE {
            sleep(Duration::from_millis(
                ((WINDOW_FRAME_RATE - ft) * 1000.) as u64,
            ));
        }

        next_frame().await;
    }
}
