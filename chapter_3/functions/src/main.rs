fn main() {
    another_function(42, 'h');
    statements();
    println!("Here is five: {}", five());
}

fn another_function(x: i32, unit: char) {
    println!("The value of x is {x}{unit}");
}

fn statements() {
    let y = {
        let mut x = 3;
        x += 1;
        x * 3
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
