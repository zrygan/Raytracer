//! Traits and common behaviors for raytracer objects
//!
//! This module defines the core traits that all renderable objects in the raytracer
//! must implement, as well as an enum for polymorphic handling of different object types.
//! It serves as the foundation for the object behavior system in the raytracer.
//!
//! author:         Zhean Ganituen (zrygan)
//! last updated:   April 16, 2025

use super::circle::ObjectCircle;
use super::emitters::{Emitter, EmitterCollimated, EmitterSpotlight, Emitters};

/// Enum that represents all possible object types in the raytracer.
///
/// This enum allows for polymorphic handling of different object types
/// in collections and the rendering system. Each variant wraps a concrete
/// object implementation.
pub enum RaytracerObjects {
    /// A circular shape object (used for barriers, mirrors, etc.)
    ObjectCircle(ObjectCircle),
    /// A standard isotropic light emitter
    Emitter(Emitter),
    /// A specialized light emitter that produces parallel rays
    EmitterCollimated(EmitterCollimated),
    /// A specialized light emitter that produces a spot light
    EmitterSpotlight(EmitterSpotlight),
}

/// Trait for objects that can be rendered to the screen.
///
/// All visible objects in the raytracer must implement this trait,
/// which provides the necessary method to render the object.
pub trait Drawable {
    /// Renders the object to the screen.
    ///
    /// Each implementing type will define its own rendering behavior
    /// appropriate to its visual representation.
    fn draw_object(&self);
}

/// Trait for objects that can be moved to a new position.
///
/// Provides functionality for interactive placement and animation
/// of objects within the raytracer environment.
pub trait Movable {
    /// Moves the object to a new position.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    fn move_object(&mut self, pos_x: f32, pos_y: f32);
}
