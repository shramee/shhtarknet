#[cfg(test)]
mod tests;

use std::str::FromStr;

use starknet::{
    accounts::{single_owner::SignError, Account, AccountError, Call, SingleOwnerAccount},
    core::{
        chain_id,
        types::{AddTransactionResult, BlockId, CallContractResult, CallFunction, FieldElement},
        utils::get_selector_from_name,
    },
    providers::{Provider, ProviderError, SequencerGatewayProvider},
    signers::{local_wallet, LocalWallet, SigningKey},
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
        calldata: Vec<FieldElement>,
    ) -> Result<CallContractResult, ProviderError<<SequencerGatewayProvider as Provider>::Error>>
    {
        let contract_address =
            FieldElement::from_hex_be(contract).expect("invalid contract address");

        self.provider
            .call_contract(
                CallFunction {
                    contract_address: contract_address,
                    entry_point_selector: get_selector_from_name(&entrypoint)
                        .expect("invalid selector name"),
                    calldata,
                },
                BlockId::Latest,
            )
            .await
    }

    pub fn vec_to_felt(calldata: Vec<&str>) -> Vec<FieldElement> {
        calldata
            .iter()
            .map(|d| FieldElement::from_str(d).unwrap())
            .collect()
    }

    pub fn hex_vec_to_felt(calldata: Vec<&str>) -> Vec<FieldElement> {
        calldata
            .iter()
            .map(|d| FieldElement::from_hex_be(d).unwrap())
            .collect()
    }

    pub async fn invoke(
        &self,
        contract: &str,
        entrypoint: &str,
        calldata: Vec<FieldElement>,
    ) -> Result<
        AddTransactionResult,
        AccountError<
            SignError<local_wallet::SignError>,
            <SequencerGatewayProvider as Provider>::Error,
        >,
    > {
        let contract_address =
            FieldElement::from_hex_be(contract).expect("invalid contract address");

        self.account
            .clone()
            .unwrap()
            .execute(vec![Call {
                to: contract_address,
                selector: get_selector_from_name(entrypoint).unwrap(),
                calldata,
            }])
            .send()
            .await
    }
}
