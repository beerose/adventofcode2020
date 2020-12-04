use std::collections::HashMap;
use std::fs;

fn parse_height(value: &String) -> bool {
  let h = value.chars().filter(|c| c.is_digit(10)).collect::<String>();
  match h.parse::<i32>() {
    Ok(num) => {
      if value.contains("cm") {
        return num >= 150 && num <= 193;
      }
      if value.contains("in") {
        return num >= 59 && num <= 76;
      }
      return false;
    }
    _ => false,
  }
}

fn parse_color(value: &String) -> bool {
  if !(value.starts_with("#")) {
    return false;
  }
  if !(value.chars().count() == 7) {
    return false;
  }
  // it's a bit lousy
  return value.chars().all(|c| c == '#' || char::is_alphanumeric(c));
}

fn validate_entry(pair: (&String, &String)) -> bool {
  let (key, value) = pair;
  return match key.as_ref() {
    "byr" => match value.parse::<i32>() {
      Ok(num) => num >= 1920 && num <= 2002,
      _ => false,
    },
    "iyr" => match value.parse::<i32>() {
      Ok(num) => num >= 2010 && num <= 2020,
      _ => false,
    },
    "eyr" => match value.parse::<i32>() {
      Ok(num) => num >= 2020 && num <= 2030,
      _ => false,
    },
    "hgt" => parse_height(value),
    "pid" => value.chars().count() == 9,
    "hcl" => parse_color(value),
    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
      .iter()
      .any(|x| x == value),
    _ => true,
  };
}

fn parse_data(lines: Vec<&str>) -> Vec<HashMap<String, String>> {
  let mut res: Vec<HashMap<String, String>> = vec![];

  for s in lines {
    let values = s.split(|x| (x == ' ') || (x == '\n')).collect::<Vec<_>>();

    let mut entry = HashMap::new();
    for v in values {
      let info = v.split(":").collect::<Vec<_>>();
      entry.insert(info[0].to_string(), info[1].to_string());
    }
    res.push(entry);
  }

  return res;
}

fn main() {
  let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");

  let lines = contents.split("\n\n").collect::<Vec<_>>();
  let entries = parse_data(lines);
  let mut valid_passports = 0;

  let all_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

  for e in entries {
    let keys = e.keys().collect::<Vec<_>>();
    if !(all_keys.iter().all(|key| keys.iter().any(|v| v == key))) {
      continue;
    }
    if !(e.iter().all(|p| validate_entry(p))) {
      continue;
    }
    valid_passports += 1;
  }
  println!("{}", valid_passports);
}
