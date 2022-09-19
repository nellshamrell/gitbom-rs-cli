#![feature(test)]

use assert_cmd::Command;
use predicates::prelude::*;
extern crate test;

#[test]
fn artifact_tree_output_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("directory/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}

#[test]
fn artifact_tree_output_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("tests/fixtures/directory_thing");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated GitBom for 2 files"));
    Ok(())
}