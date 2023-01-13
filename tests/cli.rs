use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn no_args() {
    temp_env::with_var("PATH", Some("p1:p2"), || {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.assert().success();
        cmd.assert()
            .success()
            .stdout(predicate::eq("export PATH=\"p1:p2\"\n"));
    });
}
