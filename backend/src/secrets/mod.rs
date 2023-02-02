use sha256::digest;

pub struct Secret {
    id: u128,
    secret_hash: String,
    contract: String,
}

impl Secret {
    fn new(secret: String, contract: &str) -> Secret {
        Self {
            id: 0,
            secret_hash: digest(secret),
            contract: contract.into(),
        }
    }
}
