use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EthgasAPIWholeblockMarketsResponse {
    pub success: bool,
    data: EthgasAPIWholeblockMarketsResponseData,
    pub error_msg_key: Option<String>,
}

impl EthgasAPIWholeblockMarketsResponse {

    pub fn relay_mode_for_slot(&self, slot: u64) -> Option<u8> {
        self.data.relay_mode_for_slot(slot)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthgasAPIWholeblockMarketsResponseData {
    markets: Vec<WholeblockMarket>,
}

impl EthgasAPIWholeblockMarketsResponseData {
    fn relay_mode_for_slot(&self, slot: u64) -> Option<u8> {
        self.markets
            .iter()
            .find(|market| market.slot == slot)
            .map(|market| market.relay_mode)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WholeblockMarket {
    slot: u64,
    relay_mode: u8,
}
