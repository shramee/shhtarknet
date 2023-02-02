use crate::starknet;

#[tokio::test]
async fn test_goerli_call() {
    let manager = starknet::StarknetManager::goerli().await;
    let resp = manager
        .call(
            "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            "symbol",
            vec![],
        )
        .await
        .unwrap();

    let res: Vec<String> = resp
        .result
        .iter()
        .map(|f| {
            let bytes = f.to_bytes_be().to_vec();
            String::from_utf8(bytes.iter().filter(|v| **v > 0).map(|x| *x).collect()).unwrap()
        })
        .collect();

    assert_eq!(&res[0], "ETH");
}

#[tokio::test]
async fn test_add_account() {
    let key_secret = "00ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let address = "02da37a17affbd2df4ede7120dae305ec36dfe94ec96a8c3f49bbf59f4e9a9fa";
    let manager = starknet::StarknetManager::setup(key_secret, address).await;

    manager.account.expect("Died XO");
}
