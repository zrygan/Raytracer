use crate::{
    globals::OBJ_COLLECTION,
    helpers::action_utils::object_at_cursor_index,
    objects::behavior::{RaytracerObjects, VariableOrientation, VariableSize},
};

pub fn object_change_size(mouse_x: f32, mouse_y: f32, change_factor: f32) {
    if let Some(object_index) = object_at_cursor_index(mouse_x, mouse_y) {
        let mut collection = OBJ_COLLECTION.write().unwrap();
        if let Some(object) = collection.get_mut(object_index) {
            match object {
                RaytracerObjects::ObjectCircle(o) => o.change_radius(change_factor),
                RaytracerObjects::Absorbers(o) => o.change_radius(change_factor),
                RaytracerObjects::Emitters(o) => o.change_radius(change_factor),
            }
        }
    }
}

pub fn object_change_orientation(mouse_x: f32, mouse_y: f32, change_factor: f32) {
    if let Some(object_index) = object_at_cursor_index(mouse_x, mouse_y) {
        let mut collection = OBJ_COLLECTION.write().unwrap();
        if let Some(object) = collection.get_mut(object_index) {
            match object {
                RaytracerObjects::Emitters(o) => o.change_orientation(change_factor),
                _ => {}
            }
        }
    }
}
