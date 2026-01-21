use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EthgasAPIWholeblockMarketsResponse {
    pub success: bool,
    data: EthgasAPIWholeblockMarketsResponseData,
    pub error_msg_key: Option<String>,
}

impl EthgasAPIWholeblockMarketsResponse {

    pub fn is_multi_relay_for_slot(&self, slot: u64) -> Option<bool> {
        self.data.is_multi_relay_for_slot(slot)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthgasAPIWholeblockMarketsResponseData {
    markets: Vec<WholeblockMarket>,
}

impl EthgasAPIWholeblockMarketsResponseData {
    fn is_multi_relay_for_slot(&self, slot: u64) -> Option<bool> {
        self.markets
            .iter()
            .find(|market| market.slot == slot)
            .map(|market| market.multi_relay)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WholeblockMarket {
    slot: u64,
    multi_relay: bool,
}
