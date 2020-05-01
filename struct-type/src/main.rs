fn main() {
    let rect = Rectangle {
        width: 30,
        height: 15,
    };

    println!("{}", rect.area());
    println!(
        "rect1 can hold rect2? {}",
        rect.can_hold(&Rectangle {
            width: 20,
            height: 10
        })
    );

    println!("{}", Rectangle::square(10));
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> u32 {
        size * size
    }
}
