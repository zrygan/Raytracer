//! Utility functions for interacting with scene objects
//!
//! This module provides helper functions for common operations on the raytracer's
//! object collection, including object selection, removal, and debugging.
//!
//! The functions in this module primarily deal with:
//! - Finding objects at specific positions
//! - Removing objects from the scene
//! - Debugging the scene state
//!
//! author:         Zhean Ganituen
//! last updated:   April 18, 2025

use crate::{
    globals::{OBJ_COLLECTION, OBJC_MOUSE_EPSILON, OBJD_CIRCLE_RADIUS},
    objects::{
        absorber::Absorbers,
        behavior::{RaytracerObjects, VariableSize},
        emitters::*,
    },
};

/// Removes an object from the scene at the specified index
///
/// This function provides safe removal of objects from the global collection
/// by acquiring a write lock and validating the index before removal.
///
/// # Arguments
///
/// * `index` - The index of the object to remove from the global collection
///
/// # Thread Safety
///
/// This function acquires a write lock on the `OBJ_COLLECTION` global,
/// so it's safe to call from multiple threads.
///
/// # Examples
///
/// ```
/// // Remove the first object in the collection
/// remove_object_at_index(0);
///
/// // Remove an object found at the cursor position
/// if let Some(index) = object_at_cursor_index(mouse_x, mouse_y) {
///     remove_object_at_index(index);
/// }
/// ```
pub fn remove_object_at_index(index: usize) {
    let mut temp = OBJ_COLLECTION.write().unwrap();
    if (index) < temp.len() {
        temp.remove(index);
    } else {
        eprintln!("Raytracer Err: Removing object at index is out of bounds.")
    }
}

/// Finds the first object located at or near the specified cursor position
///
/// This function checks all objects in the scene to find one that contains the
/// given cursor position, using a proximity detection with the global epsilon value.
///
/// # Arguments
///
/// * `mouse_x` - The x-coordinate of the cursor
/// * `mouse_y` - The y-coordinate of the cursor
///
/// # Returns
///
/// * `Some(index)` - The index of the first object found at the cursor position
/// * `None` - If no object is found within the specified epsilon of the cursor
///
/// # Selection Logic
///
/// Objects are considered "at the cursor" if the distance between the cursor
/// and object's center is less than `OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS`,
/// which accounts for both the cursor's proximity tolerance and the object's size.
pub fn object_at_cursor_index(mouse_x: f32, mouse_y: f32) -> Option<usize> {
    let temp = OBJ_COLLECTION.read().unwrap();

    for (index, object) in temp.iter().enumerate() {
        let (x, y) = object.get_pos();

        if (mouse_x - x).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
            && (mouse_y - y).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
        {
            return Some(index);
        }
    }

    None
}

pub fn get_object_scope(object: &RaytracerObjects) -> ((f32, f32), Option<f32>) {
    let pos = object.get_pos();
    let rad = match object {
        RaytracerObjects::Absorbers(o) => Some(o.get_radius()),
        RaytracerObjects::ObjectCircle(o) => Some(o.get_radius()),
        RaytracerObjects::Emitters(o) => Some(o.get_radius()),
    };

    (pos, rad)
}

pub fn object_at_cursor_type(mouse_x: f32, mouse_y: f32, specify: bool) -> &'static str {
    let temp = OBJ_COLLECTION.read().unwrap();

    for object in temp.iter() {
        let (pos, rad) = get_object_scope(object);
        if let Some(r) = rad {
            if (mouse_x - pos.0).abs() < OBJC_MOUSE_EPSILON + r
                && (mouse_y - pos.1).abs() < OBJC_MOUSE_EPSILON + r
            {
                return match object {
                    RaytracerObjects::ObjectCircle(_) => "ObjectCircle",
                    RaytracerObjects::Absorbers(absorber) => {
                        if specify {
                            match absorber {
                                Absorbers::AbsorberPerfect(_) => "Perfect",
                            }
                        } else {
                            "Absorber"
                        }
                    }
                    RaytracerObjects::Emitters(emitter) => {
                        if specify {
                            match emitter {
                                Emitters::EmitterIsotropic(_) => "Isotropic",
                                Emitters::EmitterCollimated(_) => "Collimated",
                                Emitters::EmitterSpotlight(_) => "Spotlight",
                            }
                        } else {
                            "Emitter"
                        }
                    }
                };
            }
        }
    }

    "None"
}

/// Prints details of all objects in the scene to the console
///
/// This function is primarily a debugging tool that outputs a formatted
/// representation of every object in the global collection, including their
/// indices and full Debug representations.
///
/// # Usage
///
/// This function is intended for debugging only and may produce extensive output
/// for scenes with many objects.
///
/// # Example Output
///
/// ```text
/// RaytracerObject: 0
/// ObjectCircle(
///     ObjectCircle {
///         pos_x: 100.0,
///         pos_y: 150.0,
///         radius: 50.0,
///     },
/// )
/// RaytracerObject: 1
/// Emitters(
///     EmitterIsotropic(
///         EmitterIsotropic {
///             base_object: ObjectCircle { ... },
///             rays: [...],
///         },
///     ),
/// )
/// ```
pub fn print_all_objects() {
    for (index, obj) in OBJ_COLLECTION.read().unwrap().iter().enumerate() {
        println!("RaytracerObject: {}", index);
        println!("{:#?}", obj);
    }
}
