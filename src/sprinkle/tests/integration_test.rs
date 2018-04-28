#[cfg(test)]
mod integration_test {
  use std::io::Write;
  use std::process::{Command, Stdio};

  #[test]
  fn test_run_with_piped_stdin() {
    let mut sprinkle = Command::new("./target/debug/sprinkle")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("sprinkle command failed to start");
    {
      let stdin = sprinkle.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all("hoi".as_bytes()).expect("failed to write to stdin");
    }

    let output = sprinkle.wait_with_output().expect("failed to read stdin");

    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert!(String::from_utf8_lossy(&output.stdout).contains("hoi"));
  }
}
