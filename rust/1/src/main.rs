use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  let mut floor = 0;

  let path = Path::new("input.txt");
  let fh = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
    Ok(file) => file,
  };
  for b in fh.bytes() {
    let c = b.unwrap() as char;
    if c == '(' {
      floor += 1;
    }
    else if c == ')' {
      floor -= 1;
    }
  }

  println!("floor: {}\n", floor);
}
