use std::process::Command;

use assert_cmd::prelude::*;
use assert_fs::{prelude::*, TempDir};

#[test]
fn it_outputs_one_files_contents() {
    let expected = "good doggy!\n";

    let temp = TempDir::new().unwrap();
    let f = temp.child("a-file");
    f.write_str(expected).unwrap();
    let path = f.path().to_str().unwrap().to_string();

    let result = Command::cargo_bin("ci-experiments-rust").unwrap().arg(path).unwrap();

    assert_eq!(String::from_utf8(result.stdout).unwrap(), expected);
}
