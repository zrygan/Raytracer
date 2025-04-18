use crate::globals::OBJ_COLLECTION;

use super::{absorber::Absorbers, behavior::RaytracerObjects, emitters::*, ray::ObjectRay};

pub fn occlusion(occluder: &Absorbers, ray: &ObjectRay) -> Option<(f32, f32)> {
    // get the slope of the ray
    let xs = ray.start_x;
    let xf = ray.end_x;
    let ys = ray.start_y;
    let yf = ray.end_y;
    let slope = (xf - xs, yf - ys);

    let (pos_x, pos_y, radius) = match occluder {
        Absorbers::AbsorberPerfect(o) => (
            o.base_object.pos_x,
            o.base_object.pos_y,
            o.base_object.radius,
        ),
    };

    // coefficients of the quadratic
    let a: f32 = slope.0.powi(2) + slope.1.powi(2);
    let b: f32 = 2.0 * (slope.0 * (xs - pos_x) + slope.1 * (ys - pos_y));
    let c: f32 = (xs - pos_x).powi(2) + (ys - pos_y).powi(2) - radius.powi(2); // Add the radius term

    // check if the quadratic has a solution
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        // if it has no solution, return None
        return None;
    }

    // if there is a solution, there must be two
    let sqrt_discriminant = discriminant.sqrt();
    let sol_1 = if a != 0.0 {
        (-b - sqrt_discriminant) / (2.0 * a)
    } else {
        0.0
    };

    let sol_2 = if a != 0.0 {
        (-b + sqrt_discriminant) / (2.0 * a)
    } else {
        0.0
    };

    // check both solutions choose the one that is after the start of the ray
    if (0.0 < sol_1) && (sol_1 <= 1.0) {
        return Some((xs + sol_1 * slope.0, ys + sol_1 * slope.1));
    } else if (0.0 < sol_2) && (sol_2 <= 1.0) {
        return Some((xs + sol_2 * slope.0, ys + sol_2 * slope.1));
    }

    return None;
}

pub fn check_for_occlusion() {
    let mut temp = OBJ_COLLECTION.lock().unwrap();

    // First, collect the absorber objects for reference
    let absorbers: Vec<_> = temp
        .iter()
        .filter_map(|a| {
            if let RaytracerObjects::Absorbers(o) = a {
                Some(o.clone()) // Clone to avoid borrowing issues
            } else {
                None
            }
        })
        .collect();

    // Process each object that contains rays
    for obj_index in 0..temp.len() {
        if let RaytracerObjects::Emitters(emitter) = &mut temp[obj_index] {
            // Get a mutable reference to the rays
            let rays = match emitter {
                Emitters::EmitterIsotropic(o) => &mut o.rays,
                Emitters::EmitterCollimated(o) => &mut o.base_emitter.rays,
                Emitters::EmitterSpotlight(o) => &mut o.base_emitter.rays,
            };

            // For each ray, check occlusion against all absorbers
            for ray in rays.iter_mut() {
                for absorber in &absorbers {
                    if let Some(hit_point) = occlusion(absorber, ray) {
                        let current_length = ((ray.end_x - ray.start_x).powi(2)
                            + (ray.end_y - ray.start_y).powi(2))
                        .sqrt();
                        let new_length = ((hit_point.0 - ray.start_x).powi(2)
                            + (hit_point.1 - ray.start_y).powi(2))
                        .sqrt();

                        if new_length < current_length {
                            ray.end_x = hit_point.0;
                            ray.end_y = hit_point.1;
                        }
                    }
                }
            }
        }
    }
}
