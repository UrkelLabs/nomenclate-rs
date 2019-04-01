mod common;

#[test]
fn test_get_address_history() {
    let client = common::setup();

    let history = client.get_address_history("ts1qlg60q0tzfr7hr358zqcn97w3ahs5ugt7utdvyu", 0, 25);

    assert!(history.is_ok());
}

#[test]
fn test_get_balance() {
    let client = common::setup();

    let balance = client.get_balance("ts1qlg60q0tzfr7hr358zqcn97w3ahs5ugt7utdvyu");

    assert!(balance.is_ok());
}

#[test]
fn test_get_unspent() {
    let client = common::setup();

    let unspent = client.get_unspent("ts1qlg60q0tzfr7hr358zqcn97w3ahs5ugt7utdvyu", 0, 25);

    assert!(unspent.is_ok());
}

//TODO test mempool endpoint when implemented
