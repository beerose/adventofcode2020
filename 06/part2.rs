use std::fs;
use std::collections::HashSet;


fn main() {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

  let lines = contents.split("\n\n").collect::<Vec<&str>>();

  let mut sum = 0;
  for s in lines {
    let mut all_chars = HashSet::new();
    for c in s.chars().filter(|c| !c.is_whitespace()) {
      all_chars.insert(c);
    }
    let answers = s.split("\n").collect::<Vec<&str>>();
    for q in all_chars.into_iter() {
      if answers.iter().all(|a| a.contains(q)) {
        sum += 1;
      }
    }
  }

  println!("{}", sum);
}
