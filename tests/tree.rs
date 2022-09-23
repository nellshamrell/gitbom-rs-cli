#![feature(test)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
extern crate test;

#[test]
fn artifact_tree_output_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    // Set up temporary test directory
    // Need to create a different one per test since, by default,
    // tests are run in parallel
    // We could also use 'cargo test -- --test-threads 1`
    // whenever we run tests, but I wanted to stick
    // to standard cargo commands as the default as much as possible
    fs::create_dir_all("temp_test_dir_1")?;
 
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("directory/does/not/exist");
    cmd.current_dir("temp_test_dir_1");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    fs::remove_dir_all("temp_test_dir_1")?;

    Ok(())
}

#[test]
fn artifact_tree_output_test() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_2")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated GitBom for 2 files"));

    fs::remove_dir_all("temp_test_dir_2")?;

    Ok(())
}