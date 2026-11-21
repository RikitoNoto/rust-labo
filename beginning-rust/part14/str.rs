
// コンパイル時にサイズがわからないのでエラー
// str型は動的な長さを持つので、コンパイル時にサイズが確定しない
// fn f(a: str) {
// }

fn main(){
  let mut a: String = "He".to_string();
  a.push('l');
  a.push_str("lo");
  println!("{}", a);
}
