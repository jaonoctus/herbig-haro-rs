use std::process::{Command, exit};

fn assert_btrfs_is_installed() {
    let output = Command::new("btrfs")
        .arg("--version")
        .output();

    if output.is_err() == true {
        println!("ERROR: Command 'btrfs' not found, but can be installed with:");
        // TODO: check OS first
        println!("$ sudo apt install btrfs-progs");
        exit(1);
    }
}

fn main() {
    assert_btrfs_is_installed();
}
