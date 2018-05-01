#![feature(pattern)]

extern crate rand;

use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::str::pattern::Pattern;
use rand::thread_rng;
use rand::distributions::{Distribution, Range};

const TLDS: &[&'static str] = &[
  "com",
  "net",
];

const ALLOWED_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789_-";

fn main() {
  let between = Range::new(0, TLDS.len());
  let mut rng = thread_rng();

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match line {
      Ok(_) => {
        let text = &line.unwrap().to_lowercase();
        let mut new_text: Vec<char> = vec![];
        for mut r in text.chars() {
          if r.is_whitespace() {
            r = '-';
          }
          if !r.is_contained_in(ALLOWED_CHARS) {
            continue;
          }
          new_text.push(r);
        }
        let i = between.sample(&mut rng);
        println!("{}", String::from_iter(new_text) + "." + TLDS[i]);
      },
      Err(error) => panic!(error)
    };
  }
}
