use std::fs;

fn main() {
  let contents = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let mut trees = 0;
  let lines = contents.lines().collect::<Vec<&str>>();
  let mut position_in_line = 0;
  let mut current_line = 0;

  while current_line < lines.len() {
    let line = lines[current_line].chars().collect::<Vec<char>>();
    if line[position_in_line] == '#' {
      trees += 1;
    }
    current_line+=1;
    position_in_line = (position_in_line + 3) % line.len();
  }

  println!("{:?}", trees);
}