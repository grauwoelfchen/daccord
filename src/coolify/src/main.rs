#![feature(slice_concat_ext)]
extern crate rand;

use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::slice::SliceConcatExt;
use rand::{Rng, thread_rng};

const DUPLICATE_VOWEL: bool = true;
const REMOVE_VOWEL: bool = false;

fn rand_bool() -> bool {
  let mut rng = thread_rng();
  let x: u32 = rng.gen_range(0, 2); // high: exclusive
  x == 0
}

fn main() {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match line {
      Ok(_) => {
        let text = &line.unwrap();
        let mut word: Vec<char> = text.chars().collect();
        if rand_bool() {
          let mut v_i: i32 = -1;
          for (i, mut c) in word.iter().enumerate() {
            match c {
              'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                if rand_bool() {
                  v_i = i as i32;
                }
              },
              _ => ()
            }
          }
          if v_i >= 0 {
            let i: usize = v_i as usize;
            match rand_bool() {
              DUPLICATE_VOWEL => {
                word = [&word[..(i + 1)], &word[i..]].concat();
              },
              REMOVE_VOWEL => {
                word = [&word[..i], &word[(i + 1)..]].concat();
              }
            }
          }
        }
        println!("{}", String::from_iter(word));
      },
      Err(error) => panic!(error)
    }
  }
}
