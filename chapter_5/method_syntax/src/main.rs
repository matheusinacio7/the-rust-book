use std::io;
use std::io::Write;

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
    let sqr1: Rectangle = Rectangle::square(30);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold sqr1? {}", rect1.can_hold(&sqr1));
}

fn calculate_rectangle_area() {
    println!("Calculate rectangle area.\n");

    let mut width = String::new();
    print!("Enter width:  ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut width).expect("Failed to read stdin");
    let width: u32 = width.trim().parse().unwrap();


    let mut height = String::new();
    print!("Enter height:  ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut height).expect("Failed to read stdin");
    let height: u32 = height.trim().parse().unwrap();

    let rectangle = Rectangle {
        width: dbg!(width),
        height,
    };

    println!("The rectangle is {:?}", rectangle);

    println!(
        "\nThe area of the rectangle is: {}",
        rectangle.get_area(),
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
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
