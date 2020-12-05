use std::fs;

fn get_binary(input: String) -> String {
  return input
    .chars()
    .map(|x| match x {
      'R' => '1',
      'L' => '0',
      'F' => '0',
      'B' => '1',
      _ => x,
    })
    .collect();
}

fn main() {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

  let lines = contents.split("\n").collect::<Vec<&str>>();

  let mut ids: Vec<usize> = vec![];
  for s in lines {
    let column: String = get_binary(s[7..10].to_string());
    let row: String = get_binary(s[0..7].to_string());

    let columnNum = usize::from_str_radix(&column, 2).unwrap();
    let rowNum = usize::from_str_radix(&row, 2).unwrap();
    let id = columnNum + (rowNum * 8);
    ids.push(id);
  }
  ids.sort();

  for i in 0..ids[ids.len() - 1] {
    if i == 0 || i >= ids.len() {
      continue;
    }
    if !ids.contains(&i) && ids.contains(&(i - 1)) && ids.contains(&(i + 1)) {
      println!("missing {}", i);
    }
  }
}
