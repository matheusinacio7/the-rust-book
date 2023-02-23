struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let abra = CustomSmartPointer {
        data: String::from("Abra"),
    };

    let kadabra = CustomSmartPointer {
        data: String::from("Kadabra"),
    };
    println!("CustomSmartPointers created");

    drop(kadabra);

    println!("End of main");
}
