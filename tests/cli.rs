use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// A Helper function to run cli tests
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
    let args = &["--add", "/opt/bin1", "/opt/pkg/bin"];
    let expected = "export PATH=\"/opt/bin1:/opt/pkg/bin:p1:p2\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}

#[test]
fn add_before_multi() -> TestResult {
    let args = &["--add", "/opt/bin1", "--add", "/opt/pkg/bin"];
    let expected = "export PATH=\"/opt/bin1:/opt/pkg/bin:p1:p2\"\n";
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
fn add_last_first() -> TestResult {
    let args = &["--after", "l1", "--add", "l1"];
    let expected = "export PATH=\"l1:p1:p2\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}

// TODO: after cancel first
#[ignore]
#[test]
fn add_last_first_rev() -> TestResult {
    let args = &["--add", "l1", "--after", "l1"];
    let expected = "export PATH=\"p1:p2:l1\"\n";
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

#[ignore]
#[test]
fn delete_all() -> TestResult {
    let args = &["--delete-all"];
    let expected = "export PATH=\"\"\n";
    let env = "PATH";
    let env_value = "p1:p2";
    run_cmd(args, expected, env, env_value)
}
