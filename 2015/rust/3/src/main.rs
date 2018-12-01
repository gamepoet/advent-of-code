use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

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

  let mut houses = HashMap::new();
  let mut pos_santa = Point { x: 0, y: 0 };
  let mut pos_robo = pos_santa;
  houses.insert(pos_santa, 1);

  for (index, b) in fh.bytes().enumerate() {
    let c = b.unwrap() as char;

    let mut pos = match index % 2 {
      0 => &mut pos_santa,
      1 => &mut pos_robo,
      _ => panic!("not possible"),
    };

    match c {
      '^' => { pos.y += 1; },
      'v' => { pos.y -= 1; },
      '<' => { pos.x -= 1; },
      '>' => { pos.x += 1; },
      '\n' => { ; },
      _   => panic!("unexpected input: {}", c),
    }

    let visits = houses.entry(*pos).or_insert(0);
    *visits += 1;
  }

  let house_count = houses.len();
  println!("houses visited: {}", house_count);
}
