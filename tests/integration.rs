use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn basic_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Creates a GitBOM for a single file"));

    Ok(())
}