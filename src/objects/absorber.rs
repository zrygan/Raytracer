//! Absorber objects initialization and behaviors
//!
//! This module provides light absorber implementation for the raytracer system.
//! Absorbers are objects that can block or absorb light rays in the simulation.
//! Currently, the system supports perfect absorbers that completely block light.
//!
//! # Types of Absorbers
//!
//! * `AbsorberPerfect` - A perfect light absorber that completely blocks all light rays
//!
//! # Usage
//!
//! ```rust
//! use crate::objects::circle::ObjectCircle;
//! use crate::objects::absorber::{Absorbers, AbsorberPerfect};
//!
//! // Create a base circle object
//! let circle = ObjectCircle::new(100.0, 100.0, 50.0);
//!
//! // Create a perfect absorber using the circle
//! let perfect_absorber = AbsorberPerfect::new(circle);
//!
//! // Create an absorber enum variant
//! let absorber = Absorbers::AbsorberPerfect(perfect_absorber);
//! ```
//!
//! author:         Zhean Ganituen
//! last updated:   April 18, 2025

use super::behavior::*;
use super::circle::ObjectCircle;

/// Enum representing different types of light absorbing objects
///
/// This enum allows for polymorphic handling of different absorber types
/// through the system. All variants implement the `Drawable` and `Movable` traits.
#[derive(Clone, Debug)]
pub enum Absorbers {
    /// A perfect absorber that completely blocks light rays
    AbsorberPerfect(AbsorberPerfect),
}

impl Drawable for Absorbers {
    /// Draws the absorber object on screen
    ///
    /// Delegates to the underlying object's drawing implementation.
    fn draw_object(&self) {
        match self {
            Absorbers::AbsorberPerfect(obj) => obj.base_object.draw_object(),
        }
    }
}

impl Movable for Absorbers {
    /// Moves the absorber object to a new position
    ///
    /// # Parameters
    ///
    /// * `pos_x` - The new X coordinate
    /// * `pos_y` - The new Y coordinate
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Absorbers::AbsorberPerfect(obj) => obj.base_object.move_object(pos_x, pos_y),
        }
    }
}

impl VariableSize for Absorbers {
    /// Changes the radius of the absorber
    ///
    /// # Parameters
    ///
    /// * `factor` - The change size factor
    fn change_radius(&mut self, factor: f32) {
        match self {
            Absorbers::AbsorberPerfect(obj) => {
                let new_radius = obj.base_object.radius + factor;
                obj.base_object.radius = if new_radius > 0.0 { new_radius } else { 0.0 };
            }
        }
    }

    fn get_radius(&self) -> f32 {
        match self {
            Absorbers::AbsorberPerfect(obj) => obj.base_object.radius,
        }
    }
}

/// A perfect absorber that completely blocks all light
///
/// This absorber type will stop any ray that intersects with it.
/// It is represented visually as a black circle.
#[derive(Clone, Debug)]
pub struct AbsorberPerfect {
    /// The underlying circle object that defines the absorber's shape and position
    pub base_object: ObjectCircle,
}

impl AbsorberPerfect {
    /// Creates a new perfect absorber from a circle object
    ///
    /// # Parameters
    ///
    /// * `base_object` - The circle that defines the absorber's shape and position
    ///
    /// # Returns
    ///
    /// A new `AbsorberPerfect` instance
    pub fn new(base_object: ObjectCircle) -> AbsorberPerfect {
        AbsorberPerfect { base_object }
    }
}
