pub struct Words {
  syn: Vec<&'static str>,
}

pub struct Synonyms {
  noun: &'static str,
  verb: &'static str,
}

pub trait Thesaurus {
  fn synonyms(&self, term: String) -> Result<Vec<&'static str>, String>;
}


pub struct BigHuge {
  pub api_key: String
}

impl Thesaurus for BigHuge {
  fn synonyms(&self, term: String) -> Result<Vec<&'static str>, String> {
      // TODO
    Result::Err("Error".to_string())
  }
}
