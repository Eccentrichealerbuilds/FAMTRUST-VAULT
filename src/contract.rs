#![allow(unused)]
use alloy::{
    primitives::{
        Address, U256, address,
        utils::{Unit, format_ether},
    },
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::Revert,
};
use eyre::Result;

// Codegen from artifact.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    FamilyTrust,
    "./assets/FamilyTrust.json"
);

#[tokio::main]
pub async fn deploy(unlocktime: U256, private_key: String) -> std::result::Result<Address, String> {
    let rpc = "https://testnet-rpc.monad.xyz/";
    let wallet: PrivateKeySigner = private_key.parse().unwrap();
    let provider = ProviderBuilder::new().wallet(wallet).connect(rpc).await;
    if let Err(_) = provider {
        return Err(String::from("Unable to connect to blockchain"));
    }
    let provider = provider.unwrap();
    let deployer = FamilyTrust::deploy(&provider, unlocktime).await;
    if let Err(err) = deployer {
        let reason = err.as_decoded_error::<Revert>();
        let reason = reason.map(|r| r.to_string()).unwrap_or(err.to_string());
        return Err(reason);
    }
    Ok(deployer.unwrap().address().clone())
}
