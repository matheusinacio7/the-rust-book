fn main() {
    let mut x = String::from("Hello");
    let y: &String = &x;
    println!("{} and {}", x, y);
    x.push_str(" world");
}