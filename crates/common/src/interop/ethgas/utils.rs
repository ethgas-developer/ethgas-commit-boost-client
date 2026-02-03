use alloy::primitives::U256;
use eyre::Result;
use reqwest::Client;
use tracing::{error, debug};
use url::Url;
use crate::{
    types::Chain,
    pbs::{BuilderBid, GetHeaderResponse}, 
    interop::ethgas::types::EthgasAPIWholeblockMarketsResponse
};


pub fn adjust_ethgas_bid_value(res: &mut GetHeaderResponse) {
    // subtract 11,000 ETH from the bid value for ethgas relays
    let delta = U256::from(11_000u64) * U256::from(1_000_000_000_000_000_000u128);
    match &mut res.data.message {
        BuilderBid::Bellatrix(bid) => bid.value -= delta,
        BuilderBid::Capella(bid) => bid.value -= delta,
        BuilderBid::Deneb(bid) => bid.value -= delta,
        BuilderBid::Electra(bid) => bid.value -= delta,
        BuilderBid::Fulu(bid) => bid.value -= delta,
        BuilderBid::Gloas(bid) => bid.value -= delta,
    }
}

pub fn restore_ethgas_bid_value(res: &mut GetHeaderResponse) {
    let delta = U256::from(11_000u64) * U256::from(1_000_000_000_000_000_000u128);
    match &mut res.data.message {
        BuilderBid::Bellatrix(bid) => bid.value += delta,
        BuilderBid::Capella(bid) => bid.value += delta,
        BuilderBid::Deneb(bid) => bid.value += delta,
        BuilderBid::Electra(bid) => bid.value += delta,
        BuilderBid::Fulu(bid) => bid.value += delta,
        BuilderBid::Gloas(bid) => bid.value += delta,
    }
}

pub async fn fetch_is_multi_relay(chain: &Chain, slot: u64) -> Result<bool> {
    let exchange_base_url = match chain {
        Chain::Mainnet => "https://mainnet.app.ethgas.com",
        Chain::Hoodi => "https://hoodi.app.ethgas.com",
        _ => return Err(std::io::Error::other( "unsupported chain") .into()),
    };
    let exchange_api_url = Url::parse(&format!("{}{}{}", exchange_base_url, "/api/v1/p/wholeblock/market?slot=", slot))?;
    let client = Client::new();
    let res = client
        .get(exchange_api_url.to_string())
        .header("User-Agent", "cb_ethgas_pbs")
        .send()
        .await?;
    let is_multi_relay = match res.json::<EthgasAPIWholeblockMarketsResponse>().await {
        Ok(result) => match result.success {
            true => {
                match result.is_multi_relay_for_slot(slot) {
                    Some(is_multi_relay) => is_multi_relay,
                    None => {
                        error!("multi_relay not found from wholeblock markets API, is_multi_relay is set to true by default");
                        return Ok(true)
                    }
                }
            },
            false => {
                error!(
                    "failed to get successful result from wholeblock markets API: {}, is_multi_relay is set to true by default",
                    result.error_msg_key.unwrap_or_default()
                );
                return Ok(true)
            }
        },
        Err(err) => {
            error!(?err, "failed to call wholeblock markets API, is_multi_relay is set to true by default");
            return Ok(true)
        }
    };
    debug!(is_multi_relay, "ethgas wholeblock markets API");

    Ok(is_multi_relay)
}
