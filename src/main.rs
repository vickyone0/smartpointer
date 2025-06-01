

fn main() {
    let list = List::Cons(1, List::Cons(2, List::Nil));
    println!("{:?}", list);
}


#[derive(Debug)]
enum List {
    Cons(i32, List),
    Nil,
}