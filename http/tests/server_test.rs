mod common;

#[test]
fn test_get_banner() {
    let client = common::setup();

    let banner = client.get_banner();

    dbg!(banner.unwrap());

    // assert!(banner.is_ok());
}
