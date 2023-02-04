mod tests;
use std::ops::Add;

use sha256::digest;
use sled::IVec;
use starknet::core::types::FieldElement;

use serde::{Deserialize, Serialize};

use crate::starknet::str_to_felt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    id: FieldElement,
    secret_hash: String,
    contract: String,
}

impl Secret {
    fn new(id: &str, secret: &str, contract: &str) -> Self {
        Self::new_from_hash(id, &digest(secret), contract)
    }

    fn new_from_hash(id: &str, secret_hash: &str, contract: &str) -> Self {
        Self {
            id: str_to_felt(&id),
            secret_hash: secret_hash.into(),
            contract: contract.into(),
        }
    }

    fn compare(&self, candidate: &str) -> bool {
        self.secret_hash == digest(candidate)
    }
}

#[derive(Clone)]
pub struct SecretsManager {
    pub db: sled::Db,
}

impl SecretsManager {
    pub fn new() -> Self {
        Self::new_custom("sled")
    }

    pub fn new_custom( db_name: &str ) -> Self {
        let mut path = std::env::current_dir().unwrap();
        path.push(".data");
        path.push(db_name);
        println!("The current directory is {}", path.display());
        Self {
            db: sled::open(path).unwrap(),
        }
    }

    pub fn key(contract: &str, id: &FieldElement) -> String {
        String::from(contract).add("::").add(&id.to_string())
    }

    pub fn save(&self, secret: Secret) -> sled::Result<Option<IVec>> {
        self.db.insert(
            &Self::key(&secret.contract, &secret.id),
            secret.secret_hash.as_bytes(),
        )
    }

    pub fn get(&self, contract: &str, id: &str) -> Option<Secret> {
        let id_felt = str_to_felt(&id);
        let hash_res = self.db.get(&Self::key(contract, &id_felt));
        match hash_res {
            sled::Result::Ok(v) => match v {
                Some(hash) => {
                    if hash.len() == 64 {
                        Some(Secret::new_from_hash(
                            id,
                            &String::from_utf8(hash.to_vec()).unwrap(),
                            contract,
                        ))
                    } else {
                        None
                    }
                }
                None => None,
            },
            sled::Result::Err(e) => None,
        }
    }
}
