use crate::globals::{OBJD_CIRCLE_FILL, OBJD_CIRCLE_RADIUS};
use crate::objects::circle::ObjectCircle;
use macroquad::input::mouse_position;

pub fn add_object_to_scene(object_type: &str) {
    if let "circle_none" = object_type {
        ObjectCircle::new(
            mouse_position().0,
            mouse_position().1,
            OBJD_CIRCLE_FILL,
            OBJD_CIRCLE_RADIUS,
        );
    }
}
