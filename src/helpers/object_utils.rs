//! Utility functions for raytracer objects
//!
//! This module provides helper functions used throughout the raytracer for
//! calculations related to object positioning, ray transformations, and
//! coordinate system operations.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

use crate::OBJ_COLLECTION;
use crate::RaytracerObjects;
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

pub fn init_all_rays() {
    let mut collection = OBJ_COLLECTION.lock().unwrap();

    // Iterate through the objects directly
    for obj in collection.iter_mut() {
        if let RaytracerObjects::Emitters(emitter_enum) = obj {
            match emitter_enum {
                Emitters::EmitterIsotropic(e) => {
                    e.rays = init_isotropic_rays(e.base_object.pos_x, e.base_object.pos_y)
                }
                Emitters::EmitterCollimated(e) => {
                    e.base_emitter.rays = init_collimated_rays(
                        e.base_emitter.base_object.pos_x,
                        e.base_emitter.base_object.pos_y,
                        e.orientation,
                        e.collimated_beam_diameter,
                    )
                }
                Emitters::EmitterSpotlight(e) => {
                    e.base_emitter.rays = init_spotlight_rays(
                        e.base_emitter.base_object.pos_x,
                        e.base_emitter.base_object.pos_y,
                        e.orientation,
                        e.spotlight_beam_angle,
                    )
                }
            }
        }
    }
}
