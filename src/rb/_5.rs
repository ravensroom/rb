pub fn run_rectangle() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{:#?}", rect1);

    println!(
        "The area of the rectangle is {}/{} square pixels.",
        rect1.area(),
        area(&rect1)
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(5);
    dbg!(&square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions without &self as parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
