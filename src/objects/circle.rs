//! This module provides the structure and implementation for the circle
//! object

/// A circle object
struct Circle {
    /// The radius of the circle
    radius: i16, // i don't think this needs to be larger than 16 bits
    /// The position of the circle on the x-axis
    pos_x: i32,
    /// The position of the circle on the y-axis
    pos_y: i32,
    /// The color of the circle in RGB
    fill_color: [i32; 3],
    /// The boolean value that determines if the object is penetrable
    /// (light passes through it)
    penetrable: bool,
}

impl Circle {
    /// Creates a Circle object
    fn make(
        radius: i16,
        pos_x: i32,
        pos_y: i32,
        fill_color: Option<[i32; 3]>,
        penetrable: Option<bool>,
    ) -> Self {
        Circle {
            radius,
            pos_x,
            pos_y,
            fill_color,
            penetrable,
        }
    }
}

/// An emitter point object
struct Emitter_Point {
    /// Circle structure
    object: Cirlce,
    // TODO: rays
    /// The color of the rays emitted by this object
    emit_color: [int32; 3],
}

impl Emitter_Point {
    /// Creates an Emitter Point object
    fn make(
        radius: i16,
        pos_x: i32,
        pos_y: i32,
        fill_color: Option<[i32; 3]>,
        penetrable: Option<bool>,
        emit_color: Option<[i32; 3]>,
    ) -> Self {
        let fill_color_option: [i32; 3] = match fill_color {
            Some(value) => Some(value),
            None => Some([0, 0, 0]),
        };

        let penetrable_option: bool = match penetrable {
            Some(value) => Some(value),
            None => Some(true),
        };

        let emit_color_option: [i32; 3] = match emit_color {
            Some(value) => Some(value),
            None => Some([
                // TODO: change this to cornflower blue
                [0,0,0]
            ])
        };

        Emitter_Point {
            circle: Circle::make(radius, pos_x, pos_y, fill_color_option, penetrable_option),
            emit_color_option,
        };
    }

    /// Creates the rays of the object using the default number of rays
    fn init_rays(&self) {}

    /// Creates the rays of the object given a specific number of rays
    fn init_n_rays(&self, number_of_rays: int16) {}

    /// Moves the Emitter Point to a specific location
    /// Then, reinitializes the rays
    fn move_to(&self, x: int32, y: int32) {
        self.init_rays(&self);
    }
}

struct Emitter_Directional {
    object: Cirlce,
    emit_angle: f64,
    emit_spacing: f64,
    emit_color: [int32; 3],
}

impl Emitter_Directional {
    fn make(
        radius: i16,
        pos_x: i32,
        pos_y: i32,
        fill_color: Option<[i32; 3]>,
        penetrable: Option<bool>,
        emit_color: Option<[i32; 3]>,
        emit_angle: f64,
        emit_spacing: f64,
        emit_color: [int32; 3k],
    ) -> Self {
        Emitter_Directional {
            circle: Cirlce::make(radius, pos_x, pos_y, fill_color, penetrable),
        }
    }
}
