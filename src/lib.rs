#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

use std::io::Read;
use std::{env, fs, str};

pub fn get_input() -> String {
    let path = env::args().nth(1).unwrap();
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
}

pub fn read_input(buf: &mut [u8]) -> &str {
    let path = env::args().nth(1).unwrap();
    let mut file = fs::File::open(path).unwrap();
    let n = file.read(buf).unwrap();

    str::from_utf8(&buf[..n]).unwrap()
}
