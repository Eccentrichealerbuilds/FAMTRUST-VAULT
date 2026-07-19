use crate::contract_calls::{ReadOutput, execute_read, execute_write, parse_address};
use crate::contract_functions::{ContractFunction, FieldKind, fields_for};
use crate::router::MyRoute;
use dioxus::prelude::*;
use dioxus_icons::lucide;

const EXPLORER_BASE: &str = "https://testnet.monadvision.com";

#[derive(Clone, PartialEq)]
enum PageState {
    Idle,
    Pending,
    WriteSuccess(String),
    ReadSuccess(String),
    ReadSuccessList(Vec<String>),
    Error(String),
}

#[component]
pub fn ContractFunctionPage(func: String) -> Element {
    let Some(function) = ContractFunction::from_slug(&func) else {
        return rsx! {
            div { class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
                div {
                    class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                    p { "FAM TRUST VAULT" }
                }
                div { class: "flex-1 flex border-2 rounded-2xl border-white max-w-md mx-auto w-full",
                    p { class: "text-white mx-auto my-auto", "Unknown function" }
                }
                div { class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                    p { class: "text-white text-center ", "Developed By: Eccentric Healer" }
                }
            }
        };
    };

    let fields = fields_for(&function);
    let mut f1 = use_signal(String::new);
    let mut f2 = use_signal(String::new);
    let mut f3 = use_signal(String::new);
    let mut state = use_signal(|| PageState::Idle);
    let mut duration_unit = use_signal(|| String::from("seconds"));

    let function_for_submit = function.clone();
    let mut use_external = use_signal(|| false);
    let mut external_contract = use_signal(String::new);
    let submit = move |_| {
        let function = function_for_submit.clone();
        async move {
            state.set(PageState::Pending);

            let contract_addr_str = if use_external() {
                external_contract.read().clone()
            } else {
                crate::app::CURRENT_CONTRACT.read().clone()
            };
            let Ok(contract_addr) = parse_address(&contract_addr_str) else {
                state.set(PageState::Error("Invalid contract address".to_string()));
                return;
            };

            let resolved_f1 = if function == ContractFunction::SetUnlockTimeByOwner {
                let amount = match f1.read().parse::<u128>() {
                    Ok(v) => v,
                    Err(_) => {
                        state.set(PageState::Error("Invalid amount".to_string()));
                        return;
                    }
                };
                let seconds = match duration_unit().as_str() {
                    "seconds" => amount,
                    "minutes" => amount * 60,
                    "hours" => amount * 60 * 60,
                    "days" => amount * 60 * 60 * 24,
                    _ => amount,
                };
                seconds.to_string()
            } else {
                f1.read().clone()
            };

            if function.is_write() {
                let wallet_addr = crate::app::CURRENT_ADDRESS.read().clone();
                let maps = crate::app::WALLET_MAPS.read();
                let Some(keypass) = maps.get(&wallet_addr) else {
                    state.set(PageState::Error("Wallet not found".to_string()));
                    return;
                };
                let private_key = keypass.private_key.clone();
                drop(maps);

                match execute_write(
                    &function,
                    contract_addr,
                    &private_key,
                    &resolved_f1,
                    &f2.read(),
                    &f3.read(),
                )
                .await
                {
                    Ok(hash) => state.set(PageState::WriteSuccess(hash)),
                    Err(reason) => state.set(PageState::Error(reason)),
                }
            } else {
                match execute_read(&function, contract_addr, &resolved_f1).await {
                    Ok(ReadOutput::Text(t)) => state.set(PageState::ReadSuccess(t)),
                    Ok(ReadOutput::List(l)) => state.set(PageState::ReadSuccessList(l)),
                    Err(reason) => state.set(PageState::Error(reason)),
                }
            }
        }
    };

    match state() {
        PageState::WriteSuccess(hash) => return rsx! { SuccessPage { tx_hash: hash } },
        PageState::Error(reason) => return rsx! { ErrorPage { reason } },
        PageState::ReadSuccess(text) => {
            return rsx! { ReadResultPage { title: function.to_string(), lines: vec![text] } };
        }
        PageState::ReadSuccessList(list) => {
            return rsx! { ReadResultPage { title: function.to_string(), lines: list } };
        }
        PageState::Idle | PageState::Pending => {}
    }
    rsx! {
        div { class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p { "FAM TRUST VAULT" }
            }

            div { class: "flex-1 flex flex-col border-2 rounded-2xl p-4 border-white gap-5 max-w-md mx-auto w-full",
                a { class: "w-max h-max",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                            lucide::House { size: 24, color: "white" }
                            "Back Home"
                        }
                    }
                }

                p { class: "text-white text-center font-semibold text-xl", "{function}" }
                if function.allows_external_contract() {
                    div { class: "flex flex-col gap-2",
                        div { class: "flex gap-2",
                            button {
                                class: if !use_external() {
                                    "flex-1 border-2 border-white bg-white text-black rounded-xl p-2 font-medium"
                                } else {
                                    "flex-1 border-2 border-white text-white rounded-xl p-2 font-medium"
                                },
                                onclick: move |_| use_external.set(false),
                                "My Contract"
                            }
                            button {
                                class: if use_external() {
                                    "flex-1 border-2 border-white bg-white text-black rounded-xl p-2 font-medium"
                                } else {
                                    "flex-1 border-2 border-white text-white rounded-xl p-2 font-medium"
                                },
                                onclick: move |_| use_external.set(true),
                                "External Contract"
                            }
                        }
                        if use_external() {
                            input {
                                class: "border-2 border-white rounded-xl p-2 bg-black text-white",
                                r#type: "text",
                                placeholder: "0x... (their FamilyTrust contract)",
                                value: "{external_contract}",
                                oninput: move |e| external_contract.set(e.value()),
                            }
                        }
                    }
                }
                div { class: "flex flex-col gap-4",
                    if fields.len() > 0 {
                        {
                            let sigs = [f1, f2, f3];
                            fields.iter().enumerate().map(move |(i, spec)| {
                                let mut sig = sigs[i];
                                match spec.kind {
                                    FieldKind::Duration => rsx! {
                                        div { class: "flex flex-col gap-1",
                                            label { class: "text-white text-sm my-auto", "{spec.label}" }
                                            div { class: "flex flex-col gap-2",
                                                input {
                                                    class: "mx-auto my-auto border-2 border-white rounded-xl p-2 bg-black text-white flex-1",
                                                    r#type: "number",
                                                    placeholder: "{spec.placeholder}",
                                                    value: "{sig}",
                                                    oninput: move |e| sig.set(e.value()),
                                                }
                                                select {
                                                    class: "mx-auto my-auto bg-zinc-900 text-zinc-100 border border-zinc-700 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-emerald-500",
                                                    value: "{duration_unit}",
                                                    oninput: move |evt| duration_unit.set(evt.value()),
                                                    option { value: "seconds", "Seconds" }
                                                    option { value: "minutes", "Minutes" }
                                                    option { value: "hours", "Hours" }
                                                    option { value: "days", "Days" }
                                                }
                                            }
                                        }
                                    },
                                    _ => {
                                        let input_type = match spec.kind {
                                            FieldKind::Uint256 => "number",
                                            _ => "text",
                                        };
                                        rsx! {
                                            div { class: "flex flex-col gap-1",
                                                label { class: "text-white text-sm", "{spec.label}" }
                                                input {
                                                    class: "border-2 border-white rounded-xl p-2 bg-black text-white",
                                                    r#type: "{input_type}",
                                                    inputmode: if spec.kind == FieldKind::EtherAmount { "decimal" } else { "text" },
                                                    placeholder: "{spec.placeholder}",
                                                    value: "{sig}",
                                                    oninput: move |e| sig.set(e.value()),
                                                }
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    } else {
                        p { class: "text-white text-center opacity-70", "This function takes no inputs" }
                    }
                }

                button {
                    class: "border-2 border-white rounded-xl p-3 text-white font-bold mt-auto disabled:opacity-50",
                    disabled: state() == PageState::Pending,
                    onclick: submit,
                    if state() == PageState::Pending { "Sending..." } else { "Call Function" }
                }
            }

            div { class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p { class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }
        }
    }
}
#[component]
fn SuccessPage(tx_hash: String) -> Element {
    let explorer_url = format!("{EXPLORER_BASE}/tx/{tx_hash}");
    rsx! {
        style {
            r#"
            .check-circle {{ stroke: #22c55e; stroke-width: 3; stroke-dasharray: 157; stroke-dashoffset: 157; animation: draw-circle 0.5s ease-out forwards; }}
            .check-mark {{ stroke: #22c55e; stroke-width: 4; stroke-linecap: round; stroke-linejoin: round; stroke-dasharray: 36; stroke-dashoffset: 36; animation: draw-check 0.3s ease-out 0.5s forwards; }}
            @keyframes draw-circle {{ to {{ stroke-dashoffset: 0; }} }}
            @keyframes draw-check {{ to {{ stroke-dashoffset: 0; }} }}
            "#
        }
        div { class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p { "FAM TRUST VAULT" }
            }

            div { class: "flex-1 flex flex-col border-2 rounded-2xl p-4 border-white gap-5 max-w-md mx-auto w-full",
                a { class: "w-max h-max",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                            lucide::House { size: 24, color: "white" }
                            "Back Home"
                        }
                    }
                }
                div { class: "flex flex-col gap-5 mx-auto my-auto p-5",
                    svg { class: "success-check mx-auto", view_box: "0 0 52 52", width: "120", height: "120",
                        circle { class: "check-circle", cx: "26", cy: "26", r: "25", fill: "none" }
                        path { class: "check-mark", fill: "none", d: "M14.1 27.2l7.1 7.2 16.7-16.8" }
                    }
                    a {
                        href: "{explorer_url}",
                        target: "_blank",
                        class: "text-green-600 font-semibold text-center mx-auto underline",
                        "Transaction Successful — View on Explorer"
                    }
                }
            }

            div { class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p { class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }
        }
    }
}
#[component]
fn ErrorPage(reason: String) -> Element {
    rsx! {
        div { class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p { "FAM TRUST VAULT" }
            }

            div { class: "flex-1 flex flex-col border-2 rounded-2xl p-4 border-white gap-5 max-w-md mx-auto w-full",
                a { class: "w-max h-max",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                            lucide::House { size: 24, color: "white" }
                            "Back Home"
                        }
                    }
                }
                div { class: "flex flex-col gap-5 mx-auto my-auto p-5",
                    lucide::TriangleAlert {
                        size: 160,
                        color: "red",
                        class: "animate-pulse font-bold mx-auto my-auto"
                    }
                    p { class: "text-red-500 font-semibold text-center mx-auto break-words", "{reason}" }
                }
            }

            div { class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p { class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }
        }
    }
}
#[component]
fn ReadResultPage(title: String, lines: Vec<String>) -> Element {
    rsx! {
        div { class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p { "FAM TRUST VAULT" }
            }

            div { class: "flex-1 flex flex-col border-2 rounded-2xl p-4 border-white gap-5 max-w-md mx-auto w-full overflow-auto",
                a { class: "w-max h-max",
                    Link {
                        draggable: false,
                        to: MyRoute::Home,
                        button {
                            class: "flex border-2 border-solid border-white gap-1 font-bold p-1.5 rounded-xl text-white",
                            lucide::House { size: 24, color: "white" }
                            "Back Home"
                        }
                    }
                }
                p { class: "text-white text-center font-semibold text-lg", "{title}" }
                div { class: "flex flex-col gap-2 overflow-auto",
                    if lines.is_empty() {
                        p { class: "text-white text-center my-auto mx-auto opacity-70", "No beneficiary is added yet" }
                    } else {
                        for line in lines {
                            p { class: "text-white text-center border border-white rounded-xl p-2", "{line}" }
                        }
                    }
                }
            }

            div { class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p { class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }
        }
    }
}
