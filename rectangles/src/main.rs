#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);
    println!("rect1 is {rect1:?}");
    // println!("The area of rectangle is {} square pixels", area(&rect1));
    println!("The area of rectangle is {} square pixels", rect1.area());
}

// fn area(width: u32, height: u32) -> u32 {
//     return width * height;
// }

// fn area(dimensions:(u32, u32)) -> u32 {
//     return dimensions.0 * dimensions.1;
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     return rectangle.width * rectangle.height;
// }
