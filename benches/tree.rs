#![feature(test)]

use assert_cmd::Command;
extern crate test;
use test::Bencher;
use std::fs;

#[bench]
fn bench_artifact_tree_output_test(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_1")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/large_directory");
    cmd.current_dir("temp_test_dir_1");

    b.iter(||
        // Executes the command
        cmd.unwrap()
    );

    fs::remove_dir_all("temp_test_dir_1")?;

    Ok(())
}

