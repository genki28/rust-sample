fn main() {
  // let x: f32 = 100.234;
  // println!("x is {}", x);
  // let x: f64 = 100.234;
  // println!("x is {}", x);

  // let f = true;
  // println!("f is {}", f);

  // let dog = "DOG";
  // let cat = "CAT";
  // println!("{} and {}", dog, cat);

  // let s = String::from("Hello Rust world.");
  // println!("{}", s);

  // let s1 = String::from("Hello");
  // let s2 = String::from("Rust");
  // let s3 = String::from("world.");
  // let s = s1 + " " + &s2 + " " + &s3;// &は借用
  // println!("{}", s);

  // let s1 = String::from("Hello");
  // let s2 = String::from("Rust");
  // let s3 = String::from("world.");

  // let ns = format!("{} {} {}", s1, s2, s3);
  // println!("{}", ns);

  // let t = ("genki", 30);
  // println!("name is {} age {}", t.0, t.1);

  // let a = ["春", "夏", "秋", "冬"];
  // println!("最初の季節は{}", a[0]);
  // println!("最後の季節は{}", a[3]);
  let x = 100;
  let y = x;
  println!("x is {}", x);
  println!("y is {}", y);

  // moveしている
  // let x = String::from("Hello");
  // let y = x;
  // println!("x is {}", x);
  // println!("y is {}", y);
  let x = String::from("Hello");
  let len = string_length(&x);
  println!("len is {}", len);
  println!("x is {}", x);

  let mut x = 100;
  println!("x is {}", x);
  x = 200;
  println!("x is {}", x);
}

fn string_length(s: &String) -> usize {
  s.len()
}

fn add(x: i32, y: i32) -> i32 {
  println!("call add");
  x + y
}
