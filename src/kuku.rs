pub fn main() {
  for y in 1..10 {
    let s = (1..10)
      .map(|x| format!("{:3}", x * y))
      .collect::<Vec<String>>()
      .join(",");

    println!("{}", s)
  }
}
pub fn year_map() {
  for y in 1926..2026 {
    if y >= 2019 {
      if y == 2019 {
        println!("令和元年");
      } else {
        println!("令和{}年", y - 2019 + 1)
      }
    } else if y >= 1989 {
      if y == 1989 {
        println!("平成元年")
      } else {
        println!("平成{}年", y - 1986 + 1)
      }
    } else if y >= 1926 {
      if y == 1926 {
        println!("昭和元年")
      } else {
        println!("昭和{}年", y - 1926 + 1)
      }
    }
  }
}
