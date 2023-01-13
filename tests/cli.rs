use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run_cmd(args: &[&str], expected: &str, env_name: &str, env_value: &str) -> TestResult {
    temp_env::with_var(env_name, Some(env_value), || -> TestResult {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
        cmd.args(args)
            .assert()
            .success()
            .stdout(predicate::eq(expected));
        Ok(())
    })
}

#[test]
fn no_args() -> TestResult {
    let args = &[];
    let expected = "export PATH=\"p1:p2\"\n";
    let env_name = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env_name, env_value)
}

#[test]
fn add_before() -> TestResult {
    let args = &["--add", "f1", "f2"];
    let expected = "export PATH=\"f1:f2:p1:p2\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}

#[test]
fn add_last() -> TestResult {
    let args = &["--after", "l1", "l2"];
    let expected = "export PATH=\"p1:p2:l1:l2\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}

#[test]
fn delete_path() -> TestResult {
    let args = &["--delete", "p2", "l2"];
    let expected = "export PATH=\"p1\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}
