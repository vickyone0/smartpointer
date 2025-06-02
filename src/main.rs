


fn main() {
    
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);
 
}

struct MyBox <T> (T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}