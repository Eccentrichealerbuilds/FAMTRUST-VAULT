use crate::router::MyRoute;
use dioxus::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum FieldKind {
    Address,
    Uint256,
    EtherAmount,
    Text,
    Duration,
}
#[derive(Clone, PartialEq, Debug)]
pub enum ContractFunction {
    AddBeneficiary,
    ModifyBeneficiaryShare,
    UnAddBeneficiary,
    ResetUnlocked,
    RequireToUnlockAll,
    SetUnlockTimeByOwner,
    Withdraw,
    Unlock,
    Transfer,
    BeneficiacyAllowance,
    BalancOf,
    UnlockedBenList,
    BenList,
    RequireToUnlock,
    UnlockTime,
    PublicUnlockTime,
}

impl fmt::Display for ContractFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::AddBeneficiary => "Add Beneficiary",
            Self::ModifyBeneficiaryShare => "Modify Beneficiary Share",
            Self::UnAddBeneficiary => "Remove Beneficiary",
            Self::ResetUnlocked => "Reset Unlocked",
            Self::RequireToUnlockAll => "Set Required Unlock Count",
            Self::SetUnlockTimeByOwner => "Set Unlock Time",
            Self::Withdraw => "Withdraw",
            Self::Unlock => "Unlock",
            Self::Transfer => "Claim Your Share",
            Self::BeneficiacyAllowance => "Check Allowance",
            Self::BalancOf => "Contract Balance",
            Self::UnlockedBenList => "Unlocked Beneficiaries",
            Self::BenList => "All Beneficiaries",
            Self::RequireToUnlock => "Required Unlock Count",
            Self::UnlockTime => "Unlock Time (Owner)",
            Self::PublicUnlockTime => "Public Unlock Time",
        };
        write!(f, "{label}")
    }
}

impl ContractFunction {
    pub fn allows_external_contract(&self) -> bool {
        matches!(
            self,
            Self::Unlock
                | Self::Transfer
                | Self::BeneficiacyAllowance
                | Self::UnlockedBenList
                | Self::BenList
                | Self::RequireToUnlock
                | Self::PublicUnlockTime
        )
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Self::AddBeneficiary => "add-beneficiary",
            Self::ModifyBeneficiaryShare => "modify-beneficiary-share",
            Self::UnAddBeneficiary => "un-add-beneficiary",
            Self::ResetUnlocked => "reset-unlocked",
            Self::RequireToUnlockAll => "require-to-unlock-all",
            Self::SetUnlockTimeByOwner => "set-unlock-time",
            Self::Withdraw => "withdraw",
            Self::Unlock => "unlock",
            Self::Transfer => "transfer",
            Self::BeneficiacyAllowance => "beneficiacy-allowance",
            Self::BalancOf => "balanc-of",
            Self::UnlockedBenList => "unlocked-ben-list",
            Self::BenList => "ben-list",
            Self::RequireToUnlock => "require-to-unlock",
            Self::UnlockTime => "unlock-time",
            Self::PublicUnlockTime => "public-unlock-time",
        }
    }

    pub fn from_slug(s: &str) -> Option<Self> {
        Some(match s {
            "add-beneficiary" => Self::AddBeneficiary,
            "modify-beneficiary-share" => Self::ModifyBeneficiaryShare,
            "un-add-beneficiary" => Self::UnAddBeneficiary,
            "reset-unlocked" => Self::ResetUnlocked,
            "require-to-unlock-all" => Self::RequireToUnlockAll,
            "set-unlock-time" => Self::SetUnlockTimeByOwner,
            "withdraw" => Self::Withdraw,
            "unlock" => Self::Unlock,
            "transfer" => Self::Transfer,
            "beneficiacy-allowance" => Self::BeneficiacyAllowance,
            "balanc-of" => Self::BalancOf,
            "unlocked-ben-list" => Self::UnlockedBenList,
            "ben-list" => Self::BenList,
            "require-to-unlock" => Self::RequireToUnlock,
            "unlock-time" => Self::UnlockTime,
            "public-unlock-time" => Self::PublicUnlockTime,
            _ => return None,
        })
    }

    pub fn is_write(&self) -> bool {
        matches!(
            self,
            Self::AddBeneficiary
                | Self::ModifyBeneficiaryShare
                | Self::UnAddBeneficiary
                | Self::ResetUnlocked
                | Self::RequireToUnlockAll
                | Self::SetUnlockTimeByOwner
                | Self::Withdraw
                | Self::Unlock
                | Self::Transfer
        )
    }
}

#[derive(Clone)]
pub struct FieldSpec {
    pub label: &'static str,
    pub placeholder: &'static str,
    pub kind: FieldKind,
}

