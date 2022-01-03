fn main() {
  let x: f32 = 100.234;
  println!("x is {}", x);
  let x: f64 = 100.234;
  println!("x is {}", x);

  let f = true;
  println!("f is {}", f);

  let dog = "DOG";
  let cat = "CAT";
  println!("{} and {}", dog, cat);

  let s = String::from("Hello Rust world.");
  println!("{}", s);

  let s1 = String::from("Hello");
  let s2 = String::from("Rust");
  let s3 = String::from("world.");
  let s = s1 + " " + &s2 + " " + &s3;// &は借用
  println!("{}", s);

  let s1 = String::from("Hello");
  let s2 = String::from("Rust");
  let s3 = String::from("world.");

  let ns = format!("{} {} {}", s1, s2, s3);
  println!("{}", ns);

  let t = ("genki", 30);
  println!("name is {} age {}", t.0, t.1);

  let a = ["春", "夏", "秋", "冬"];
  println!("最初の季節は{}", a[0]);
  println!("最後の季節は{}", a[3]);
}

fn add(x: i32, y: i32) -> i32 {
  println!("call add");
  x + y
}
