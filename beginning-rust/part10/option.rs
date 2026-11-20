fn main (){
  let mut v = vec![1,2,3];
  for _ in 0..4 {
    let item: Option<i32> = v.pop();
    println!("{:?}", item);

    match item{
      Some(value) => println!("Popped value: {}", value),
      None => println!("Vector is empty"),
    }

  }

  let result = div(10.0, 0.0);
  match result {
    Ok(value) => println!("Result: {}", value),
    Err(error) => println!("Error: {}", error),
  }
}


fn div(num: f64, den: f64) -> Result<f64, String> {
    if den == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(num / den)
    }
}
