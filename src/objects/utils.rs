//! Utility functions for raytracer objects
//!
//! This module provides helper functions used throughout the raytracer for
//! calculations related to object positioning, ray transformations, and
//! coordinate system operations.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

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
