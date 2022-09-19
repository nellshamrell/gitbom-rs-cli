#![feature(test)]

use assert_cmd::Command;
use predicates::prelude::*;
extern crate test;
use test::Bencher;

#[bench]
fn bench_artifact_tree_output_test(b: &mut Bencher) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("tests/fixtures/large_directory");

    b.iter(||
        cmd.assert().success()
    );

    Ok(())
}

