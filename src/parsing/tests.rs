#[test]
fn test_parser_positional() {
    let input = "pos1 pos2 pos3    pos 4";
    let tokens = crate::lexing::tokenise(String::from(input)).unwrap();
    let arguments = crate::parsing::parse(tokens);

    assert_eq!(arguments.len(), 5);
    assert_eq!(arguments[0].value, Some(String::from("pos1")));
    assert_eq!(arguments[1].value, Some(String::from("pos2")));
    assert_eq!(arguments[2].value, Some(String::from("pos3")));
    assert_eq!(arguments[3].value, Some(String::from("pos")));
    assert_eq!(arguments[4].position, Some(4));
}

#[test]
fn test_parser_named() {
    let input = "-named1=value1 -named2=\"value2\" -named3=3.01";
    let tokens = crate::lexing::tokenise(String::from(input)).unwrap();
    let arguments = crate::parsing::parse(tokens);

    assert_eq!(arguments.len(), 3);
    assert_eq!(arguments[0].name, Some(String::from("named1")));
    assert_eq!(arguments[0].value, Some(String::from("value1")));
    assert_eq!(arguments[1].name, Some(String::from("named2")));
    assert_eq!(arguments[1].value, Some(String::from("value2")));
    assert_eq!(arguments[2].name, Some(String::from("named3")));
    assert_eq!(arguments[2].value, Some(String::from("3.01")));
}
