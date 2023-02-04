// use crate::starknet;
use crate::starknet::{felt_to_str, StarknetManager};

use super::str_to_felt;

#[tokio::test]
async fn test_goerli_call() {
    let manager = StarknetManager::goerli().await;
    let resp = manager
        .call(
            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            "symbol",
            vec![],
        )
        .await
        .unwrap();

    let res: Vec<String> = resp.result.iter().map(|f| felt_to_str(f)).collect();

    assert_eq!(&res[0], "ETH");
}

#[tokio::test]
async fn test_add_account() {
    let key_secret = "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let address = "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa";
    let manager = StarknetManager::setup(key_secret, address).await;

    assert!(manager.account.is_some(), "Account should be set");
}

#[tokio::test]
async fn test_invoke() {
    let key_secret = "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let address = "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa";
    let manager = StarknetManager::setup(key_secret, address).await;

    let resp = manager
        .invoke(
            "07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
            "mint",
            StarknetManager::hex_vec_to_felt(vec![
                "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa",
                "ffffff",
                "0",
            ]),
        )
        .await;

    assert!(resp.is_ok(), "Invoke shouldn't have an error");
}

#[test]
fn test_felts() {
    let subject = "my_test";
    let sub_felt = str_to_felt(subject);
    let sub_str = felt_to_str(&sub_felt);

    assert_eq!(sub_str, subject)
}
