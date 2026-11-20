fn f(p: &f64){
  let a = Box::new(*p);
  {
    let b = Box::new([1, 2, 3]);
    println!("a = {}, b = {:?}", *a, *b);
    // スコープから抜けると自動で解放される
  }
  let c = Box::new(true);
  println!("{} {}", *a, c); // *はなくてもOK
}

fn main(){
  f(&3.14);
}
