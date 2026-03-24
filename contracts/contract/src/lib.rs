#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address};

#[derive(Clone)]
#[contracttype]
pub struct RentalAgreement {
    pub id: u64,
    pub landlord: Address,
    pub tenant: Address,
    pub rent_amount: i128,
    pub duration: u64,
    pub is_active: bool,
    pub is_accepted: bool,
}

#[contract]
pub struct RentalManager;

#[contractimpl]
impl RentalManager {

    // Create agreement with unique ID
    pub fn create_agreement(
        env: Env,
        landlord: Address,
        tenant: Address,
        rent_amount: i128,
        duration: u64,
    ) -> u64 {
        landlord.require_auth();

        if rent_amount <= 0 {
            panic!("Invalid rent amount");
        }

        if landlord == tenant {
            panic!("Landlord and tenant cannot be same");
        }

        // Get current ID counter
        let counter_key = Symbol::new(&env, "COUNTER");
        let mut id: u64 = env.storage().instance().get(&counter_key).unwrap_or(0);

        id += 1;

        let agreement = RentalAgreement {
            id,
            landlord: landlord.clone(),
            tenant: tenant.clone(),
            rent_amount,
            duration,
            is_active: true,
            is_accepted: false,
        };

        let key = (Symbol::new(&env, "AGREEMENT"), id);
        env.storage().instance().set(&key, &agreement);

        env.storage().instance().set(&counter_key, &id);

        id
    }

    // Get agreement by ID
    pub fn get_agreement(env: Env, id: u64) -> RentalAgreement {
        let key = (Symbol::new(&env, "AGREEMENT"), id);
        env.storage().instance().get(&key).unwrap()
    }

    // Tenant accepts agreement
    pub fn accept_agreement(env: Env, id: u64, tenant: Address) {
        tenant.require_auth();

        let key = (Symbol::new(&env, "AGREEMENT"), id);
        let mut agreement: RentalAgreement =
            env.storage().instance().get(&key).unwrap();

        if agreement.tenant != tenant {
            panic!("Not authorized tenant");
        }

        agreement.is_accepted = true;

        env.storage().instance().set(&key, &agreement);
    }

    // Terminate agreement
    pub fn terminate_agreement(env: Env, id: u64, landlord: Address) {
        landlord.require_auth();

        let key = (Symbol::new(&env, "AGREEMENT"), id);
        let mut agreement: RentalAgreement =
            env.storage().instance().get(&key).unwrap();

        if agreement.landlord != landlord {
            panic!("Not authorized landlord");
        }

        agreement.is_active = false;

        env.storage().instance().set(&key, &agreement);
    }
}