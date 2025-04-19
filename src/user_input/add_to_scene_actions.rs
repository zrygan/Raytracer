//! User input action handlers
//!
//! This module contains functions that respond to user interactions by creating
//! and manipulating objects in the raytracer scene.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 17, 2025

use crate::globals::{
    OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS, OBJD_COLLIMATED_BEAM_DIAMETER,
    OBJD_COLLIMATED_ORIENTATION, OBJD_RAY_COUNT, OBJD_SPOTLIGHT_BEAM_ANGLE,
    OBJD_SPOTLIGHT_ORIENTATION,
};
use crate::helpers::object_utils::add_object_to_collection;
use crate::objects::absorber::{AbsorberPerfect, Absorbers};
use crate::objects::behavior::RaytracerObjects;
use crate::objects::circle::ObjectCircle;
use crate::objects::emitters::{EmitterCollimated, EmitterIsotropic, EmitterSpotlight, Emitters};
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
        let new_object = ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS);

        add_object_to_collection(RaytracerObjects::ObjectCircle(new_object));
    } else if let "emitter_isotropic" = object_type {
        // Create an isotropic emitter (radiating in all directions)
        let new_object = EmitterIsotropic::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_isotropic_rays(mouse_x, mouse_y, OBJD_RAY_COUNT),
        );

        add_object_to_collection(RaytracerObjects::Emitters(Emitters::EmitterIsotropic(
            new_object,
        )));
    } else if let "emitter_collimated" = object_type {
        // Create a collimated emitter (parallel rays, like a laser)
        let new_object = EmitterCollimated::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_collimated_rays(
                mouse_x,
                mouse_y,
                OBJD_COLLIMATED_ORIENTATION,
                OBJD_COLLIMATED_BEAM_DIAMETER,
                OBJD_RAY_COUNT,
            ),
            OBJD_COLLIMATED_ORIENTATION,
            OBJD_COLLIMATED_BEAM_DIAMETER,
        );

        add_object_to_collection(RaytracerObjects::Emitters(Emitters::EmitterCollimated(
            new_object,
        )));
    } else if let "emitter_spotlight" = object_type {
        // Create a spotlight emitter (like a flashlight)
        let new_object = EmitterSpotlight::new(
            ObjectCircle::new(mouse_x, mouse_y, OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS),
            init_spotlight_rays(
                mouse_x,
                mouse_y,
                OBJD_SPOTLIGHT_ORIENTATION,
                OBJD_SPOTLIGHT_BEAM_ANGLE,
                OBJD_RAY_COUNT,
            ),
            OBJD_SPOTLIGHT_ORIENTATION,
            OBJD_SPOTLIGHT_BEAM_ANGLE,
        );

        add_object_to_collection(RaytracerObjects::Emitters(Emitters::EmitterSpotlight(
            new_object,
        )));
    } else if let "absorber_perfect" = object_type {
        // Create a perfect absorber (full opaque)
        let new_object = AbsorberPerfect::new(ObjectCircle::new(
            mouse_x,
            mouse_y,
            OBJD_CIRCLE_FILL,
            OBJD_CIRCLE_RADIUS,
        ));

        add_object_to_collection(RaytracerObjects::Absorbers(Absorbers::AbsorberPerfect(
            new_object,
        )));
    }
}
