
fn  larger<'a> (x: &'a i32,y: &'a i32) -> &'a i32 {
    if x > y {
        x
    }
    else {
        y
    }
}

fn get_ref (x: &i32) -> &i32 {
    x
} 
fn main() {
    // let x = 10;
    //let z;
    // {
    //     let y = 5;
    //     z = larger(&x,&y);
    // }
    // println!("{}",z);
    println!("=============");
    //let u;
    // {
    //     let w = 100;
    //     u = get_ref(&w);
    // }
    // println!("{:?}",u);

}
