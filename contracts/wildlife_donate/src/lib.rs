#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

/// The `WildlifeDonate` contract is an on-chain registry and donation tracker
/// for wildlife conservation programs. Charities register programs with a
/// funding target, donors contribute to those programs, and the registered
/// charity periodically posts milestone updates before closing the program
/// with a final outcome. All amounts are tracked in integer units; no real
/// XLM or other asset is moved on-chain by this contract.
#[contract]
pub struct WildlifeDonate;

/// On-chain representation of a single wildlife conservation program.
#[contracttype]
#[derive(Clone)]
pub struct Program {
    /// Address of the charity that registered and controls the program.
    pub charity: Address,
    /// Short hash identifying the human-readable program name / metadata.
    pub name_hash: u64,
    /// Target funding amount (in contract-defined units) for the program.
    pub target_amount: u32,
    /// Total amount raised so far across every donor contribution.
    pub total_raised: u32,
    /// Whether the program is still open and accepting donations.
    pub active: bool,
    /// Number of milestone updates the charity has posted for the program.
    pub updates_count: u32,
}

#[contractimpl]
impl WildlifeDonate {
    /// Register a new wildlife conservation program on-chain.
    ///
    /// The caller (`charity`) becomes the program owner and must authorize
    /// the call. `program_id` must be unique; `name_hash` is an opaque hash
    /// referencing off-chain program metadata (name, description, etc.);
    /// `target_amount` is the funding goal expressed in contract units.
    pub fn register_program(
        env: Env,
        charity: Address,
        program_id: Symbol,
        name_hash: u64,
        target_amount: u32,
    ) {
        // Charity must authorize the registration.
        charity.require_auth();

        if target_amount == 0 {
            panic!("Target amount must be greater than zero");
        }

        let key = (Symbol::new(&env, "program"), program_id.clone());
        if env.storage().instance().has(&key) {
            panic!("Program already exists");
        }

        let program = Program {
            charity: charity.clone(),
            name_hash,
            target_amount,
            total_raised: 0,
            active: true,
            updates_count: 0,
        };

        env.storage().instance().set(&key, &program);

        // Index the program under the registering charity for lookup.
        let owner_key = (Symbol::new(&env, "owner"), charity, program_id);
        env.storage().instance().set(&owner_key, &true);
    }

    /// Contribute `amount` units to an active conservation `program_id`.
    ///
    /// The `donor` must authorize the call. Returns the new total amount
    /// raised for the program after this donation is applied. The donor's
    /// running contribution total for the program is also recorded.
    pub fn donate(
        env: Env,
        donor: Address,
        program_id: Symbol,
        amount: u32,
    ) -> u32 {
        // Donor must authorize the contribution.
        donor.require_auth();

        if amount == 0 {
            panic!("Donation must be greater than zero");
        }

        let key = (Symbol::new(&env, "program"), program_id.clone());
        let mut program: Program = env
            .storage()
            .instance()
            .get(&key)
            .expect("Program not found");

        if !program.active {
            panic!("Program is not active");
        }

        program.total_raised += amount;
        env.storage().instance().set(&key, &program);

        // Track this donor's running contribution to the program.
        let donor_key = (Symbol::new(&env, "donor"), program_id, donor);
        let current: u32 = env
            .storage()
            .instance()
            .get(&donor_key)
            .unwrap_or(0u32);
        env.storage().instance().set(&donor_key, &(current + amount));

        program.total_raised
    }

    /// Post a milestone update for a conservation program.
    ///
    /// Only the charity that originally registered the program may post
    /// updates. `update_hash` is an opaque hash referencing the off-chain
    /// update payload (e.g. photo report, field notes). Returns the new
    /// milestone count for the program.
    pub fn post_update(
        env: Env,
        charity: Address,
        program_id: Symbol,
        update_hash: u64,
    ) -> u32 {
        // Charity must authorize the update.
        charity.require_auth();

        let key = (Symbol::new(&env, "program"), program_id.clone());
        let mut program: Program = env
            .storage()
            .instance()
            .get(&key)
            .expect("Program not found");

        if program.charity != charity {
            panic!("Only the registered charity can post updates");
        }

        if !program.active {
            panic!("Program is not active");
        }

        program.updates_count += 1;
        env.storage().instance().set(&key, &program);

        // Store the hash for this specific milestone number.
        let update_key = (
            Symbol::new(&env, "update"),
            program_id,
            program.updates_count,
        );
        env.storage().instance().set(&update_key, &update_hash);

        program.updates_count
    }

    /// Close a conservation program and record its final outcome.
    ///
    /// Only the charity that registered the program may close it. After
    /// closure the program no longer accepts donations. `outcome_hash`
    /// references the off-chain final report.
    pub fn close_program(
        env: Env,
        charity: Address,
        program_id: Symbol,
        outcome_hash: u64,
    ) {
        // Charity must authorize the closure.
        charity.require_auth();

        let key = (Symbol::new(&env, "program"), program_id.clone());
        let mut program: Program = env
            .storage()
            .instance()
            .get(&key)
            .expect("Program not found");

        if program.charity != charity {
            panic!("Only the registered charity can close this program");
        }

        program.active = false;
        env.storage().instance().set(&key, &program);

        // Persist the final outcome hash for the program.
        let outcome_key = (Symbol::new(&env, "outcome"), program_id);
        env.storage().instance().set(&outcome_key, &outcome_hash);
    }

    /// Get the total amount raised for a conservation program.
    ///
    /// Returns the running `total_raised` value stored on the program
    /// record. Panics if the program has not been registered.
    pub fn get_total_raised(env: Env, program_id: Symbol) -> u32 {
        let key = (Symbol::new(&env, "program"), program_id);
        let program: Program = env
            .storage()
            .instance()
            .get(&key)
            .expect("Program not found");
        program.total_raised
    }

    /// Check whether a conservation program is still active.
    ///
    /// Returns `true` if the program exists and is still open for
    /// donations, `false` if it has been closed or does not exist.
    pub fn is_active(env: Env, program_id: Symbol) -> bool {
        let key = (Symbol::new(&env, "program"), program_id);
        match env.storage().instance().get::<_, Program>(&key) {
            Some(p) => p.active,
            None => false,
        }
    }
}
