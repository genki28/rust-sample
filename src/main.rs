// struct Sample {
//   x: i32,
// }
// impl Sample {
//   fn new(x: i32) -> Sample {
//     Sample {x: x}
//   }

//   fn inc(&self) -> i32 {
//     self.x + 1
//   }
//   fn add(&self, x: i32) -> i32 {
//     self.x + x
//   }
// }
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
  // let x = 100;
  // let y = x;
  // println!("x is {}", x);
  // println!("y is {}", y);

  // moveしている
  // let x = String::from("Hello");
  // let y = x;
  // println!("x is {}", x);
  // println!("y is {}", y);
  // let x = String::from("Hello");
  // let len = string_length(&x);
  // println!("len is {}", len);
  // println!("x is {}", x);

  // let mut x = 100;
  // println!("x is {}", x);
  // x = 200;
  // println!("x is {}", x);

  // let x = 100;
  // println!("x is {}", x);
  // {
  //   let x = 200;
  //   println!("x is {}", x)
  // }
  // println!("x is {}", x);
  // let x = test(-1);
  // println!("x is {}", x);
  // let ans = add_two(10, 20);
  // println!("ans is {}", ans);
  // let ans = add_one(30);
  // println!("ans is {}", ans);
  // let a = Sample::new(10);
  // let ans = a.inc();
  // println!("ans is {}", ans);
  // let ans = a.add(20);
  // println!("ans is {}", ans);
  // let num = 10;
  // let add_one = |x: i32| {num + x};
  // let add_two = |x: i32, y: i32| {x + y};

  // let ans = add_one(1);
  // println!("ans is {}", ans);
  // let ans = add_two(10, 20);
  // println!("ans is {}", ans);
  // let ch = 'A';
  // println!("ch is {}", ch);
  // let ch = 'あ';
  // println!("ch is {}", ch);
  // let ch = 'A';
  // let u = ch as u8;
  // println!("u is {}", u);
  // let ch = u as char;
  // println!("ch is {}", ch);
  // let s = "hello rust world";
  // println!("s is {}", s);
  // let s = "hello rust world";
  // let hello = &s[0..5];
  // let world = &s[11..];
  // println!("hello is {}", hello);
  // println!("world is {}", world);
  // let len = s.len();
  // println!("s.len is {}", len);
  // let mut s = String::new();
  // s.push_str("hello ");
  // s.push_str("rust ");
  // s.push_str("world.");
  // println!("s is {}", s);
  // let hello = "HELLO";
  // let rust = "RUST";
  // let world = "WORLD.";
  // let s = format!("{} {} {}", hello, rust, world);
  // println!("s is {}", s);
  // // &String型の文字列
  // let s = "hello rust world.".to_string();
  // println!("s is {}", s);
  // let s = String::from("hello rust world.");
  // println!("s is {}", s);
  // let s = "こんにちは rust コードの世界";
  // println!("s is {}", s);
  // // 実行時エラー
  // // let hello = &s[0..5];
  // // let world = &s[11..];
  // // println!("こんにちは is {}", hello);
  // // println!("コードの世界 is {}", world);
  // // 以下は大丈夫。あくまで文字列のバイト位置を示している。
  // let hello = &s[0..15];
  // let world = &s[21..];
  // println!("こんにちは is {}", hello);
  // println!("コードの世界 is {}", world);
  // let len = s.len();
  // println!("s.len is {}", len);
  // let s = "This is ねこ🐱neko 文字列";
  // // ベクターへ
  // let mut v: Vec<char> = Vec::new();
  // for c in s.chars() {
  //   v.push(c);
  // }
  // // 上↑でcharに直してから8文字目から14文字目までを取得
  // let v = &v[8..15];
  // let mut s = String::new();
  // for c in v {
  //   s.push(*c);
  // }
  // println!("s is {}", s);
  // let s = "hello rust world.";
  // let a = &s[6..10];
  // println!("a is {}", a);
  // // 10ってわかりにくいなぁ...
  // let a = &s[6..(6+4)];
  // println!("a is {}", a);
  let a = 10;
  let b = 3;
  let ans = a / b;
  println!("a / b is {}", ans);
  let a = 10.0;
  let b = 3.0;
  let ans = a / b;
  println!("a / b is {}", ans);
  let a: u8 = 0b1111;
  let b: u8 = 0b0011;
  println!("a & b is {:04b}", a & b);
  println!("a | b is {:04b}", a | b);
  let a: u8 = 0x02;
  println!("a << 1 is {}", a << 1);
  println!("a >> 1 is {}", a >> 1);
  let v = vec![1, 2, 3, 4, 5];
  let sum = vec_param(&v);
  println!("sum is {}", sum);
  let v = vec_return(10);
  for i in v {
    print!("{} ", i);
  }
  println!();
  let mut v = vec![1, 2, 3, 4, 5];
  vec_change(&mut v);
  for i in v {
    print!("{} ", i);
  }
  println!();
}

fn vec_change(v: &mut Vec<i32>) {
  println!("called vec_change");
  for i in v {
    *i *= 10;
  }
}

fn vec_return(max: i32) -> Vec<i32> {
  println!("called vec_return");
  let mut v = Vec::new();
  for i in 0..max {
    v.push(i);
  }
  v
}

fn vec_param(v: &[i32]) -> i32 {
  println!("called vec_param");
  let mut sum = 0;
  for i in v {
    sum += i;
  }
  sum
}

// fn add_two(x: i32, y: i32) -> i32 {
//   x + y
// }

// fn add_one(x: i32) -> i32 {
//   x + 1
// }

// fn string_length(s: &String) -> usize {
//   s.len()
// }

// fn add(x: i32, y: i32) -> i32 {
//   println!("call add");
//   x + y
// }

// fn test(x: i32) -> i32 {
//   // let mut ans = x;
//   // if x < 0 {
//   //   ans = 0;
//   // }
//   // if x > 0 {
//   //   ans = 100;
//   // }
//   // ans
//   let ans = if x < 0 {
//     0
//   } else if x > 100 {
//     100
//   } else {
//     x
//   };
//   ans
// }
