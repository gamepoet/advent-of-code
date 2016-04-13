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

  let mut total_paper_needed = 0;

  for iter in reader.lines() {
    let line = iter.unwrap();
    let parts: Vec<&str> = line.split('x').collect();
    let l = parts[0].parse::<u32>().unwrap();
    let w = parts[1].parse::<u32>().unwrap();
    let h = parts[2].parse::<u32>().unwrap();

    let area0 = l*w;
    let area1 = w*h;
    let area2 = h*l;

    let mut smallest_area = area0;
    if area1 < smallest_area {
      smallest_area = area1;
    }
    if area2 < smallest_area {
      smallest_area = area2;
    }

    let surface_area = 2*area0 + 2*area1 + 2*area2;
    let scrap_area = smallest_area;
    let package_area = surface_area + scrap_area;

    total_paper_needed += package_area;
  }

  println!("total paper needed: {}", total_paper_needed);
}
