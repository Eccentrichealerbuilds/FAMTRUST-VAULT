#[cfg(target_os = "android")]
use crate::android::key_handler::save_json;
use crate::app::KeyPass;
use crate::app::WALLET_MAPS;
use crate::router::MyRoute;
use alloy::signers::local::PrivateKeySigner;
use dioxus::prelude::*;
use dioxus_icons::lucide;
use std::str::FromStr;
#[derive(PartialEq)]
pub enum State {
    EnterKey,
    KeyOk,
    EnterPassWord,
}
#[component]
pub fn ImportWallet() -> Element {
    let state = use_signal(|| State::EnterKey);
    let key_input = use_signal(|| String::new());
    let has_saved = use_signal(|| false);
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div {
            class: "bg-black w-full h-screen flex flex-col py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p {
                    "FAM TRUST VAULT"
                }
            }if *state.read() == State::EnterKey{
                KeyEnter {state, key_input}
            }else if *state.read() == State::EnterPassWord{
                PasswordEnter { key_input, has_saved, state }
            }else if *state.read() == State::KeyOk {
                SuccessCheck {  }
            }
            div{
                class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p {class: "text-white text-center font-semibold", "Developed By: Eccentric Healer" }
            }

        }
    }
}
use std::clone::Clone;
use std::cmp::PartialEq;
#[component]
fn KeyEnter(state: Signal<State>, mut key_input: Signal<String>) -> Element {
    let mut has_incorrect = use_signal(|| false);
    let mut check_key = move || {
        let key_input_mod = key_input.clone();
        let key_input_mod = key_input_mod.read().clone();
        let key_input_mod = key_input_mod
            .trim()
            .strip_prefix("0x")
            .map_or(key_input_mod.as_str(), |v| v);
        let verify_originality = PrivateKeySigner::from_str(key_input_mod);
        if let Ok(_) = verify_originality {
            state.set(State::EnterPassWord);
            key_input.set(key_input_mod.to_string());
        } else if let Err(_) = verify_originality {
            has_incorrect.set(true);
        }
    };
    rsx! {
        div {
            class: "flex-1 flex border-2 rounded-2xl border-white max-w-md mx-auto w-full",
            div{ class: "flex flex-col gap-8 mx-auto my-auto px-2.5",
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
                p{class: "text-white", "Input Your Evm Private Key Below"}
                textarea {
                    value: "{key_input}",
                    oninput: move |evt| key_input.set(evt.value()),
                    class: "text-white border-3 text-wrap resize-none [&::-webkit-scrollbar]:hidden overflow-y-auto border-double font-bold rounded py-6 placeholder:text-center placeholder:text-2xl",
                    placeholder: "enter private key here",
                }
                button{
                    onclick: move |_|{
                        has_incorrect.set(false);
                        check_key()
                    },
                    class: "border-2 border-solid mx-auto text-white font-bold text-xl rounded-xl p-4",
                    "IMPORT WALLET"
                }
                if *has_incorrect.read() {
                    div {
                        class:"text-yellow-500 border-2 border-yellow-500 text-xs py-5 px-2 gap-2 flex rounded-2xl",
                        lucide::TriangleAlert{
                            class: "animate-pulse", color: "yellow"
                        }

                        p{class:"pt-1 ", "Please Enter A Valid Private Key"}
                     }
                }
            }
        }
    }
}

#[component]
fn PasswordEnter(
    key_input: Signal<String>,
    has_saved: Signal<bool>,
    state: Signal<State>,
) -> Element {
    let mut wallet_name = use_signal(|| String::new());
    let mut wallet_pass = use_signal(|| String::new());
    let mut save_account = move || {
        let address_getter =
            alloy::signers::local::PrivateKeySigner::from_str(key_input.read().as_str())
                .unwrap()
                .address()
                .to_string();
        let new_key = KeyPass {
            name: wallet_name.read().clone(),
            private_key: key_input.read().cloned(),
            password: wallet_pass.read().clone(),
            contract: None,
        };
        let mut loader = WALLET_MAPS.read().cloned();
        loader.insert(address_getter, new_key);
        #[cfg(target_os = "android")]
        save_json(&loader).unwrap();
        has_saved.set(true);
        state.set(State::KeyOk);
    };
    rsx! {
        div{
            class: "flex-1 flex border-2 rounded-2xl border-white max-w-md mx-auto w-full",
            div{ class: "flex flex-col gap-8 mx-auto my-auto px-2.5",
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
                div {
                    class: " flex-col mx-auto text-center gap-1 flex",
                    label { class: "text-white","Enter a name for this account below" }
                    input {
                    value: "{wallet_name}",

                    oninput: move |evt| wallet_name.set(evt.value()),
                    class: " text-white text-center border-3 font-bold rounded-xl py-2 placeholder:text-center",
                    placeholder: "enter wallet name here",
                }
                }
                div {
                    class: " flex-col mx-auto text-center gap-1 flex",
                    label { class: "text-white","Enter a password for this account below" }
                    input {
                    value: "{wallet_pass}",

                    oninput: move |evt| wallet_pass.set(evt.value()),
                    class: " text-white border-3 text-wrap text-center font-bold rounded-xl py-2 placeholder:text-center",
                    placeholder: "enter wallet password here",
                }
                }
                button{
                    onclick: move |_|{
                        save_account()
                    },
                    class: "border-3 border-solid mx-auto text-white font-bold text-xl rounded-xl p-4",
                    "SAVE"
                }

            }
        }
    }
}

#[component]
fn SuccessCheck() -> Element {
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
                p { class: "text-green-600 font-semibold text-center mx-auto", "Account Linking Successful" }
            }
        }
    }
}
