use std::mem::size_of;
use std::mem::size_of_val;

fn main(){
    println!("size_of::<i32>() = {}", size_of::<i32>());
    println!("size_of_val(&42) = {}", size_of_val(&42));

    let x: i32 = -42;
    let y: u32 = 42;
    
    println!("size_of_val(&x) = {}", size_of_val(&x));
    println!("size_of_val(&y) = {}", size_of_val(&y));
    
    println!("{:p}", &x);
    println!("{:p}", &y);


    println!("size_of_val(&[0i16; 50]) = {}", size_of_val(&[0i16; 50]));
    println!("size_of::<[i16; 50]>() = {}", size_of::<[i16; 50]>());
}
