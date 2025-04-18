use crate::globals::{OBJ_COLLECTION, OBJC_MOUSE_EPSILON, OBJD_CIRCLE_RADIUS};

pub fn remove_object_at_index(index: usize) {
    let mut temp = OBJ_COLLECTION.lock().unwrap();
    if (index) < temp.len() {
        temp.remove(index);
    } else {
        eprintln!("Raytracer Err: Removing object at index is out of bounds.")
    }
}

pub fn object_at_cursor(mouse_x: f32, mouse_y: f32) -> Option<usize> {
    let temp = OBJ_COLLECTION.lock().unwrap();

    for (index, object) in temp.iter().enumerate() {
        let (x, y) = object.get_pos();

        if (mouse_x - x).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
            && (mouse_y - y).abs() < OBJC_MOUSE_EPSILON + OBJD_CIRCLE_RADIUS
        {
            return Some(index);
        }
    }

    None
}

pub fn print_all_objects() {
    for (index, obj) in OBJ_COLLECTION.lock().unwrap().iter().enumerate() {
        println!("RaytracerObject: {}", index);
        println!("{:#?}", obj);
    }
}
