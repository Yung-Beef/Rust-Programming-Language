#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of rectangle 1 is {} square pixels.",
        rect1.area()
    );

    if rect1.width() { // by following rect1.width with (), rust knows it's a method, not the field(/attribute?)
        println!("Rectangle 1 has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
//     println!("rect1 is {rect1:#?}"); // :? tells println to output with the debug format, but requires adding debug to the struct
//     // :#? just makes it a prettier version of debug output

//     // dbg! takes ownership of an expression's value
//     // here it takes ownership of 30 * scale, and thus sets width = 60, as if dbg was never there
//     let scale = 2;
//     let rect2 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50
//     };

//     dbg!(&rect2);
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
