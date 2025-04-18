//! User input action handlers
//!
//! This module contains functions that respond to user interactions by creating
//! and manipulating objects in the raytracer scene.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 17, 2025

use crate::globals::{
    OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS, OBJD_COLLIMATED_BEAM_DIAMETER,
    OBJD_COLLIMATED_ORIENTATION, OBJD_SPOTLIGHT_BEAM_ANGLE, OBJD_SPOTLIGHT_ORIENTATION,
};
use crate::objects::circle::ObjectCircle;
use crate::objects::emitters::{Emitter, EmitterCollimated, EmitterSpotlight};
use crate::objects::ray::{init_collimated_rays, init_isotropic_rays, init_spotlight_rays};
use macroquad::input::mouse_position;

/// Creates and adds a new object to the scene at the current mouse position.
///
/// This function handles the creation of different types of objects based on the
/// provided object type string. It uses the current mouse cursor position as the
/// placement point for the new object.
///
/// # Arguments
///
/// * `object_type` - A string identifier for the type of object to create:
///   - "circle_none": Creates a simple circle object
///   - "emitter_isotropic": Creates an isotropic emitter (rays in all directions)
///   - "emitter_collimated": Creates a collimated emitter (parallel rays)
///
/// # Example
///
/// ```
/// // Create a new isotropic emitter at the current mouse position
/// add_object_to_scene("emitter_isotropic");
/// ```
pub fn add_object_to_scene(object_type: &str) {
    // Get the current mouse cursor position
    let (mouse_x, mouse_y) = mouse_position();

    if let "circle_none" = object_type {
        // Create a basic circle object at the mouse position
        ObjectCircle::new_and_add(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS);
    } else if let "emitter_isotropic" = object_type {
        // Create an isotropic emitter (radiating in all directions)
        Emitter::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_isotropic_rays(mouse_x, mouse_y),
        );
    } else if let "emitter_collimated" = object_type {
        // Create a collimated emitter (parallel rays, like a laser)
        EmitterCollimated::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_collimated_rays(
                mouse_x,
                mouse_y,
                OBJD_COLLIMATED_ORIENTATION,
                OBJD_COLLIMATED_BEAM_DIAMETER,
            ),
            OBJD_COLLIMATED_ORIENTATION,
            2.0 * OBJD_CIRCLE_RADIUS,
        );
    } else if let "emitter_spotlight" = object_type {
        // Create a spotlight emitter (like a flashlight)
        EmitterSpotlight::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_spotlight_rays(
                mouse_x,
                mouse_y,
                OBJD_SPOTLIGHT_ORIENTATION,
                OBJD_SPOTLIGHT_BEAM_ANGLE,
            ),
            OBJD_SPOTLIGHT_ORIENTATION,
            OBJD_SPOTLIGHT_BEAM_ANGLE,
        );
    }
}
