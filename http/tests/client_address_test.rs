mod common;

#[test]
fn test_address_history() {
    let client = common::setup();

    let history = client.address_history("ts1qlg60q0tzfr7hr358zqcn97w3ahs5ugt7utdvyu");

    dbg!(history.unwrap());
}

#[test]
fn test_banner() {
    let client = common::setup();

    let banner = client.banner();

    dbg!(banner.unwrap());
}
