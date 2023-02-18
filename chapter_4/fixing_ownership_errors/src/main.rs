fn main() {
    println!("Hello, world!");
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
        dst.iter().max_by_key(
            |s| s.chars().count()
        ).unwrap();
    for s in src {
        if s.chars().count() > largest.chars().count() {
            dst.push(s.clone());
        }
    }
}
