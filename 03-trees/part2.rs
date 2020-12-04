use std::fs;

fn traverse(right: usize, down: usize, lines: &Vec<&str>) -> i32 {
  let mut trees = 0;
  let mut position_in_line = 0;
  let mut current_line = 0;

  while current_line < lines.len() {
    let line = lines[current_line].chars().collect::<Vec<char>>();
    if line[position_in_line] == '#' {
      trees += 1;
    }
    current_line+=down;
    position_in_line = (position_in_line + right) % line.len();
  }

  return trees;
}

fn main() {
  let contents = fs::read_to_string("./input.txt")
      .expect("Something went wrong reading the file");

  let lines = contents.lines().collect::<Vec<&str>>();
  let t1: i64 = traverse(3, 1, &lines).into();
  let t2: i64 = traverse(1, 1, &lines).into();
  let t3: i64 = traverse(5, 1, &lines).into();
  let t4: i64 = traverse(7, 1, &lines).into();
  let t5: i64 = traverse(1, 2, &lines).into();
  let trees: i128 = (t1 * t2 * t3 * t4 * t5).into();
  println!("{:?}", trees);
}