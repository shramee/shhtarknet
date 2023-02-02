use crate::secrets::Secret;

#[test]
fn test_create_secret() {
    let _secret = "abcd";
    let secret = Secret::new(_secret, "contract_addr");
    assert_eq!(secret.contract, "contract_addr");
    assert_eq!(secret.id, 0);
    assert_ne!(secret.secret_hash, _secret);
}

#[test]
fn test_match_secret() {
    let _secret = "abcd";
    let secret = Secret::new(_secret, "contract_addr");

    assert!(
        secret.compare(_secret),
        "Secret hash should match the secret"
    );
}
