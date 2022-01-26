pub fn main() {
  let mut a = 1;
  let mut b = 1;
  println!("{}", a);
  println!("{}", b);
  for _ in 0..30 {
    println!("{}", a + b);
    let tmp = a;
    a = b;
    b = tmp + b;
  }
}

pub fn question() {
  let price: f64 = 98000.0;
  let a = price * 0.8 + 1200.0;
  let b = price * 0.9;
  println!("A社{}円", a);
  println!("B社{}円", b);
}