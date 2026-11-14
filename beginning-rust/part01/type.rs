fn main(){
  let i8_val: i8 = 1;
  let i16_val: i16 = 2;
  let i32_val: i32 = 3;
  let i64_val: i64 = 4;
  let i128_val: i128 = 5;
  let isize_val: isize = 6;
  
  let u8_val: u8 = 1;
  let u16_val: u16 = 2;
  let u32_val: u32 = 3;
  let u64_val: u64 = 4;
  let u128_val: u128 = 5;
  // 配列アクセスのインデックスはコンパイルターゲットのビット数でのアクセスが一番効率が良い
  // sizeはコンパイラが自動で、ビット数に合わせる
  // 配列のインデックスはusize型のみ利用可能
  let usize_val: usize = 6;

  let _a: i16 = -150;
  let _b: = -150 as i16;
  let _c = -150 + _b - _b;
  let _d = -150i16;
}
