#![feature(test)]
#![feature(option_result_contains)]

use assert_cmd::Command;
use std::path::Path;
use predicates::prelude::*;
use std::fs;
extern crate test;

#[test]
fn artifact_tree_directory_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
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
fn generates_bom_directory() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_2")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_2");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created GitBOM directory"));

    let bom_dir_exists = Path::new("temp_test_dir_2/.bom").is_dir();
    assert_eq!(bom_dir_exists, true);
    fs::remove_dir_all("temp_test_dir_2")?;
    Ok(())
}

#[test]
fn when_bom_directory_already_exists() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_3/.bom/objects")?;
    let bom_dir_exists = Path::new("temp_test_dir_3/.bom").is_dir();
    assert_eq!(bom_dir_exists, true);

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_3");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitBOM directory already exists"));

    fs::remove_dir_all("temp_test_dir_3")?;
    Ok(())
}

#[test]
fn generating_gitoid_files() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_4")?;
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_4");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha256 GitOid: 99288e47fc18ca8301c2ab1fc67c6d176e344d4528c618705967f8191254bb17\n"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha256 GitOid: 88737472dddbec36c85dc76803dd92c045a5d5c2a1d96c024d16e2fe92f5a734"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha1 GitOid: 3bbaf1bfd298af102de0a2a1065f8ec674daae4c"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha1 GitOid: 7fcdb5a991587f05f251d23f84d0fb3b027d464e"));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated GitBom for 2 files"));

    let first_gitoid_dir_exists = Path::new("temp_test_dir_4/.bom/objects/99/288e47fc18ca8301c2ab1fc67c6d176e344d4528c618705967f8191254bb17").exists();
    assert_eq!(first_gitoid_dir_exists, true);
    let second_gitoid_dir_exists = Path::new("temp_test_dir_4/.bom/objects/88/737472dddbec36c85dc76803dd92c045a5d5c2a1d96c024d16e2fe92f5a734").exists();
    assert_eq!(second_gitoid_dir_exists, true);
    let third_gitoid_dir_exists = Path::new("temp_test_dir_4/.bom/objects/3b/baf1bfd298af102de0a2a1065f8ec674daae4c").exists();
    assert_eq!(third_gitoid_dir_exists, true);
    let fourth_gitoid_dir_exists = Path::new("temp_test_dir_4/.bom/objects/7f/cdb5a991587f05f251d23f84d0fb3b027d464e").exists();
    assert_eq!(fourth_gitoid_dir_exists, true);
    fs::remove_dir_all("temp_test_dir_4")?;
    Ok(())
}

#[test]
fn generating_gitoid_for_sha256_bom_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_5")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_5");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitOid for Sha256 GitBOM file: 9e96d2713315518b59d95efefbe4767f91c1314437f8eab1c15c1017d710e917"));

    let gitoid_dir_exists = Path::new("temp_test_dir_5/.bom/objects/9e/96d2713315518b59d95efefbe4767f91c1314437f8eab1c15c1017d710e917").exists();
    assert_eq!(gitoid_dir_exists, true);

    fs::remove_dir_all("temp_test_dir_5")?;
    Ok(())
}

#[test]
fn generating_gitoid_for_sha1_bom_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_6")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("artifact-tree").arg("../tests/fixtures/directory_thing");
    cmd.current_dir("temp_test_dir_6");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitOid for Sha1 GitBOM file: 97cb5351c67a9f183caa2a19071814e1431984f5"));

    let gitoid_dir_exists = Path::new("temp_test_dir_6/.bom/objects/97/cb5351c67a9f183caa2a19071814e1431984f5").exists();
    assert_eq!(gitoid_dir_exists, true);

    fs::remove_dir_all("temp_test_dir_6")?;
    Ok(())
}
