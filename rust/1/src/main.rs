use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let mut floor = 0;
  let mut first_basement_index = -1;

  let path = Path::new("input.txt");
  let fh = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
    Ok(file) => file,
  };
  for (index, b) in fh.bytes().enumerate() {
    let c = b.unwrap() as char;
    if c == '(' {
      floor += 1;
    }
    else if c == ')' {
      floor -= 1;
      if first_basement_index == -1 && floor < 0 {
        first_basement_index = index as i32;
      }
    }
  }

  println!("floor: {}, first_basement_index: {}\n", floor, first_basement_index + 1);
}
