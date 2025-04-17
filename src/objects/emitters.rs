//! Emitter objects initialization and behaviors
//!
//! This module provides light emitter implementations for the raytracer system.
//! It defines two types of emitters: isotropic (radiating in all directions)
//! and collimated (parallel rays, like a laser).

use macroquad::shapes::{draw_circle, draw_line};

use super::behavior::{Drawable, Movable, RaytracerObjects};
use super::circle::ObjectCircle;
use super::ray::ObjectRay;
use crate::OBJ_COLLECTION;

/// Enumeration of all emitter types supported by the raytracer.
///
/// This enum allows different emitter types to be treated polymorphically
/// in the rendering and physics systems.
pub enum Emitters {
    /// Standard isotropic emitter that radiates light in all directions
    Emitter(Emitter),
    /// Directional emitter that produces parallel light rays
    EmitterCollimated(EmitterCollimated),
}

impl Drawable for Emitters {
    /// Draws the emitter on the screen.
    ///
    /// Delegates to the appropriate draw_object method based on the emitter type.
    fn draw_object(&self) {
        match self {
            Emitters::Emitter(e) => e.draw_object(),
            Emitters::EmitterCollimated(e) => e.base_emitter.draw_object(),
        }
    }
}

impl Movable for Emitters {
    /// Moves the emitter to a new position.
    ///
    /// Delegates to the appropriate move_object method based on the emitter type.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    // #[warn(unused_variables)]
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Emitters::Emitter(e) => e.move_object(pos_x, pos_y),
            Emitters::EmitterCollimated(e) => e.base_emitter.move_object(pos_x, pos_y),
        }
    }
}

impl Drawable for Emitter {
    /// Draws the isotropic emitter and its rays on the screen.
    ///
    /// Renders the emitter as a colored circle and draws all of its
    /// associated light rays emanating from it.
    fn draw_object(&self) {
        // Draw the emitter's physical representation (a circle)
        draw_circle(
            self.base_object.pos_x,
            self.base_object.pos_y,
            self.base_object.radius,
            self.base_object.color_fill,
        );

        // Draw all the light rays associated with this emitter
        for ray in self.rays.clone() {
            draw_line(
                ray.start_x,
                ray.start_y,
                ray.end_x,
                ray.end_y,
                ray.thickness,
                ray.color,
            );
        }
    }
}

impl Movable for Emitter {
    /// Moves the emitter to a new position.
    ///
    /// Currently only logs the new position; implementation for actually
    /// moving the emitter and updating its rays would need to be added.
    ///
    /// # Arguments
    ///
    /// * `pos_x` - The new x-coordinate position
    /// * `pos_y` - The new y-coordinate position
    // #[warn(unused_variables)]
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        println!("{}, {}", pos_x, pos_y)
    }
}

/// Represents a standard isotropic light emitter.
///
/// This emitter radiates light rays in all directions from a central point,
/// similar to a point light source in real-world physics.
#[derive(Clone)]
pub struct Emitter {
    /// The physical representation of the emitter (position, size, color)
    pub base_object: ObjectCircle,
    /// Collection of light rays emanating from this emitter
    pub rays: Vec<ObjectRay>,
}

impl Emitter {
    /// Creates a new isotropic emitter with the given properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_object` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    ///
    /// # Returns
    ///
    /// A new `Emitter` instance with the specified parameters
    pub fn new(base_object: ObjectCircle, rays: Vec<ObjectRay>) -> Emitter {
        // if rays.len() > OBJC_MAX_RAY_COUNT as usize {
        //     panic!("Raytracer: Emitter has too many rays. See OBJC_MAX_RAY_COUNT in globals.")
        // }

        let new_emitter: Emitter = Emitter { base_object, rays };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Emitter(new_emitter.clone()));

        new_emitter
    }
}

/// Represents a collimated (parallel rays) light emitter.
///
/// This emitter produces rays that travel in parallel, similar to a laser beam
/// in real-world physics. It has an orientation and beam diameter.
#[derive(Clone)]
pub struct EmitterCollimated {
    /// The underlying emitter providing basic functionality
    pub base_emitter: Emitter,
    /// The angle (in radians) at which rays are emitted
    pub orientation: f32,
    /// The width of the beam of parallel rays
    pub collimated_beam_diameter: f32,
}

impl EmitterCollimated {
    /// Creates a new collimated emitter with the specified properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_object` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    /// * `orientation` - The angle (in radians) at which rays are emitted
    /// * `collimated_beam_diameter` - The width of the beam of parallel rays
    ///
    /// # Returns
    ///
    /// A new `EmitterCollimated` instance with the specified parameters
    pub fn new(
        base_object: ObjectCircle,
        rays: Vec<ObjectRay>,
        orientation: f32,
        collimated_beam_diameter: f32,
    ) -> EmitterCollimated {
        let new_emitter = EmitterCollimated {
            base_emitter: Emitter { base_object, rays },
            orientation,
            collimated_beam_diameter,
        };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::EmitterCollimated(new_emitter.clone()));

        new_emitter
    }
}
