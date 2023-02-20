use std::cmp::PartialOrd;

fn get_largest<T: PartialOrd>(collection: &[T]) -> &T {
    let mut largest: &T = &collection[0];
    for val in collection {
        if val > largest {
            largest = val;
        }
    }
    return largest;
}

fn main() {
    let largest_i32 = get_largest(&[1, 8, 0, 3, -5]);
    println!("Largest i32: {largest_i32}");
    
    let largest_char = get_largest(&['A', 'a', '$']);
    println!("Largest char: {largest_char}");
}
