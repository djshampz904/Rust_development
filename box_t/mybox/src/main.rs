use mybox::List::{Cons, Nil};


fn main() {
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let new_box = Box::new(5);

    println!("b = {new_box}");
    
    println!("list = {:?}", list);

}
