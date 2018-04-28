extern crate rand;

use std::io::{self, BufRead};
use rand::thread_rng;
use rand::distributions::{Distribution, Range};

macro_rules! other_word {
  () => ("*")
}

const TRANSFORMS: &[&'static str] = &[
  other_word!(),
  other_word!(),
  other_word!(),
  other_word!(),
  concat!(other_word!(), "app"),
  concat!(other_word!(), "site"),
  concat!(other_word!(), "time"),
  concat!("get", other_word!()),
  concat!("go", other_word!()),
  concat!("lets", other_word!()),
];

fn main() {
  let between = Range::new(0, TRANSFORMS.len());
  let mut rng = thread_rng();

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let i = between.sample(&mut rng);
    let t = TRANSFORMS[i];
    match line {
      Ok(_) => {
        println!("{}", str::replace(t, other_word!(), &line.unwrap()));
      },
      Err(error) => panic!(error)
    };
  }
}
