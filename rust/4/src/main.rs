extern crate crypto;

use crypto::digest::Digest;
use crypto::md5;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let path = Path::new("input.txt");
  let fh = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
    Ok(file) => file,
  };
  let mut reader = io::BufReader::new(fh);

  let mut prefix = String::new();
  reader.read_line(&mut prefix).ok();
  prefix = prefix.trim_right().to_string();
  println!("prefix: '{}'", prefix);

  let mut hasher = md5::Md5::new();

  let mut number = 1;
  loop {
    if number % 10000 == 0 {
      print!("\r{}", number);
      io::stdout().flush().ok().expect("Could not flush stdout");
    }

    let text = format!("{}{}", prefix, number);
    hasher.reset();
    hasher.input_str(text.as_str());
    let hash = hasher.result_str();

    if hash.starts_with("000000") {
      println!("\n[{}] {} {}", number, text, hash);
      break;
    }

    number += 1;
  }
}
