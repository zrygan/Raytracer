use crate::globals::OBJ_COLLECTION;

pub fn remove_object_at_index(index: usize) {
    let mut temp = OBJ_COLLECTION.lock().unwrap();
    if (index) < temp.len() {
        temp.remove(index);
    } else {
        eprintln!("Raytracer Err: Removing object at index is out of bounds.")
    }
}
