use std::{fs,env};

fn main() {
    let generated_data = fs::read_to_string(env::var("GENERATED_DATA").unwrap()).unwrap();
    // our generated data file should be readable as the build script runs
    assert_eq!(generated_data, "hello\n");
    // and we should be able to read (and thus execute) our tool
    assert_eq!(fs::read(env::var("SOME_TOOL").unwrap()).unwrap().is_empty(), false);
}
