extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;


fn main() {
  let path = Path::new("input.txt");
  let fh = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
    Ok(file) => file,
  };
  let reader = io::BufReader::new(fh);

  let forbidden = ["ab", "cd", "pq", "xy"];
  let vowels = ["a", "e", "i", "o", "u"];

  let mut nice_count = 0;

  for line in reader.lines() {
    let l = line.unwrap();
    let word = l.trim_right();

    if forbidden.iter().any(|&x| word.contains(x)) {
//      println!("{} REJECT forbidden", word);
      continue;
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
//      println!("{} REJECT no-doubles", word);
      continue;
    }

    let vowel_counts = vowels.iter().map(|vowel| word.matches(vowel).count());
    let vowels_total = vowel_counts.fold(0, |accum, count| accum + count);
    if vowels_total < 3 {
//      println!("{} REJECT not-enough-vowels", word);
      continue;
    }

//    println!("{} MATCH", word);
    nice_count += 1;
  }

  println!("nice count: {}", nice_count);
}
