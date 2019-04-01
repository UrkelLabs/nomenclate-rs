mod common;

#[test]
fn test_get_block_header() {
    let client = common::setup();

    let header = client.get_block_header(5, 0);

    assert!(header.is_ok());
}

#[test]
fn test_get_block_header_with_checkpoint() {
    let client = common::setup();

    let header = client.get_block_header(5, 10);

    assert!(header.is_ok());
}

#[test]
fn test_get_block_headers() {
    let client = common::setup();

    let headers = client.get_block_headers(5, 0, 2);

    assert!(headers.is_ok());
}

#[test]
fn test_get_block_headers_with_checkpoint() {
    let client = common::setup();

    let headers = client.get_block_headers(5, 10, 2);

    assert!(headers.is_ok());
}

#[test]
fn test_get_estimate_fee() {
    let client = common::setup();

    let fee = client.get_estimate_fee(5);

    assert!(fee.is_ok());
}

#[test]
fn test_get_relay_fee() {
    let client = common::setup();

    let fee = client.get_relay_fee();

    assert!(fee.is_ok());
}
