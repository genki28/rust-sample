pub fn main() {
  let enc = encrypt("I LOVE YOU", 3);
  let dec = encrypt(&enc, -3);
  println!("{} => {}", enc, dec)
}


fn encrypt(txt: &str, shift: i16) -> String {
  let a = 'A' as i16;
  let is_az = |c| ('A'..='Z').contains(&c);
  let conv = |c: i16| (((c - a + shift + 26) % 26 + a) as u8) as char;
  let enc1 = |c| if is_az(c) { conv(c as i16) } else { c };
  //個人的には、多分returnって明示的に書いたほうが事故無くなりそうな気はするけど...
  txt.chars().map(enc1).collect()
}