use std::fs;

fn main() {
  let contents = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let mut valid_passwords = 0;
  let lines = contents.lines();

  for s in lines {
    let line = s.split(":").collect::<Vec<&str>>();
    let password = line[1].trim().chars().collect::<Vec<char>>();
    let rules = line[0].split_whitespace().collect::<Vec<&str>>();

    let rule = rules[0].split("-").collect::<Vec<&str>>();
    let character = rules[1].chars().collect::<Vec<char>>()[0];

    let first_char: char = password[rule[0].parse::<usize>().unwrap() - 1];
    let second_char: char = password[rule[1].parse::<usize>().unwrap() - 1];

    if (first_char == character) ^ (second_char == character) {
      valid_passwords += 1;
    }
  }
  println!("{}", valid_passwords);
}