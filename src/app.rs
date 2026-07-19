#[cfg(target_os = "android")]
use crate::android::key_handler::load_json;
use crate::router::MyRoute;
use dioxus::prelude::*;
use dioxus_icons::lucide;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, str::FromStr};
pub static CURRENT_ADDRESS: GlobalSignal<String> = Signal::global(|| String::new());
pub static CURRENT_CONTRACT: GlobalSignal<String> = Signal::global(|| String::new());
pub static WALLET_MAPS: GlobalSignal<BTreeMap<String, KeyPass>> =
    Signal::global(|| BTreeMap::new());
#[derive(Deserialize, Serialize, Clone)]
pub struct KeyPass {
    pub name: String,
    pub private_key: String,
    pub password: String,
    pub contract: Option<String>,
}
#[component]
pub fn Home() -> Element {
    // let mut keys = use_signal(|| HashMap::new());
    let mut file_check = use_signal(|| String::from("Checking file system"));
    let mut is_loading = use_signal(|| true);
    #[cfg(target_os = "android")]
    let instantiator = move || async move {
        // tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let keymap: Result<BTreeMap<String, KeyPass>, String> = load_json();
        if let Ok(imported_keymap) = keymap {
            if imported_keymap.is_empty() {
                file_check.set("No keys are imported yet".to_string());
            } else {
                WALLET_MAPS.signal().set(imported_keymap);
            }
        } else if let Err(_) = keymap {
            file_check.set(String::from("Unable to get file path"));
        }
        is_loading.set(false);
    };
    #[cfg(target_os = "android")]
    use_resource(move || instantiator());
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "bg-black w-full h-screen flex flex-col pt-1 py-6 px-5 gap-3 select-none",
            div {
                class: "border-solid border-2 border-white max-w-md mx-auto bg-zinc-900 w-full text-white font-extrabold rounded-t-2xl text-center py-4 font-['Jacques_Francois:Regular'] not-italic text-2xl",
                p {
                    "FAM TRUST VAULT"
                }
            } wallet_check_isLoading{is_loading }
            div{
                class: "border-2 border-white border-solid w-full max-w-md mx-auto rounded-b-3xl p-6",
                p {class: "text-white text-center ", "Developed By: Eccentric Healer" }
            }

        }
    }
}

#[component]
fn wallet_check_isLoading(is_loading: Signal<bool>) -> Element {
    let truncate_middle = |s: &str, start_len: usize, end_len: usize| -> String {
        let address_getter = alloy::signers::local::PrivateKeySigner::from_str(s)
            .unwrap()
            .address()
            .to_string();
        if address_getter.len() <= start_len + end_len {
            return address_getter.to_string();
        }
        format!(
            "{}...{}",
            &address_getter[..start_len],
            &address_getter[address_getter.len() - end_len..]
        )
    };
    rsx! {
        if *is_loading.clone().read() {
                div {
                    class: "flex-1 flex border-2 rounded-2xl select-none border-white max-w-md mx-auto w-full",
                    div { class: "flex flex-col gap-5 mx-auto my-auto",
                        lucide::LoaderCircle{size:100, color: "white",
                        class: "animate-spin font-bold mx-auto my-auto"
                        }
                        p{class: "text-white", "Checking for Imported Wallets"}
                }
            }
        }else {
            if WALLET_MAPS().is_empty(){
            div {
                    class: "flex-1 flex border-2 rounded-2xl border-white max-w-md mx-auto w-full",
                    div { class: "flex flex-col gap-5 mx-auto my-auto",
                        lucide::TriangleAlert{size:100, color: "white",
                        class: " font-bold mx-auto my-auto"
                        }
                        p{class: "text-white", "No Wallet Is Imported Yet"}
                        a {Link { to: MyRoute::ImportWallet{}, draggable: false,
                                button {
                                    class: " border-2 border-solid mx-auto text-white font-medium text-xl  rounded-xl p-4", "IMPORT WALLET"
                                }
                            }
                        }
                    }
                }
            }else if !WALLET_MAPS().is_empty() {
                p { class: "text-center mx-auto text-white font-semibold p-2" ,"Select The Account You Will Like To Use Below "
        }
                div {
                    class: "flex-1 flex flex-col overflow-auto border-2 p-4 gap-3 rounded-2xl border-white max-w-md mx-auto w-full",
                    // for _ in 0..20 {
                    for (key, value) in WALLET_MAPS().clone() {
                                button {
                                    onclick: move |_| {
                                    CURRENT_ADDRESS.signal().set(key.clone());
                                },Link{draggable:false, to: MyRoute::InteractWithContract,
                                div{
                                class: "shrink-0 flex gap-3 mx-auto p-7 px-3 border-2 border-white w-full rounded-2xl",
                                p { class:"text-white overflow-auto", "{value.name}:" }
                                p { class:"text-white overflow-hidden text-clip", "{truncate_middle(&value.private_key, 8,8)}" }
                                }
                            }
                        }}
                    // }
                }Link{ draggable: false, to: MyRoute::ImportWallet {  },
                p { class:"border-2 p-2 text-white text-center w-full mx-auto rounded-b-2xl font-semibold", "Add New Account" }
                }

            }
        }
    }
}
