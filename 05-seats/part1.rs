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

  let mut highest_num = 0;
  for s in lines {
    let column: String = get_binary(s[7..10].to_string());
    let row: String = get_binary(s[0..7].to_string());

    let columnNum = isize::from_str_radix(&column, 2).unwrap();
    let rowNum = isize::from_str_radix(&row, 2).unwrap();
    if (columnNum + (rowNum * 8)) > highest_num {
      highest_num = columnNum + (rowNum * 8);
    }
  }

  println!("{}", highest_num);
}
