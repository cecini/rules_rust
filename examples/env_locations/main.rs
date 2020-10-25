#[test]
fn test() {
    let generated_data = std::fs::read_to_string(env!("GENERATED_DATA")).unwrap();
    // our generated data file should be readable
    assert_eq!(generated_data, "hello\n");
    // and we should be able to read (and thus execute) our tool
    assert_eq!(std::fs::read(env!("SOME_TOOL")).unwrap().is_empty(), false);
}
