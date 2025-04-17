use crate::globals::{OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS, OBJD_RAY_COLOR, OBJD_RAY_WIDTH};
use crate::objects::circle::ObjectCircle;
use crate::objects::emitters::{Emitter, EmitterCollimated};
use crate::objects::ray::{init_collimated_rays, init_isotropic_rays};
use macroquad::input::mouse_position;

pub fn add_object_to_scene(object_type: &str) {
    if let "circle_none" = object_type {
        ObjectCircle::new(
            mouse_position().0,
            mouse_position().1,
            OBJD_CIRCLE_FILL,
            OBJD_CIRCLE_RADIUS,
        );
    } else if let "emitter_isotropic" = object_type {
        Emitter::new(
            ObjectCircle::new(
                mouse_position().0,
                mouse_position().1,
                OBJD_CIRCLE_FILL,
                OBJD_CIRCLE_RADIUS,
            ),
            init_isotropic_rays(mouse_position().0, mouse_position().1),
        );
    } else if let "emitter_collimated" = object_type {
        EmitterCollimated::new(
            ObjectCircle::new(
                mouse_position().0,
                mouse_position().1,
                OBJD_CIRCLE_FILL,
                OBJD_CIRCLE_RADIUS,
            ),
            init_collimated_rays(
                mouse_position().0,
                mouse_position().1,
                30.0,
                50.0 * OBJD_RAY_WIDTH,
            ),
            30.0,
            50.0 * OBJD_CIRCLE_RADIUS,
        );
    }
}
