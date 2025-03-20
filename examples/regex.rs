use regex::Regex;

fn main() {
  let mut input = "fOobar";
  let mut re = Regex::new(r"((?i)o{2})").unwrap();
  
  println!("{}", re.is_match(input));
  println!("{}", re.replace_all(input, "*$1*"));

  input = "2025-03-20";
  re = Regex::new(r"((?mi)[0-9]+)").unwrap();
  
  println!("{}", re.is_match(input));
}
