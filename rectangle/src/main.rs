#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        ( self.width >= other.width && self.height >= other.height ) || ( self.width >= other.height && self.height >= other.width )
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square thingies.", rect1.area());
    let rect2 = Rectangle {width: 25, height: 40};
    let rect3 = Rectangle {width: 40, height: 20};
    let rect4 = Rectangle {width: 50, height: 60};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}
