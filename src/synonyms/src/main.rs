extern crate thesaurus;

use std::env;
use std::io::{self, BufRead};

use thesaurus::thesaurus::Thesaurus;
use thesaurus::thesaurus as t;


fn main() {
  let api_key = match env::var("BHT_APIKEY") {
    Ok(val) => val,
    Err(err) => panic!(err),
  };

  let provider = t::BigHuge { api_key: api_key };

  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match line {
      Ok(_) => {
        let word = &line.unwrap().to_lowercase();
        let syns = match provider.synonyms(word.to_string()) {
          Ok(val) => val,
          Err(err) => panic!(err),
        };

        for syn in syns {
          println!("{}", syn);
        }
      },
      Err(err) => panic!(err)
    };
  }
}
