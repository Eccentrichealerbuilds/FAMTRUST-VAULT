use alloy::{
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::Revert,
};
use chrono::DateTime;
use std:: str::FromStr;

sol! {
    #[sol(rpc)]
    contract FamilyTrust {
        function addBeneficiary(address newBeneficiary, uint256 share, string calldata _name) external;
        function modifyBeneficiaryShare(uint256 newShare, address _beneficiary) public;
        function unAddBeneficiary(address toUnAdd) external;
        function unlock() external;
        function resetUnlocked() external;
        function requireToUnlockAll(uint256 _required) external;
        function setUnlockTimeByOwner(uint256 _seconds) external;
        function transfer() external;
        function withdraw(uint256 amount) external;
        function beneficiacyAllowance(address bene) public view returns (uint256);
        function balancOf() public view returns (uint256);
        function unlockedBenList() public view returns (string[] memory);
        function benList() public view returns (string[] memory);
        function _requireToUnlock() external view returns (uint256);
        function unlockTime() public view returns (uint256);
        function publicUnlockTime() public view returns (uint256);
    }
}

pub fn parse_address(s: &str) -> Result<Address, String> {
    Address::from_str(s.trim()).map_err(|_| "Invalid address".to_string())
}

pub fn parse_u256(s: &str) -> Result<U256, String> {
    U256::from_str(s.trim()).map_err(|_| "Invalid number".to_string())
}

fn decode_err(e: impl std::error::Error) -> String {
    e.to_string()
}

pub enum ReadOutput {
    Text(String),
    List(Vec<String>),
}

pub async fn execute_read(
    func: &crate::contract_functions::ContractFunction,
    contract_addr: Address,
    f1: &str,
) -> Result<ReadOutput, String> {
    use crate::contract_functions::ContractFunction::*;
    let rpc = "https://testnet-rpc.monad.xyz/";
    let provider = ProviderBuilder::new()
        .connect(rpc)
        .await
        .map_err(|e| e.to_string())?;
    let contract = FamilyTrust::new(contract_addr, provider);

    let result = match func {
        BeneficiacyAllowance => {
            let addr = parse_address(f1)?;
            contract
                .beneficiacyAllowance(addr)
                .call()
                .await
                .map(|v| ReadOutput::Text(format!("{:.4}", v.to::<u128>() as f64 / 1e18)))
        }
        BalancOf => contract
            .balancOf()
            .call()
            .await
            .map(|v| ReadOutput::Text(format!("{:.4}", v.to::<u128>() as f64 / 1e18))),
        UnlockedBenList => contract
            .unlockedBenList()
            .call()
            .await
            .map(ReadOutput::List),
        BenList => contract.benList().call().await.map(ReadOutput::List),
        RequireToUnlock => contract
            ._requireToUnlock()
            .call()
            .await
            .map(|v| ReadOutput::Text(v.to_string())),
        UnlockTime => contract.unlockTime().call().await.map(|v| {
            let ts = v.to::<u128>() as i64;
            ReadOutput::Text(
                DateTime::from_timestamp(ts, 0)
                    .map(|d| d.to_rfc2822())
                    .unwrap_or_else(|| ts.to_string()),
            )
        }),
        PublicUnlockTime => contract.publicUnlockTime().call().await.map(|v| {
            let ts = v.to::<u128>() as i64;
            ReadOutput::Text(
                DateTime::from_timestamp(ts, 0)
                    .map(|d| d.to_rfc2822())
                    .unwrap_or_else(|| ts.to_string()),
            )
        }),
        _ => return Err("Not a read function".to_string()),
    };

    result.map_err(|e| {
        let reason = e.as_decoded_error::<Revert>();
        reason.map(|r| r.reason).unwrap_or(decode_err(e))
    })
}

pub async fn execute_write(
    func: &crate::contract_functions::ContractFunction,
    contract_addr: Address,
    private_key: &str,
    f1: &str,
    f2: &str,
    f3: &str,
) -> Result<String, String> {
    use crate::contract_functions::ContractFunction::*;

    let signer: PrivateKeySigner = private_key
        .parse()
        .map_err(|_| "Invalid private key".to_string())?;
    let rpc = "https://testnet-rpc.monad.xyz/";
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect(rpc)
        .await
        .map_err(|e| e.to_string())?;
    let contract = FamilyTrust::new(contract_addr, provider);

    let call_result = match func {
        AddBeneficiary => {
            let addr = parse_address(f1)?;
            let share = parse_ether_amount(f2)?; // was parse_u256
            contract
                .addBeneficiary(addr, share, f3.to_string())
                .send()
                .await
        }
        ModifyBeneficiaryShare => {
            let share = parse_ether_amount(f1)?; // was parse_u256
            let addr = parse_address(f2)?;
            contract.modifyBeneficiaryShare(share, addr).send().await
        }
        UnAddBeneficiary => contract.unAddBeneficiary(parse_address(f1)?).send().await,
        Unlock => contract.unlock().send().await,
        ResetUnlocked => contract.resetUnlocked().send().await,
        RequireToUnlockAll => contract.requireToUnlockAll(parse_u256(f1)?).send().await,
        SetUnlockTimeByOwner => contract.setUnlockTimeByOwner(parse_u256(f1)?).send().await,
        Transfer => contract.transfer().send().await,
        Withdraw => contract.withdraw(parse_ether_amount(f1)?).send().await, // was parse_u256
        _ => return Err("Not a write function".to_string()),
    };

    match call_result {
        Ok(pending) => match pending.get_receipt().await {
            Ok(receipt) => Ok(format!("{:?}", receipt.transaction_hash)),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => {
            let reason = e.as_decoded_error::<Revert>();
            Err(reason.map(|r| r.reason).unwrap_or(decode_err(e)))
        }
    }
}

use alloy::primitives::utils::parse_ether;

pub fn parse_ether_amount(s: &str) -> Result<U256, String> {
    parse_ether(s.trim()).map_err(|_| "Invalid amount".to_string())
}
