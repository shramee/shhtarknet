#[cfg(test)]
mod tests;

use std::str::FromStr;

use starknet::{
    accounts::SingleOwnerAccount,
    core::{
        chain_id,
        types::{BlockId, CallContractResult, CallFunction, FieldElement},
        utils::get_selector_from_name,
    },
    providers::{Provider, ProviderError, SequencerGatewayProvider},
    signers::{LocalWallet, SigningKey},
};

pub struct StarknetManager {
    pub provider: SequencerGatewayProvider,
    pub account: Option<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>>,
}

impl StarknetManager {
    pub async fn goerli() -> Self {
        Self {
            provider: SequencerGatewayProvider::starknet_alpha_goerli(),
            account: None,
        }
    }

    pub async fn setup(key_secret: &str, address: &str) -> Self {
        Self {
            provider: SequencerGatewayProvider::starknet_alpha_goerli(),
            account: Some(Self::prepare_account(key_secret, address)),
        }
    }

    pub fn prepare_account(
        key_secret: &str,
        address: &str,
    ) -> SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> {
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(
            FieldElement::from_hex_be(key_secret).unwrap(),
        ));
        let address = FieldElement::from_hex_be(address).unwrap();
        SingleOwnerAccount::new(
            SequencerGatewayProvider::starknet_alpha_goerli(),
            signer,
            address,
            chain_id::TESTNET,
        )
    }

    pub async fn call(
        &self,
        contract: &str,
        entrypoint: &str,
        calldata: Vec<&str>,
    ) -> Result<CallContractResult, ProviderError<<SequencerGatewayProvider as Provider>::Error>>
    {
        let contract_address =
            FieldElement::from_hex_be(contract).expect("invalid FieldElement value");

        self.provider
            .call_contract(
                CallFunction {
                    contract_address: contract_address,
                    entry_point_selector: get_selector_from_name(&entrypoint)
                        .expect("invalid selector name"),
                    calldata: calldata
                        .iter()
                        .map(|d| FieldElement::from_str(d).unwrap())
                        .collect(),
                },
                BlockId::Latest,
            )
            .await
    }
}
