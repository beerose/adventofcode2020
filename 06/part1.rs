use std::fs;
use std::collections::HashSet;


fn main() {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

  let lines = contents.split("\n\n").collect::<Vec<&str>>();

  let mut sum = 0;
  for s in lines {
    let mut questions = HashSet::new();
    for c in s.chars().filter(|c| !c.is_whitespace()) {
      questions.insert(c);
    }
    sum += questions.len();
  }

  println!("{}", sum);
}
