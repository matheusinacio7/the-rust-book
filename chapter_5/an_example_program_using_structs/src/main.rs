use std::io;
use std::io::Write;

fn main() {
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
        calculate_rectangle_area(&rectangle)
    );

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calculate_rectangle_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
