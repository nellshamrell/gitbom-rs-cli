use assert_cmd::Command;
use predicates::prelude::*;

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
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("file/does/not/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
#[test]
fn bom_output_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gitbom-cli")?;
    cmd.arg("bom").arg("tests/fixtures/hello.txt");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Generated GitOid: 70c379b63ffa0795fdbfbc128e5a2818397b7ef8"));
    Ok(())
}