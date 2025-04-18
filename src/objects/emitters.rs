//! EmitterIsotropic objects initialization and behaviors
//!
//! This module provides light emitter implementations for the raytracer system.
//! It defines three types of emitters: isotropic (radiating in all directions),
//! collimated (parallel rays, like a laser), and spotlight (cone-shaped beam).
//!
//! author:         Zhean Ganituen
//! last updated:   April 17, 2025

use macroquad::shapes::draw_circle;

use super::behavior::{Drawable, Movable, RaytracerObjects};
use super::circle::ObjectCircle;
use super::ray::{ObjectRay, init_collimated_rays, init_isotropic_rays, init_spotlight_rays};
use crate::OBJ_COLLECTION;

/// Enumeration of all emitter types supported by the raytracer.
///
/// This enum allows different emitter types to be treated polymorphically
/// in the rendering and physics systems.
#[derive(Clone, Debug)]
pub enum Emitters {
    /// Standard isotropic emitter that radiates light in all directions
    EmitterIsotropic(EmitterIsotropic),
    /// Directional emitter that produces parallel rays
    EmitterCollimated(EmitterCollimated),
    /// Spotlight emitter that produces a cone-shaped beam
    EmitterSpotlight(EmitterSpotlight),
}

impl Drawable for Emitters {
    /// Draws the emitter on the screen.
    ///
    /// Delegates to the appropriate draw_object method based on the emitter type.
    fn draw_object(&self) {
        match self {
            Emitters::EmitterIsotropic(e) => e.draw_object(),
            Emitters::EmitterCollimated(e) => e.base_emitter.draw_object(),
            Emitters::EmitterSpotlight(e) => e.base_emitter.draw_object(),
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
    fn move_object(&mut self, pos_x: f32, pos_y: f32) {
        match self {
            Emitters::EmitterIsotropic(obj) => {
                obj.base_object.pos_x = pos_x;
                obj.base_object.pos_y = pos_y;
                obj.rays = init_isotropic_rays(pos_x, pos_y);
            }
            Emitters::EmitterCollimated(obj) => {
                obj.base_emitter.base_object.pos_x = pos_x;
                obj.base_emitter.base_object.pos_y = pos_y;
                obj.base_emitter.rays = init_collimated_rays(
                    pos_x,
                    pos_y,
                    obj.orientation,
                    obj.collimated_beam_diameter,
                );
            }
            Emitters::EmitterSpotlight(obj) => {
                obj.base_emitter.base_object.pos_x = pos_x;
                obj.base_emitter.base_object.pos_y = pos_y;
                obj.base_emitter.rays =
                    init_spotlight_rays(pos_x, pos_y, obj.orientation, obj.spotlight_beam_angle);
            }
        }
    }
}

/// Represents a standard isotropic light emitter.
///
/// This emitter radiates light rays in all directions from a central point,
/// similar to a point light source in real-world physics.
#[derive(Clone, Debug)]
pub struct EmitterIsotropic {
    /// The physical representation of the emitter (position, size, color)
    pub base_object: ObjectCircle,
    /// Collection of light rays emanating from this emitter
    pub rays: Vec<ObjectRay>,
}

impl EmitterIsotropic {
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
    /// A new `EmitterIsotropic` instance with the specified parameters
    pub fn new(base_object: ObjectCircle, rays: Vec<ObjectRay>) -> Self {
        let new_emitter = EmitterIsotropic { base_object, rays };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Emitters(Emitters::EmitterIsotropic(
                new_emitter.clone(),
            )));

        new_emitter
    }
}

impl Drawable for EmitterIsotropic {
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
        for ray in &self.rays {
            ray.draw_object();
        }
    }
}

/// Represents a collimated (parallel rays) light emitter.
///
/// This emitter produces rays that travel in parallel, similar to a laser beam
/// in real-world physics. It has an orientation and beam diameter.
#[derive(Clone, Debug)]
pub struct EmitterCollimated {
    /// The underlying emitter providing basic functionality
    pub base_emitter: EmitterIsotropic,
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
    ) -> Self {
        let new_emitter = EmitterCollimated {
            base_emitter: EmitterIsotropic { base_object, rays },
            orientation,
            collimated_beam_diameter,
        };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Emitters(Emitters::EmitterCollimated(
                new_emitter.clone(),
            )));

        new_emitter
    }
}

/// Represents a spotlight emitter.
///
/// This emitter produces rays within a specific angular range, defined by the
/// `spotlight_beam_angle`. The rays are distributed within this angle, centered
/// around the `orientation` angle, creating a cone-like light effect.
#[derive(Clone, Debug)]
pub struct EmitterSpotlight {
    /// The underlying emitter providing basic functionality
    pub base_emitter: EmitterIsotropic,
    /// The central angle (in radians) at which the spotlight is directed
    pub orientation: f32,
    /// The angular range (in radians) within which rays are emitted
    pub spotlight_beam_angle: f32,
}

impl EmitterSpotlight {
    /// Creates a new spotlight emitter with the specified properties.
    ///
    /// Adds the newly created emitter to the global object collection
    /// for tracking in the raytracer system.
    ///
    /// # Arguments
    ///
    /// * `base_object` - The physical properties of the emitter
    /// * `rays` - Collection of rays to be emitted from this source
    /// * `orientation` - The central angle (in radians) at which the spotlight is directed
    /// * `spotlight_beam_angle` - The angular range (in radians) within which rays are emitted,
    ///   forming a cone of light. A smaller angle creates a narrower beam, while a larger angle
    ///   creates a wider beam.
    ///
    /// # Returns
    ///
    /// A new `EmitterSpotlight` instance with the specified parameters
    pub fn new(
        base_object: ObjectCircle,
        rays: Vec<ObjectRay>,
        orientation: f32,
        spotlight_beam_angle: f32,
    ) -> Self {
        let new_emitter = EmitterSpotlight {
            base_emitter: EmitterIsotropic { base_object, rays },
            orientation,
            spotlight_beam_angle,
        };

        OBJ_COLLECTION
            .lock()
            .unwrap()
            .push(RaytracerObjects::Emitters(Emitters::EmitterSpotlight(
                new_emitter.clone(),
            )));

        new_emitter
    }
}
