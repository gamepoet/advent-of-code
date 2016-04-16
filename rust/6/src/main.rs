#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

enum Inst {
    Toggle,
    Off,
    On,
}

struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

fn parse_instruction(line: &str) -> (Inst, Rect){
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    }
    let caps = RE.captures(line).unwrap();
    let inst = match &caps[1] {
        "toggle" => Inst::Toggle,
        "turn off" => Inst::Off,
        "turn on" => Inst::On,
        _ => panic!("unexpected text!"),
    };

    let rect = Rect {
        left: caps[2].parse().unwrap(),
        top: caps[3].parse().unwrap(),
        right: caps[4].parse().unwrap(),
        bottom: caps[5].parse().unwrap(),
    };

    return (inst, rect);
}

fn toggle(grid: &mut [[bool; 1000]; 1000], rect: &Rect) {
    for y in rect.top..(rect.bottom+1) {
        for x in rect.left..(rect.right+1) {
            grid[x as usize][y as usize] = !grid[x as usize][y as usize];
        }
    }
}

fn turn_off(grid: &mut [[bool; 1000]; 1000], rect: &Rect) {
    for y in rect.top..(rect.bottom+1) {
        for x in rect.left..(rect.right+1) {
            grid[x as usize][y as usize] = false;
        }
    }
}

fn turn_on(grid: &mut [[bool; 1000]; 1000], rect: &Rect) {
    for y in rect.top..(rect.bottom+1) {
        for x in rect.left..(rect.right+1) {
            grid[x as usize][y as usize] = true;
        }
    }
}

fn count_on(grid: &[[bool; 1000]; 1000]) -> i32 {
    let mut count = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if grid[x][y] {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let path = Path::new("input.txt");
    let fh = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };
    let reader = io::BufReader::new(fh);

    let mut grid = [[false; 1000]; 1000];

    for line in reader.lines() {
        let (inst, rect) = parse_instruction(line.unwrap().trim_right());
        match inst {
            Inst::Toggle => toggle(&mut grid, &rect),
            Inst::Off => turn_off(&mut grid, &rect),
            Inst::On => turn_on(&mut grid, &rect),
        }
    }

    println!("num on: {}", count_on(&grid));
}
