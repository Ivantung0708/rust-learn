use std::collections::VecDeque;
fn main() {
    let mut v:VecDeque<i32> = VecDeque::new();
    v.push_back(100);
    v.push_front(5);
    let x = v.pop_back();
    let y = v.pop_back();
    println!("success");
    println!("{}",x.unwrap_or(0));
    println!("{}",y.unwrap_or(0));
}
