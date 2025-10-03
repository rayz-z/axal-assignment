use alloy::sol;
use std::error::Error;
use alloy::{
    providers::{ProviderBuilder, RootProvider},
};


sol! {
    #[sol(rpc)]
    contract MorphoVault {
        function totalAssets() public view returns (uint256);
        function lastTotalAssets() public view returns (uint256);
    }
}

pub fn calculate_tvl_change(stored_tvl: u128, new_tvl: u128) -> (i128, bool) {
    let change = stored_tvl as i128 - new_tvl as i128;
    let warning = if stored_tvl > 0 {
        (change as f64) / (stored_tvl as f64) > 0.2
    } else {
        false
    };
    (change, warning)
}

pub async fn fetch_tvl() -> Result<(u128, u128), Box<dyn Error>> {
    let pb = ProviderBuilder::new();
    let provider = pb.connect_http(
        "https://base-mainnet.g.alchemy.com/v2/UnNVlVMIq5tqJ_0gv4tXrg4FNrbX2F4U".parse()?,
    );

    let morpho = MorphoVault::new(
        "0x616a4E1db48e22028f6bbf20444Cd3b8e3273738".parse()?,
        provider,
    );

    let tvl = morpho.totalAssets().call().await?;
    let last_tvl = morpho.lastTotalAssets().call().await?;

    Ok((tvl.try_into()?, last_tvl.try_into()?))
}


