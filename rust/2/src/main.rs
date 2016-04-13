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
  let mut total_ribbon_needed = 0;

  for iter in reader.lines() {
    // parse the package dimensions
    let line = iter.unwrap();
    let dims_str: Vec<&str> = line.split('x').collect();
    let dims_int = dims_str.iter().map(|x| x.parse::<u32>().unwrap());
    let mut dims: Vec<u32> = dims_int.collect();
    dims.sort();

    // calculate paper area
    let mut areas = vec![
      dims[0] * dims[1],
      dims[1] * dims[2],
      dims[2] * dims[0],
    ];
    areas.sort();

    let surface_area = areas.iter().map(|x| 2*x).fold(0, |accum, x| accum + x);
    let scrap_area = areas[0];
    let package_area = surface_area + scrap_area;

    total_paper_needed += package_area;

    // calculate ribbon
    let ribbon_perimeter = 2*dims[0] + 2*dims[1];
    let ribbon_bow = dims.iter().fold(1, |accum, x| accum * x);
    let ribbon_len = ribbon_perimeter + ribbon_bow;

    total_ribbon_needed += ribbon_len;
  }

  println!("total paper needed: {}, ribbon: {}", total_paper_needed, total_ribbon_needed);
}
