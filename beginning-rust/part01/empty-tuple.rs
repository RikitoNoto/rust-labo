fn main(){
  let a: () = ();
  let b = { 12; 87; 283 };
  let c = { 12; 87; 283; };
  let d = {};
  let e = if false {};
  let f = while false {};
  println!("{:?}, {}, {:?}, {:?}, {:?}, {:?}", a, b, c, d, e, f);
}
