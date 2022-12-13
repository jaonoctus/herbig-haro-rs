use std::env;

fn get_current_path() -> String {
    let path = env::current_dir().unwrap();

    path.display().to_string()
}

fn main() {
    // TODO:
    // ask for path (default=current)
    // show options

    // subvolume: create, snapshot, remove
    let _path = "";

    println!("{:?}", get_current_path())
}
