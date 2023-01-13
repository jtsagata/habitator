use assert_cmd::Command;

#[test]
fn no_args() {
    std::env::set_var("PATH", "p1:p2");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    // cmd.assert().failure().stderr(predicate::str::is_empty());
    cmd.assert().success();
    cmd.assert().success().stdout(r#"export PATH="p1:p2"\n"#);
}
