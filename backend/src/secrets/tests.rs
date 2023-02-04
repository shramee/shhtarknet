use starknet::core::types::FieldElement;

use crate::secrets::{str_to_felt, Secret, SecretsManager};

#[test]
fn test_create_secret() {
    let _secret = "abcd";
    let secret = Secret::new("my_secret", _secret, "contract_addr");
    assert_eq!(secret.contract, "contract_addr");
    assert_ne!(secret.secret_hash, _secret);
}

#[test]
fn test_match_secret() {
    let _secret = "abcd";
    let secret = Secret::new("my_secret", _secret, "contract_addr");

    assert!(
        secret.compare(_secret),
        "Secret hash should match the secret"
    );
}

#[test]
fn save_secret() {
    let db_man = SecretsManager::new();
    let secret_orig = Secret::new("my_secret", "abcd", "contract_addr");
    let res = db_man.save(secret_orig);
    res.expect("Couldn't save...");

    let secret = db_man.get("contract_addr", "my_secret").unwrap();

    assert!(secret.compare("abcd"));
    assert!(!secret.compare("abc"));
}
