extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn is_nice_v1(word: &str) -> bool {
  let forbidden = ["ab", "cd", "pq", "xy"];
  let vowels = ["a", "e", "i", "o", "u"];

  if forbidden.iter().any(|&x| word.contains(x)) {
    return false;
  }

  let mut found_double = false;
  let mut last_c : char = '\0';
  for c in word.chars() {
    if c == last_c {
      found_double = true;
      break;
    }
    last_c = c;
  }
  if !found_double {
    return false;
  }

  let vowel_counts = vowels.iter().map(|vowel| word.matches(vowel).count());
  let vowels_total = vowel_counts.fold(0, |accum, count| accum + count);
  if vowels_total < 3 {
    return false;
  }

  return true;
}

fn is_nice_v2(word: &str) -> bool {
  let chars: Vec<char> = word.chars().collect();
  let count = chars.len();

  let mut found_double = false;
  for index in 0..(count-3) {
    let c0 = chars[index];
    let c1 = chars[index + 1];
    let mut pair = String::new();
    pair.push(c0);
    pair.push(c1);

    let (_, rest) = word.split_at(index + 2);
    if rest.contains(pair.as_str()) {
      found_double = true;
      break;
    }
  }
  if !found_double {
    return false;
  }

  let mut found_separated = false;
  for index in 0..(count-2) {
    if chars[index] == chars[index + 2] {
      found_separated = true;
      break;
    }
  }
  if !found_separated {
    return false;
  }

//  println!("MATCH {}", word);
  return true;
}

fn main() {
  let path = Path::new("input.txt");
  let fh = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
    Ok(file) => file,
  };
  let reader = io::BufReader::new(fh);

  let mut nice_count_v1 = 0;
  let mut nice_count_v2 = 0;

  for line in reader.lines() {
    let l = line.unwrap();
    let word = l.trim_right();

    if is_nice_v1(word) {
      nice_count_v1 += 1;
    }
    if is_nice_v2(word) {
      nice_count_v2 += 1;
    }
  }

  println!("nice count v1={} v2={}", nice_count_v1, nice_count_v2);
}
