enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use List::{Cons, Nil};
use std::rc::Rc;

fn main(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(List::Nil)))));

    println!("a has {} strong references", Rc::strong_count(&a));

    let b = Rc::new(List::Cons(3,Rc::clone(&a)));

    println!("a has {} strong references", Rc::strong_count(&a));
    {
    let c = Rc::new(List::Cons(4, Rc::clone(&a)));
    println!("a has {} strong references", Rc::strong_count(&a));

    }
    println!("a has {} strong references after c out of scope", Rc::strong_count(&a));

}