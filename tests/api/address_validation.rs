use autoswappr_backend::http::is_valid_address;

#[test]
fn test_valid_address() {
    assert!(is_valid_address(
        "0x40ca979f20ed76f960dc719457eaf0cef3b2c3932d58435b9192a58bc56c1e40"
    ));
}

#[test]
fn test_invalid_addresses() {
    let long_zeros = "0".repeat(64);
    let long_as = "a".repeat(67);

    let invalid_addresses = vec![
        "",
        "0x",
        "0x123",
        "123456",
        "0xXYZabc",
        &long_zeros,
        &long_as,
    ];

    for addr in invalid_addresses {
        assert!(!is_valid_address(addr));
    }
}
