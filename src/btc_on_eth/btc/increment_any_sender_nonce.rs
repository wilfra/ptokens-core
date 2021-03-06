use crate::{
    types::Result,
    traits::DatabaseInterface,
    btc_on_eth::btc::btc_state::BtcState,
    chains::eth::eth_database_utils::increment_any_sender_nonce_in_db,
};

pub fn maybe_increment_any_sender_nonce_in_db<D>(
    state: BtcState<D>
) -> Result<BtcState<D>>
    where D: DatabaseInterface
{
    if !state.use_any_sender_tx_type() {
        info!("✔ Not incrementing AnySender nonce - not an AnySender transaction!");
        return Ok(state);
    }

    match state.get_eth_signed_txs() {
        Err(_) => {
            info!("✔ Not incrementing AnySender nonce - no signatures made!");
            Ok(state)
        }
        Ok(signed_txs) => {
            info!("✔ Incrementing AnySender nonce by {}", signed_txs.len());
            increment_any_sender_nonce_in_db(
                &state.db,
                signed_txs.len() as u64,
            ).map(|_| state)
        }
    }
}
