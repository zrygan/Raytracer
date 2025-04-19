//! Utility functions for raytracer objects
//!
//! This module provides helper functions used throughout the raytracer for
//! calculations related to object positioning, ray transformations, and
//! coordinate system operations.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 18, 2025

use crate::RaytracerObjects;
use crate::globals::{OBJ_COLLECTION, OBJD_RAY_COUNT};
use crate::objects::emitters::Emitters;
use crate::objects::ray::{init_collimated_rays, init_isotropic_rays, init_spotlight_rays};

/// Gets a set of points form x1 to x2 that are linearly spaces. That is, for
/// every point xi from the set of points, the distance from xi to x(i+1) for
/// any i is equal.
///
/// # Arguments
///
/// * `x1` - the starting point
/// * `x2` - the end point
/// * `sample_size` - the number of samples, or the size of the resulting vector
///
/// # Returns
///
/// An vector of linearly spaced points from x1 to x2 (inclusive).
pub fn linspace(x1: f32, x2: f32, sample_size: i32) -> Option<Vec<f32>> {
    if sample_size <= 1 {
        return None;
    }

    if sample_size == 2 {
        return Some(vec![x1, x2]);
    }

    let diff = (x2 - x1) / ((sample_size - 1) as f32);
    let mut points: Vec<f32> = Vec::with_capacity(sample_size as usize);

    for point in 0..sample_size {
        points.push(x1 + point as f32 * diff);
    }

    Some(points)
}

/// Initializes or reinitializes all rays for all emitter objects in the scene
///
/// This function iterates through the global object collection and updates
/// the rays for each emitter based on its current position and properties.
/// It's called whenever an object is created, moved, or deleted to ensure
/// ray positions remain consistent with their emitters.
///
/// # Thread Safety
///
/// This function acquires a write lock on the `OBJ_COLLECTION` global,
/// ensuring thread-safe updates to the ray data.
///
/// # Ray Types
///
/// Each emitter type has its rays initialized differently:
/// - Isotropic emitters create rays radiating in all directions
/// - Collimated emitters create parallel rays
/// - Spotlight emitters create a cone of rays
///
/// # Performance Considerations
///
/// This operation can be computationally expensive when many emitters exist
/// in the scene, so it should only be called when necessary (after object
/// creation or movement).
pub fn init_all_rays() {
    let mut collection = OBJ_COLLECTION.write().unwrap();

    // Iterate through the objects directly
    for obj in collection.iter_mut() {
        if let RaytracerObjects::Emitters(emitter_enum) = obj {
            match emitter_enum {
                Emitters::EmitterIsotropic(e) => {
                    let ray_count = if e.rays.is_empty() {
                        OBJD_RAY_COUNT
                    } else {
                        e.rays.len() as i32
                    };

                    e.rays =
                        init_isotropic_rays(e.base_object.pos_x, e.base_object.pos_y, ray_count)
                }
                Emitters::EmitterCollimated(e) => {
                    let ray_count = if e.base_emitter.rays.is_empty() {
                        OBJD_RAY_COUNT
                    } else {
                        e.base_emitter.rays.len() as i32
                    };

                    e.base_emitter.rays = init_collimated_rays(
                        e.base_emitter.base_object.pos_x,
                        e.base_emitter.base_object.pos_y,
                        e.orientation,
                        e.collimated_beam_diameter,
                        ray_count,
                    )
                }
                Emitters::EmitterSpotlight(e) => {
                    let ray_count = if e.base_emitter.rays.is_empty() {
                        OBJD_RAY_COUNT
                    } else {
                        e.base_emitter.rays.len() as i32
                    };

                    e.base_emitter.rays = init_spotlight_rays(
                        e.base_emitter.base_object.pos_x,
                        e.base_emitter.base_object.pos_y,
                        e.orientation,
                        e.spotlight_beam_angle,
                        ray_count,
                    )
                }
            }
        }
    }
}

/// Adds a new object to the global object collection
///
/// This function safely adds a new object to the raytracer's shared object collection
/// by acquiring a write lock and appending the object to the collection vector.
///
/// # Arguments
///
/// * `new_object` - The raytracer object to add to the global collection
///
/// # Thread Safety
///
/// This function acquires a write lock on the `OBJ_COLLECTION` global,
/// ensuring thread-safe addition of new objects.
///
/// # Error Handling
///
/// If acquiring the write lock fails, an error message is printed to stderr
/// and the object is not added to the collection.
///
/// # Example
///
/// ```
/// use crate::objects::circle::ObjectCircle;
/// use crate::objects::behavior::RaytracerObjects;
/// use crate::helpers::object_utils::add_object_to_collection;
///
/// // Create a new circle
/// let circle = ObjectCircle::new(100.0, 100.0, 50.0);
///
/// // Add it to the global collection
/// add_object_to_collection(RaytracerObjects::ObjectCircle(circle));
/// ```
pub fn add_object_to_collection(new_object: RaytracerObjects) {
    match OBJ_COLLECTION.write() {
        Ok(mut collection) => {
            collection.push(new_object);
            println!("Raytracer Upd: Added new object to OBJ_COLLECTION.");
        }
        Err(e) => {
            eprintln!(
                "Raytracer Err: Failed to add a new object to OBJ_COLLECTION.\nFailed to get write lock: {:?}.",
                e
            );
        }
    }
}
