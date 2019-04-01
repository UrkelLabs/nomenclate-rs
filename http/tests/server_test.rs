mod common;

#[test]
fn test_get_banner() {
    let client = common::setup();

    let banner = client.get_banner();

    assert!(banner.is_ok());
}

#[test]
fn test_get_features() {
    let client = common::setup();

    let features = client.get_features();

    assert!(features.is_ok());
}

#[test]
fn test_ping() {
    let client = common::setup();

    let ping = client.ping();

    assert!(ping.is_ok());
}

#[test]
fn test_version() {
    let client = common::setup();

    let version = client.get_version();

    assert!(version.is_ok());
}
