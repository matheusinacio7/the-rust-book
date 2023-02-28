fn add_one(x: i32) -> i32 {
    x + 1
}

fn calc_twice(cb: fn(i32) -> i32, arg: i32) -> i32 {
    cb(cb(arg))
}

fn main() {
    let answer = calc_twice(add_one, 5);

    println!("The answer is {}", answer);
}
