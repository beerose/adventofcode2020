use std::fs;

fn main() {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

  let mut valid_passports = 0;
  let lines = contents.split("\n\n").collect::<Vec<&str>>();

  for s in lines {
    let mut is_valid = true;

    if s.contains("byr")
      && s.contains("iyr")
      && s.contains("eyr")
      && s.contains("hgt")
      && s.contains("hcl")
      && s.contains("ecl")
      && s.contains("pid")
    {
      is_valid = false;
    }

    if !line.contains("byr") {
      is_valid = false;
    }

    if (is_valid) {
      valid_passports += 1;
    }
  }
  println!("{}", valid_passports);
}
