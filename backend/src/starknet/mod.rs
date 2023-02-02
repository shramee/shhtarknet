#[cfg(test)]
mod tests;

use std::str::FromStr;

use starknet::{
    core::{
        types::{BlockId, CallContractResult, CallFunction, FieldElement},
        utils::get_selector_from_name,
    },
    providers::{Provider, ProviderError, SequencerGatewayProvider},
};

pub struct StarknetManager {
    pub provider: SequencerGatewayProvider,
}

impl StarknetManager {
    pub async fn goerli() -> Self {
        Self {
            provider: SequencerGatewayProvider::starknet_alpha_goerli(),
        }
    }

    pub async fn call(
        self,
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
