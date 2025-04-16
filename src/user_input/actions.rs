use crate::globals::{OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS};
use crate::objects::circle::ObjectCircle;
use crate::objects::emitters::EmitterPoint;
use crate::objects::ray::init_rays_for_point;
use macroquad::input::mouse_position;

pub fn add_object_to_scene(object_type: &str) {
    if let "circle_none" = object_type {
        ObjectCircle::new(
            mouse_position().0,
            mouse_position().1,
            OBJD_CIRCLE_FILL,
            OBJD_CIRCLE_RADIUS,
        );
    } else if let "emitter_point" = object_type {
        EmitterPoint::new(
            ObjectCircle::new(
                mouse_position().0,
                mouse_position().1,
                OBJD_CIRCLE_FILL,
                OBJD_CIRCLE_RADIUS,
            ),
            init_rays_for_point(mouse_position().0, mouse_position().1),
        );
    }
}
