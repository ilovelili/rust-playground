#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 50,
        width: 100,
    };
    println!("rectangle area is {}", rect.area());

    // use :: to call an associated function
    let square = Rectangle::square(100);
    println!("square area is {}", square.area());
}
