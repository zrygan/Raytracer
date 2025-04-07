
fn main() { 
    println!("{}", my_func(None))
}

fn my_func(x: Option<i32>) -> i64 {
    match x {
        Some(value) => (value as i64) * 2, 
        None => 99999 as i64,
    }
}
