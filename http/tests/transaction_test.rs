mod common;

#[test]
fn test_get_transaction_hex() {
    let client = common::setup();

    let hex = client.get_transaction_hex(
        "5c500def0d0ebf0fe325d66ce7e56992809d5da7837c6dd563cbea272e8ab201",
        false,
    );

    assert!(hex.is_ok());
}

#[test]
fn test_get_transaction_hex_merkle() {
    let client = common::setup();

    let hex = client.get_transaction_hex(
        "5c500def0d0ebf0fe325d66ce7e56992809d5da7837c6dd563cbea272e8ab201",
        true,
    );

    assert!(hex.is_ok());
}

//TODO Verbose transaction -- needs Handshake Rust library types to be implemented.

#[test]
fn test_get_transaction_merkle() {
    let client = common::setup();

    let tx = client.get_transaction_merkle(
        "0ded93a1f39b9b0b3692dbfdb89dce9a8777608f27de365ecb8cf04d94450e88",
        9634,
    );

    assert!(tx.is_ok());
}

#[test]
fn test_get_transaction_by_position() {
    let client = common::setup();

    let tx = client.get_transaction_by_position(9629, 1, false);

    assert!(tx.is_ok());
}

#[test]
fn test_get_transaction_by_position_merkle() {
    let client = common::setup();

    let tx = client.get_transaction_by_position(9629, 1, true);

    assert!(tx.is_ok());
}

#[test]
fn test_broadcast_transaction() {
    let client = common::setup();

    let tx = client.broadcast_transaction("0000000001b2265750023129fdc1ab7d7e319541c544706a9cdbbea7e3284151772d1ce6b100000000ffffffff02404b4c0000000000001408317f8fbc290d8c49de9d69f5482b7dd53c01400000d8894e3b00000000001424370d7473cd791a72d7f1a9e8fcf56287ad1b2f000000000000024158567fca6c9b52455da01b4b600f2db65842f13edd7fbb129ea56b1be13081327b942455bb34a4222a6dda6cf3ee725180762b58da9d21df51d38579c097f791012102e2561970d746f92e6e646b3dbfefbe3fe9d9e9a073d21dd16f2ca0ee39674c1f");

    assert!(tx.is_ok());
}
