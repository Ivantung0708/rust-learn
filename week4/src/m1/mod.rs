mod submod;

use submod::subfunc;

pub fn func () {
    println!("this is a function");
    subfunc();
}