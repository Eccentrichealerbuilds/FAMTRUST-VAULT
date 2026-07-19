use crate::ContractFunctionsPanel;
#[cfg(target_os = "android")]
use crate::android::key_handler::save_json;
use crate::app::CURRENT_ADDRESS;
use crate::app::CURRENT_CONTRACT;
use crate::app::KeyPass;
use crate::app::WALLET_MAPS;
use crate::contract::deploy;
use crate::router::MyRoute;
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
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_icons::lucide;
use eyre::Result;
use std::str::FromStr;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    FamilyTrust,
    "./assets/FamilyTrust.json"
);

#[component]
pub fn InteractWithContract() -> Element {
    let mut is_configuring = use_signal(|| false);
    let mut wallet_contract = String::new();
    let mut has_contract_signal = use_signal(|| false);
    let has_contract = WALLET_MAPS.cloned();
    let address = CURRENT_ADDRESS.cloned();
    let has_contract = has_contract.get(&address).unwrap();
    let has_contract = has_contract.clone().contract;
    if let Some(contract) = has_contract {
        wallet_contract = contract;
        has_contract_signal.set(true);
    }
    rsx! {
        div {
            class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",

                p {
                    "FAM TRUST VAULT"
                }
            }if !has_contract_signal() {
                CreateContract { is_configuring, has_contract_signal }
            }
            else if has_contract_signal() && is_configuring() {
                ContractConfiguration {  }
            }else if has_contract_signal() {
                ContractFunctionsPanel {  }
            }
            div{
                class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p {class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }
        }
    }
}

#[component]
fn CreateContract(is_configuring: Signal<bool>, has_contract_signal: Signal<bool>) -> Element {
    let mut valid = use_signal(|| false);
    let mut is_loading = use_signal(|| false);
    let mut unable_to_deploy = use_signal(|| false);
    let mut mode_selected = use_signal(|| String::new());
    let mut amount_entered = use_signal(|| String::new());
    let mut status = use_signal(|| String::new());

    let mut deployment = move |_| async move {
        is_loading.set(true);

        let amount_entered = match amount_entered().parse::<u128>() {
            Ok(v) => v,
            Err(_) => {
                status.set("Invalid amount".into());
                is_loading.set(false);
                return;
            }
        };

        let unlocktime = if mode_selected() == "seconds" {
            amount_entered
        } else if mode_selected() == "minutes" {
            amount_entered * 60
        } else if mode_selected() == "hours" {
            amount_entered * 60 * 60
        } else if mode_selected() == "days" {
            amount_entered * 60 * 60 * 24
        } else {
            amount_entered
        };

        let private_key = match WALLET_MAPS.read().get(&CURRENT_ADDRESS()) {
            Some(acct) => acct.clone().private_key,
            None => {
                status.set("No wallet found".into());
                is_loading.set(false);
                return;
            }
        };

        let unlock_time: U256 = match unlocktime.to_string().trim().parse::<U256>() {
            Ok(v) => v,
            Err(_) => {
                status.set("Invalid unlock time".into());
                is_loading.set(false);
                return;
            }
        };

        let rpc = "https://testnet-rpc.monad.xyz/";

        let wallet: PrivateKeySigner = match private_key.parse() {
            Ok(w) => w,
            Err(_) => {
                status.set("Invalid private key".into());
                is_loading.set(false);
                return;
            }
        };

        let provider = match ProviderBuilder::new().wallet(wallet).connect(rpc).await {
            Ok(p) => p,
            Err(_) => {
                status.set("Unable to connect to blockchain".into());
                is_loading.set(false);
                return;
            }
        };

        match FamilyTrust::deploy(&provider, unlock_time).await {
            Ok(deployer) => {
                status.set("Deploy succeeded".into());
                is_configuring.set(true);
                has_contract_signal.set(true);
                CURRENT_CONTRACT
                    .signal()
                    .set(deployer.address().to_string());
            }
            Err(e) => {
                let reason = e.as_decoded_error::<Revert>();
                let reason = reason.map(|r| r.reason).unwrap_or(e.to_string());
                status.set(format!("Deploy failed: {e:?}"));
            }
        }

        is_loading.set(false);
    };
    rsx! {
        div {
                class: "flex-1 flex border-2 rounded-2xl select-none border-white max-w-md mx-auto w-full",
                div {
                    class: "flex flex-col gap-5 mx-auto my-auto pl-2",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5  rounded-xl text-white",
                            lucide::House{size: 24, color: "white"}
                            "Back Home"
                        }
                    }
                    p {class:"text-center mx-auto text-white w-full text-wrap px-6", "No linked contract yet, Enter the period you want to lock the contract for to deploy a new contract"}
                    input { oninput: move|evt| amount_entered.set(evt.value()),
                            class: "border-2 border-white mx-auto text-center rounded p-2 text-white"
                    }
                    select {
                        class: "bg-zinc-900 text-zinc-100 border p-4 mx-auto border-zinc-700 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500",
                        value: "{mode_selected}",
                        onchange: move |evt| {
                            mode_selected.set(evt.value());
                        },
                        option { value: "seconds", "Seconds" }
                        option { value: "minutes", "Minutes" }
                        option { value: "hours", "Hours" }
                        option { value: "days", "Days" }
                    }
                    // div{
                        if !is_loading() {
                            button {disabled: !valid() || mode_selected.is_empty(),
                                class: "border-2 p-4 border-white text-center items-center justify-center text-white mx-auto rounded-2xl disabled:text-zinc-600 disabled:border-zinc-600",
                                onclick: deployment,
                                "DEPLOY CONTRACT"
                            }
                        }
                    // }
                     if is_loading.read().clone() {
                        button {
                            disabled: true,
                            class: "mx-auto flex flex-row text-center gap-2 bg-emerald-600 hover:bg-emerald-500 disabled:opacity-60 disabled:cursor-not-allowed text-white p-4 rounded-md transition-colors",
                            lucide::LoaderCircle{size: 24, color: "white",class: "animate-spin"}
                            "DEPLOYING"
                        }
                     }
                    if !amount_entered().is_empty() {
                        if let Err(_) = amount_entered().parse::<u128>() {
                            {valid.set(false)}
                            div {
                                class:"text-yellow-500 border-2 border-yellow-500 text-xs py-5 px-2 gap-2 flex mx-auto rounded-2xl",
                                lucide::TriangleAlert{
                                    class: "animate-pulse", color: "yellow"
                                }

                        p{class:"pt-1 mx-auto ", "Enter A Valid Number"}
                     }
                        }else { {valid.set(true);}}
                    } else {{valid.set(false);}}



                }
            }

    }
}

