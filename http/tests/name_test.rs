mod common;

#[test]
fn test_get_name_history() {
    let client = common::setup();

    let history = client.get_name_history("number", 0, 25);

    assert!(history.is_ok());
}
