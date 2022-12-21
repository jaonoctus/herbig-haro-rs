use std::process::{Command, Stdio};
use execute::Execute;

const BTRFS_PATH: &str = "/usr/bin/btrfs";

fn assert_btrfs_is_installed() {
    let mut btrfs = Command::new(BTRFS_PATH);
    btrfs.arg("--version");

    if btrfs.execute_check_exit_status_code(0).is_err() {
        eprintln!("The path `{}` is not a correct BTRFS executable binary file.", BTRFS_PATH);
    }
}

fn main() {
    assert_btrfs_is_installed();

    let mut btrfs = Command::new(BTRFS_PATH);
    btrfs.arg("--version");
    btrfs.stdout(Stdio::piped());
    btrfs.stderr(Stdio::piped());

    let output = btrfs.execute_output().unwrap();

    if let Some(exit_code) = output.status.code() {
        if exit_code == 0 {
            println!("Ok.");
        } else {
            eprintln!("Failed.");
        }
    } else {
        eprintln!("Interrupted!");
    }
    
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}