#[component]
fn ContractConfiguration() -> Element {
    let user = CURRENT_ADDRESS().clone();
    let mut updated_contract = false;

    {
        let mut guard = WALLET_MAPS.write();
        let contract_address = CURRENT_CONTRACT.read().clone();
        if let Some(account) = guard.get_mut(&user) {
            account.contract = Some(contract_address);
            updated_contract = true;
        }
    }

    if updated_contract {
        let updated = WALLET_MAPS.read().clone();
        save_json(&updated);
    }

    rsx! {
        style {
            r#"
            .check-circle {{
                stroke: #22c55e;
                stroke-width: 3;
                stroke-dasharray: 157;
                stroke-dashoffset: 157;
                animation: draw-circle 0.5s ease-out forwards;
            }}
            .check-mark {{
                stroke: #22c55e;
                stroke-width: 4;
                stroke-linecap: round;
                stroke-linejoin: round;
                stroke-dasharray: 36;
                stroke-dashoffset: 36;
                animation: draw-check 0.3s ease-out 0.5s forwards;
            }}
            @keyframes draw-circle {{ to {{ stroke-dashoffset: 0; }} }}
            @keyframes draw-check {{ to {{ stroke-dashoffset: 0; }} }}
            "#
        }

        div { class: "flex-1 flex flex-col border-2 rounded-2xl p-4 border-white gap-5 max-w-md mx-auto w-full",
            a{class: "w-max h-max",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                            lucide::House{size: 24, color: "white"}
                            "Back Home"
                        }
                    }
                }
            div { class: "flex flex-col gap-5 mx-auto my-auto p-5",

                svg {
                    class: "success-check mx-auto",
                    view_box: "0 0 52 52",
                    width: "120",
                    height: "120",
                    circle {
                        class: "check-circle",
                        cx: "26", cy: "26", r: "25",
                        fill: "none",
                    }
                    path {
                        class: "check-mark",
                        fill: "none",
                        d: "M14.1 27.2l7.1 7.2 16.7-16.8",
                    }
                }
                p { class: "text-green-600 font-semibold text-center mx-auto", "Contract Creation Sucessful" }
            }
        }
    }
}
