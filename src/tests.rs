#[test]
fn test_cmd_args_positional() {
    let input = "pos1 pos2 pos3    pos 4";
    let arguments = crate::CommandArguments::from_input(input).unwrap();

    assert!(arguments.get_positional_count() == 5);
    assert!(arguments.get_positional(0) == Some("pos1"));
    assert!(arguments.get_positional(1) == Some("pos2"));
    assert!(arguments.get_positional(2) == Some("pos3"));
    assert!(arguments.get_positional(3) == Some("pos"));
    assert!(arguments.get_positional(4) == Some("4"));
}

#[test]
fn test_cmd_args_named() {
    let input = "-named1=value1 -named2=\"value2\" -named3=3.01";
    let arguments = crate::CommandArguments::from_input(input).unwrap();

    assert!(arguments.get_named("named1") == Some("value1"));
    assert!(arguments.get_named("named2") == Some("value2"));
    assert!(arguments.get_named("named3") == Some("3.01"));
}
