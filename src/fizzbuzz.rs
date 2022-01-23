pub fn main() {
  for i in 1..101 {
    if i % 3 == 0 && i % 5 == 0 {
      println!("FizzBuzz")
    } else if i % 3 == 0 {
      println!("Fizz")
    } else if i % 5 == 0 {
      println!("Buzz")
    } else {
      println!("{}", i)
    }
  }
}

pub fn test() {
  for i in 1..51 {
    if i % 3 == 0 || i % 10 == 3 {
      println!("A");
      continue;
    }
    if (30..=39).contains(&i) {
      println!("A");
      continue;
    }
    println!("{}", i)
  }
}
