use std::ops::Deref;

struct Node {
    data: u32,
}

struct Mybox<T>(T);

impl<T> Deref for Mybox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Mybox<T> {
    fn new(value: T) -> Mybox<T> {
        Mybox(value)
    }
}
impl Drop for Node {
    fn drop(&mut self){
        println!("node with data {} is dropped here!",&self.data);
    }
}

fn main() {
    let a = Node {
        data: 5,
    };
    let y = Mybox::new(&a);
    println!("{}",y.data);
    //let b = &a;
    //println!("{}",b.data);
    //drop(a);
    let z = Box::new(a);
    println!("hello {}",z.data);
    //println!("{}",a.data);
}