use assert_cmd::Command;

#[test]
fn pad_left() {
    let mut cmd = Command::cargo_bin("pad").unwrap();
    let assert = cmd.arg("--left").arg("5").arg("0").arg("one").assert();
    assert.stdout("00one\n");
}

#[test]
fn lpad() {
    let mut cmd = Command::cargo_bin("lpad").unwrap();
    let assert = cmd.arg("5").arg(" ").arg("one").assert();
    assert.stdout("  one\n");
}

#[test]
fn pad_right() {
    let mut cmd = Command::cargo_bin("pad").unwrap();
    let assert = cmd.arg("--right").arg("5").arg("0").arg("one").assert();
    assert.stdout("one00\n");
}

#[test]
fn rpad() {
    let mut cmd = Command::cargo_bin("rpad").unwrap();
    let assert = cmd.arg("5").arg(" ").arg("one").assert();
    assert.stdout("one  \n");
}

#[test]
fn pad_stdin() {
    let mut cmd = Command::cargo_bin("pad").unwrap();
    let assert = cmd
        .write_stdin("one\ntwo\nthree\n")
        .arg("--left")
        .arg("5")
        .arg("0")
        .assert();
    assert.stdout("00one\n00two\nthree\n");
}

#[test]
fn pad_unicode_stdin() {
    let mut cmd = Command::cargo_bin("pad").unwrap();
    let assert = cmd
        .write_stdin("one\ntwo\nthree\n")
        .arg("--left")
        .arg("5")
        .arg("🦀")
        .assert();
    assert.stdout("🦀🦀one\n🦀🦀two\nthree\n");
}
