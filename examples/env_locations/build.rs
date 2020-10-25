use std::{fs,env};

fn main() {
    // walk up and locate bazel-out
    let mut execroot = env::current_dir().unwrap(); 
    let mut should_continue = true;
    loop {
        should_continue = !execroot.ends_with("bazel-out");
        execroot.pop();
        if !should_continue {
            break;
        }
    }

    // our generated data file should be readable
    let path = execroot.join(env::var("GENERATED_DATA").unwrap());
    let generated_data = fs::read_to_string(&path).unwrap();
    assert_eq!(generated_data, "hello\n");

    // and we should be able to read (and thus execute) our tool
    let path = execroot.join(env::var("SOME_TOOL").unwrap());
    assert_eq!(fs::read(&path).unwrap().is_empty(), false);
}
