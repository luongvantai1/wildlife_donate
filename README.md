# wildlife_donate

## Project Title
wildlife_donate

## Project Description
Wildlife conservation efforts depend on transparent, trustworthy fundraising. Today most donations to small wildlife charities are tracked in spreadsheets, PDFs, and social media posts, leaving donors with no verifiable record of how much a program raised, who ran it, or what was actually delivered in the field. `wildlife_donate` is a Soroban smart contract on Stellar that gives every conservation program a public, tamper-proof on-chain footprint. A charity registers a program with a name hash and a funding target, donors contribute and have their contributions counted, the charity posts milestone updates with hashes pointing to off-chain field reports, and the program is finally closed with an outcome hash. No real XLM or asset is moved on-chain in this version of the contract — it is a fully transparent ledger of commitments, contributions, and progress for wildlife conservation.

## Project Vision
The long-term vision of `wildlife_donate` is to become the default trust layer for community-driven wildlife conservation. By anchoring program identity, donor counts, and milestone proof hashes on Stellar, the project aims to make it impossible for fraudulent "ghost charities" to raise funds in the name of endangered species without leaving a trace. A future iteration will pair the on-chain registry with real on-chain payouts in stablecoins, a frontend that lets any visitor browse the live status of a program, and integrations with on-chain identity to verify that a registered charity is the entity it claims to be.

## Key Features
- **Program Registration** — A charity registers a conservation program with a unique `program_id`, a `name_hash` for the off-chain description, and a `target_amount` funding goal.
- **Transparent Donations** — Donors call `donate` to contribute; each donation increases the program's `total_raised` and the donor's personal contribution total, both stored on-chain.
- **Milestone Updates** — The registered charity periodically calls `post_update` to publish a hash of the latest off-chain field report, building a verifiable timeline of progress.
- **Program Closure With Outcome** — When a program ends, the charity calls `close_program` to freeze donations and record a final `outcome_hash`.
- **Public Read-Only Queries** — Anyone can call `get_total_raised` and `is_active` to see how much a program has raised and whether it is still accepting donations, with no wallet connection required.
- **Authorization Everywhere** — Every state-changing function uses Stellar `require_auth`, so only the correct charity can register/update/close a program and only the correct donor can record a contribution.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** environment dApp — see `contracts/wildlife_donate/src/lib.rs` for the full wildlife_donate business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** CDINRZBNQ5BB25NI5YARUMEJ44W6YLYXPZ34CCXBGK5F22URJJZQAUXA
- **Explorer template:** https://stellar.expert/explorer/testnet/tx/43b9dd2a25173dac56b7d8b7498a09c9d7388d28fba3943038b4f24006096952
- **Screenshot of deployed contract on Stellar Expert:**
![screenshot](https://ibb.co/Dg4yp2HB)


## Future Scope
- **Real Asset Payouts** — Wire the contract into Stellar's native asset or a stablecoin (e.g. USDC on Stellar) so donations actually settle on-chain, with the contract holding funds until milestones are reached.
- **Donor Refund Flow** — Allow donors to claim a refund on a closed program that failed to meet its target, governed by a configurable refund policy stored on the program record.
- **Multi-Charity Co-Run Programs** — Let two or more verified charities co-register a program, each with their own update key, to support joint cross-border conservation efforts.
- **Frontend dApp** — Build a React/Next.js UI that lists active programs, shows live totals, lets donors connect Freighter and contribute, and surfaces milestone updates in a feed.
- **On-Chain Identity Hooks** — Integrate with Stellar's on-chain identity framework to verify that a registered charity address is bound to a real-world conservation NGO before its program goes live.
- **Off-Chain Storage Anchoring** — Pair update and outcome hashes with content stored on IPFS or a similar decentralized store, so the hashes published on-chain can be resolved to the full report by anyone.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `wildlife_donate` (environment)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
