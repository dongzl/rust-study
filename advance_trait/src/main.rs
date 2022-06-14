use std::fmt::Debug;

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

fn list_trait() -> Option<Box<dyn Area>> {
    Some(Box::new(Square(3f32)))
}

fn main() {
    let s: &dyn Area = &Square(3f32);
    println!("{:?}", s.get_area());
    let rec: &dyn Area = &Rectangle(4f32, 2f32);
    println!("{:?}", rec.get_area());

    let trait_test = list_trait().unwrap();
    println!("{:?}", trait_test.get_area());
}
