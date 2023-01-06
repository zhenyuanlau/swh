use assert_cmd::Command;

#[test]
fn swh() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("swh")?;
    cmd.assert().failure();
    Ok(())
}

#[test]
fn swh_list() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("swh")?;
    cmd.arg("list").assert().success();
    Ok(())
}
