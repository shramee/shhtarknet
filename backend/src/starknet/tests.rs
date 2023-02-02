#[cfg(test)]
mod tests {
    use crate::starknet;

    #[tokio::test]
    async fn goerli_call() {
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

        println!("Starknet call!");
        println!("{res:#?}");
        assert_eq!(&res[0], "ETH");
    }
}