pub fn fields_for(func: &ContractFunction) -> Vec<FieldSpec> {
    use FieldKind::*;
    match func {
        ContractFunction::AddBeneficiary => vec![
            FieldSpec {
                label: "Beneficiary Address",
                placeholder: "0x...",
                kind: Address,
            },
            FieldSpec {
                label: "Share (e.g. 1.5)",
                placeholder: "1.5",
                kind: EtherAmount,
            },
            FieldSpec {
                label: "Name",
                placeholder: "e.g. John",
                kind: Text,
            },
        ],
        ContractFunction::ModifyBeneficiaryShare => vec![
            FieldSpec {
                label: "New Share (e.g. 1.5)",
                placeholder: "1.5",
                kind: EtherAmount,
            },
            FieldSpec {
                label: "Beneficiary Address",
                placeholder: "0x...",
                kind: Address,
            },
        ],
        ContractFunction::UnAddBeneficiary => vec![FieldSpec {
            label: "Beneficiary Address",
            placeholder: "0x...",
            kind: Address,
        }],
        ContractFunction::RequireToUnlockAll => vec![FieldSpec {
            label: "Required Count",
            placeholder: "e.g. 2",
            kind: Uint256,
        }],
        ContractFunction::SetUnlockTimeByOwner => vec![FieldSpec {
            label: "Unlock Duration from now",
            placeholder: "e.g. 8",
            kind: Duration,
        }],
        ContractFunction::Withdraw => vec![FieldSpec {
            label: "Amount (e.g. 1.5)",
            placeholder: "1.5",
            kind: EtherAmount,
        }],
        ContractFunction::BeneficiacyAllowance => vec![FieldSpec {
            label: "Beneficiary Address",
            placeholder: "0x...",
            kind: Address,
        }],
        _ => vec![],
    }
}

pub static SELECTED_FUNCTION: GlobalSignal<Option<ContractFunction>> = Signal::global(|| None);

const OWNER_WRITE_FNS: &[ContractFunction] = &[
    ContractFunction::AddBeneficiary,
    ContractFunction::ModifyBeneficiaryShare,
    ContractFunction::UnAddBeneficiary,
    ContractFunction::ResetUnlocked,
    ContractFunction::RequireToUnlockAll,
    ContractFunction::SetUnlockTimeByOwner,
    ContractFunction::Withdraw,
];

const BENEFICIARY_WRITE_FNS: &[ContractFunction] =
    &[ContractFunction::Unlock, ContractFunction::Transfer];

const VIEW_FNS: &[ContractFunction] = &[
    ContractFunction::BeneficiacyAllowance,
    ContractFunction::BalancOf,
    ContractFunction::UnlockedBenList,
    ContractFunction::BenList,
    ContractFunction::RequireToUnlock,
    ContractFunction::UnlockTime,
    ContractFunction::PublicUnlockTime,
];

#[component]
pub fn ContractFunctionsPanel() -> Element {
    let address = crate::app::CURRENT_ADDRESS.read().clone();
    let maps = crate::app::WALLET_MAPS.read().clone();
    let current_contract = maps.get(&address).unwrap().contract.clone().unwrap();
    let set_global_contract = crate::app::CURRENT_CONTRACT.signal().set(current_contract);
    rsx! {
        a { class: "w-max h-max",
            Link {
                draggable: false,
                to: MyRoute::Home,
                button {
                    class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                    dioxus_icons::lucide::House { size: 24, color: "white" }
                    "Back Home"
                }
            }
        }
        div {
            class: "flex-1 flex flex-col overflow-auto border-2 p-4 gap-3 rounded-2xl border-white max-w-md mx-auto w-full",

            p { class: "text-white text-center font-semibold text-lg", "Owner Actions" }
            for func in OWNER_WRITE_FNS {
                { function_button(func.clone()) }
            }

            p { class: "text-white text-center font-semibold text-lg mt-3", "Beneficiary Actions" }
            for func in BENEFICIARY_WRITE_FNS {
                { function_button(func.clone()) }
            }

            p { class: "text-white text-center font-semibold text-lg mt-3", "View / Read" }
            for func in VIEW_FNS {
                { function_button(func.clone()) }
            }
        }
    }
}

fn function_button(func: ContractFunction) -> Element {
    rsx! {
        Link {
            draggable: false,
            to: MyRoute::ContractFunctionPage { func: func.slug().to_string() },
            button {
                class: "shrink-0 flex gap-3 mx-auto p-4 px-3 border-2 border-white w-full rounded-2xl hover:bg-zinc-800 transition-colors",
                p { class: "text-white mx-auto font-medium", "{func}" }
            }
        }
    }
}
