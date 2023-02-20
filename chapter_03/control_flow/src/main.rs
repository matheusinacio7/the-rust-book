fn main() {
    condition();
    my_loop();
    lift_off();
}

fn condition() {
    let condition = true;

    let x = if condition { 5 } else { 10 };
    println!("x is {x}");
}

fn my_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn lift_off() {
    for countdown in (1..4).rev() {
        println!("{countdown}...");
    }
    println!("LIFT OFF!");
}
