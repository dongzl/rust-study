/// https://zhuanlan.zhihu.com/p/351892745

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

fn main() {
    let s: &dyn Area = &Square(3f32);
    println!("{:?}", s.get_area());
    let rec: &dyn Area = &Rectangle(4f32, 2f32);
    println!("{:?}", rec.get_area());

    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for e in shapes {
        println!("{:?}", e.get_area());
    }

    let s: Box<dyn Area> = Box::new(Square(3f32));
    println!("{:?}", s.get_area());
    let rec: Box<dyn Area> = Box::new(Rectangle(4f32, 2f32));
    println!("{:?}", rec.get_area());
    let shapes: Vec<Box<dyn Area>> = vec![Box::new(Square(3f32)), Box::new(Rectangle(4f32, 2f32))];
    for e in shapes {
        println!("{:?}", e.get_area());
    }

    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for e in shapes {
        println!("{:?}", get_area(e));
    }
}

fn get_area(item: &dyn Area) -> f32 {
    item.get_area()
}
