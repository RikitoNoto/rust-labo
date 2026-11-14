fn main(){
  enum Direction {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
  }
  
  let dir = Direction::Up(10);
  
  println!("{:?}", match dir {
    Direction::Up(n) => format!("Up: {n}"),
    Direction::Down(n) => format!("Down: {n}"),
    Direction::Left(n) => format!("Left: {n}"),
    _ => "Unknown".to_string(),
  });

  for n in -2..5 {
    println!("{} is {}.", n, match n{
      0 => "zero",
      1 => "one",
      _ if n < 0 => "negative",
      _ => "plural",
    });
  }

  if let Direction::Up(n) = dir {
    println!("Selection Up: {n}");
  }
}
