use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;
use std::fs;

#[test]
fn basic_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("A CLI for creating GitBom documents"));

    Ok(())
}

#[test]
fn help_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI for creating GitBom documents"));
    Ok(())
}

#[test]
fn bom_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    // Need to create a different one per test since, by default,
    // tests are run in parallel
    // We could also use 'cargo test -- --test-threads 1`
    // whenever we run tests, but I wanted to stick
    // to standard cargo commands as the default as much as possible
    fs::create_dir_all("temp_test_dir_1")?;
 
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("file/does/not/exist");
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
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
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
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
    cmd.current_dir("temp_test_dir_3");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitBOM directory already exists"));

    fs::remove_dir_all("temp_test_dir_3")?;
    Ok(())
}

#[test]
fn generating_sha256_gitoid_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_4")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
    cmd.current_dir("temp_test_dir_4");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha256 GitOid: 5b2f2d4e79e6387ca9dedad500ebf70e9fb3097773252cc5b9a6d5a35a987028"));

    let gitoid_dir_exists = Path::new("temp_test_dir_4/.bom/objects/5b/2f2d4e79e6387ca9dedad500ebf70e9fb3097773252cc5b9a6d5a35a987028").exists();
    assert_eq!(gitoid_dir_exists, true);
    fs::remove_dir_all("temp_test_dir_4")?;
    Ok(())
}

#[test]
fn generating_sha1_gitoid_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_5")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
    cmd.current_dir("temp_test_dir_5");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated Sha1 GitOid: 70c379b63ffa0795fdbfbc128e5a2818397b7ef8"));

    let gitoid_dir_exists = Path::new("temp_test_dir_5/.bom/objects/70/c379b63ffa0795fdbfbc128e5a2818397b7ef8").exists();
    assert_eq!(gitoid_dir_exists, true);

    fs::remove_dir_all("temp_test_dir_5")?;
    Ok(())
}

#[test]
fn generating_sha256_bom_gitoid_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_6")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
    cmd.current_dir("temp_test_dir_6");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitOid for Sha256 GitBOM file: 2ae40c6af39af6d75cca304157d2483e8e85526eaad7b7d2a2a386343d5efad7"));

    let gitoid_dir_exists = Path::new("temp_test_dir_6/.bom/objects/2a/e40c6af39af6d75cca304157d2483e8e85526eaad7b7d2a2a386343d5efad7").exists();
    assert_eq!(gitoid_dir_exists, true);


    let file_contents = fs::read_to_string("temp_test_dir_6/.bom/objects/2a/e40c6af39af6d75cca304157d2483e8e85526eaad7b7d2a2a386343d5efad7")?;
    println!("{}", file_contents);
    assert!(file_contents.contains("gitoid:blob:sha256\n"));

    fs::remove_dir_all("temp_test_dir_6")?;
    Ok(())
}

#[test]
fn generating_sha1_bom_gitoid_file() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("temp_test_dir_7")?;

    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("../tests/fixtures/hello.txt");
    cmd.current_dir("temp_test_dir_7");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("GitOid for Sha1 GitBOM file: 8b8e6bf1664bb88b8f979816fe2b73585a62580d"));

    let gitoid_dir_exists = Path::new("temp_test_dir_7/.bom/objects/8b/8e6bf1664bb88b8f979816fe2b73585a62580d").exists();
    assert_eq!(gitoid_dir_exists, true);
 
     
    let file_contents = fs::read_to_string("temp_test_dir_7/.bom/objects/8b/8e6bf1664bb88b8f979816fe2b73585a62580d")?;
    assert!(file_contents.contains("gitoid:blob:sha1\n"));

    fs::remove_dir_all("temp_test_dir_7")?;
    Ok(())
}