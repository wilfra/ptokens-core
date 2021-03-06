use crate::{
    types::Result,
    traits::DatabaseInterface,
    chains::eth::{
        eth_state::EthState,
        eth_database_utils::{
            get_latest_eth_block_number,
            get_public_eth_address_from_db,
            get_erc777_contract_address_from_db,
        },
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EthInitializationOutput {
    pub eth_address: String,
    pub eth_latest_block_num: usize,
    pub eth_ptoken_contract_tx: String,
    pub smart_contract_address: String,
}

impl EthInitializationOutput {
    pub fn new(
        eth_address: String,
        eth_latest_block_num: usize,
        eth_ptoken_contract_tx: String,
        smart_contract_address: String,
    ) -> Result<Self> {
        Ok(
            EthInitializationOutput {
                eth_address,
                eth_latest_block_num,
                eth_ptoken_contract_tx,
                smart_contract_address,
            }
        )
    }
}

fn json_stringify(output: EthInitializationOutput) -> Result<String> {
    match serde_json::to_string(&output) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.into())
    }
}

pub fn get_eth_core_init_output_json<D>(state: EthState<D>) -> Result<String> where D: DatabaseInterface {
    EthInitializationOutput::new(
        format!("0x{}", hex::encode(get_public_eth_address_from_db(&state.db)?.as_bytes())),
        get_latest_eth_block_number(&state.db)?,
        state.get_misc_string()?, // NOTE: Stored here for ease of use!
        format!("0x{}", hex::encode(get_erc777_contract_address_from_db(&state.db)?.as_bytes())),
    )
        .and_then(json_stringify)
}
