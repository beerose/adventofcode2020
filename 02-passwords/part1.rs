use std::fs;

fn main() {
  println!("In file {}", "./input.txt");

  let contents = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let mut valid_passwords = 0;
  let lines = contents.lines();

  for s in lines {
    let line = s.split(":").collect::<Vec<&str>>();
    let password = line[1].trim();
    let rules = line[0].split_whitespace().collect::<Vec<&str>>();

    if let [rule_str, character] = &rules[..] {
      let rule = rule_str.split("-").collect::<Vec<&str>>();
      let min: usize = rule[0].parse().unwrap();
      let max: usize = rule[1].parse().unwrap();
      let matches = password.matches(character).count();
      if matches >= min && matches <= max {
        valid_passwords += 1;
      }
    }
  }
  println!("{}", valid_passwords);
}