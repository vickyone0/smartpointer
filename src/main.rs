struct  CustomSmartPointer {
    value: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with value: {}", self.value);
    }
}


fn main() {

    let c = CustomSmartPointer {
        value: String::from("Hello, Rust!"),
    };

    let d = CustomSmartPointer{
        value: String::from("Goodbye, Rust!"),
    };

    println!("CustomSmartPointer created with value: {}", c.value);
}