fn double(a: &mut [i32; 10]){
  for n in 0..10 {
    (*a)[n] *= 2;
  }
}

fn main() {
  let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  double(&mut arr);
  println!("{:?}", arr);
}
