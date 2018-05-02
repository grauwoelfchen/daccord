#[cfg(test)]
mod integration_test {
  use std::io::Write;
  use std::process::{Command, Stdio, Output};

  fn run_cmd(name: &'static str, input: &'static str) -> Output {
    let mut cmd = Command::new(format!("./target/debug/{}", name))
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .expect("failed to start");

    {
      let stdin = cmd.stdin.as_mut().expect("failed to open stdin");
      stdin.write_all(input.as_bytes()).expect("failed to write to stdin");
    }
    cmd.wait_with_output().expect("failed to read stdin")
  }

  #[test]
  fn test_run_with_piped_stdin_which_is_empty() {
    let input = "";
    let output = run_cmd("coolify", input);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
  }

  #[test]
  fn test_run_with_piped_stdin_which_contains_3_vowels() {
    let input = "blueprints";
    let output = run_cmd("coolify", input);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let expected = vec![
      "blueprnts\n",
      "bleprints\n",
      "bluprints\n",
      "blueprints\n",
      "bluepriints\n",
      "bluueprints\n",
      "blueeprints\n",
    ];
    assert!(expected.iter().any(|v| {
      String::from(*v) == String::from_utf8_lossy(&output.stdout)
    }));
  }
}
