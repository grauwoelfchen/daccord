#[cfg(test)]
mod integration_test {
  use std::io::Write;
  use std::process::{Command, Stdio};

  #[test]
  fn test_run_with_piped_stdin() {
    let mut domainify = Command::new("./target/debug/domainify")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("domainify command failed to start");
    {
      let stdin = domainify.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all("hoi".as_bytes()).expect("failed to write to stdin");
    }

    let output = domainify.wait_with_output().expect("failed to read stdin");

    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(String::from_utf8_lossy(&output.stdout).contains("hoi"));
  }

  #[test]
  fn test_run_with_capital_letters_and_white_space() {
    let input = "Hello Domainify";

    let mut domainify = Command::new("./target/debug/domainify")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("domainify command failed to start");
    {
      let stdin = domainify.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all(input.as_bytes()).expect("failed to write to stdin");
    }

    let output = domainify.wait_with_output().expect("failed to read stdin");

    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(String::from_utf8_lossy(&output.stdout).contains(
      "hello-domainify"));
  }

  #[test]
  fn test_run_with_capital_letters_white_space_and_signs() {
    let input = "\"What's up?\"";

    let mut domainify = Command::new("./target/debug/domainify")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("domainify command failed to start");
    {
      let stdin = domainify.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all(input.as_bytes()).expect("failed to write to stdin");
    }

    let output = domainify.wait_with_output().expect("failed to read stdin");

    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(String::from_utf8_lossy(&output.stdout).contains(
      "whats-up"));
  }

  #[test]
  fn test_run_with_capital_letters_and_signs_with_parentheses() {
    let input = "One (two) three!";

    let mut domainify = Command::new("./target/debug/domainify")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("domainify command failed to start");
    {
      let stdin = domainify.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all(input.as_bytes()).expect("failed to write to stdin");
    }

    let output = domainify.wait_with_output().expect("failed to read stdin");

    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(String::from_utf8_lossy(&output.stdout).contains(
      "one-two-three"));
  }
}
