use smartpointer::tread::{self, thread_with_data_increment};
use std::sync::{mpsc, Mutex};
use std::thread;
use List::{Cons, Nil};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("Initial value of rc count is {}", Rc::strong_count(&a));
    println!(" a next item is {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));


    tread::thread_example();

    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });


    let received = rx.recv().unwrap();
    println!("Got: {}", received);


    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // MutexGuard is dropped here, releasing the lock
    println!("m = {:?}", m);
   thread_with_data_increment();
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
