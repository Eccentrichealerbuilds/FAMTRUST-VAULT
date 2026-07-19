# FamTrust Vault 🔐

A mobile-first, onchain family trust for Monad. Lock crypto for the people who matter, with a countdown they can actually understand.

## The Problem

Most crypto inheritance planning is a private key on a sticky note and a prayer. If you disappear, your family doesn't know your assets exist — and even if they did, there's no clean way to hand off custody without trusting a centralized service or an expensive lawyer.

## The Solution

FamTrust Vault wraps a `FamilyTrust` Solidity contract in a clean mobile app:

- **Deploy your own trust** — set a lock period in seconds, minutes, hours, or days (not raw seconds, because not everyone does that math in their head).
- **Add beneficiaries** with a name and a defined share.
- **Beneficiaries self-serve** — they unlock their share and claim it independently once conditions are met, straight from their own wallet.
- **Every contract function** is exposed as a dedicated screen: inputs matched to the function's actual parameters, decoded revert reasons on failure, and a link to the transaction on the block explorer on success.
- **Local wallet management** — private keys are imported and stored locally on-device, never sent anywhere.

Built with [Dioxus](https://dioxuslabs.com/) (Rust) for the frontend and [Alloy](https://github.com/alloy-rs/alloy) for all onchain interaction, deployed on Monad Testnet.

## Tech Stack

- **Frontend:** Dioxus (Rust, cross-platform — mobile/desktop/web from one codebase)
- **Chain interaction:** Alloy (typed contract bindings, signing, RPC)
- **Contract:** Solidity, deployed to Monad Testnet
- **Icons:** dioxus-icons (Lucide)

## Contract

`FamilyTrust.sol` — deployed on Monad Testnet at: `<contract address here>`

Core mechanics:
- Owner-only beneficiary management (`addBeneficiary`, `modifyBeneficiaryShare`, `unAddBeneficiary`)
- Configurable unlock threshold and unlock period
- Self-service `unlock()` / `transfer()` for beneficiaries — no owner action needed at claim time
- Owner can `withdraw()` remaining funds before distribution

## Running Locally

```bash
git clone <repo-url>
cd famtrust-vault
dx serve --android --target aarch64-linux-android --device
```

## Security

Private keys never leave the device and never touch a remote server. FamTrust Vault uses Android's `getFilesDir()` — the app's private internal storage directory — to persist imported wallet keys as a local JSON file. This directory is sandboxed by Android per-app: it's not accessible to other apps, not exposed on external/shared storage, and gets wiped automatically on uninstall. Keys are read/written directly through JNI calls into the Android activity, so there's no cloud sync, no third-party key management service, and no attack surface beyond the device itself.


## What's Next

This was built in a single hackathon sprint, so a few things are intentionally scoped out for now:

- **Multi-asset support** — right now the trust only holds native MON. Next up: ERC-20 tokens and NFTs, so a full portfolio (not just gas token) can be inherited.
- **Local password protection** — an optional local passphrase to gate access to imported wallet keys on-device, separate from the wallet's own private key, for an extra layer of protection if a device is lost or shared.
- **External contract lookup** — letting a beneficiary connect to *any* trust contract they've been added to, not just one tied to their own wallet's stored deployment (early version of this is already partially wired in).
- **Ownership transfer** — a way to reassign the `owner` role to a new address, so a compromised or lost owner wallet doesn't permanently lock the trust's management functions.

## Team

Developed by: Eccentric Healer

