#![no_std]
//! Trustless on-chain Semaphore groups: a LeanIMT Merkle tree maintained **on-chain**.
//!
//! Members are appended by the group admin, but the Merkle root is **computed on-chain** from
//! the actual members using a circomlib-compatible Poseidon (via `soroban-poseidon` over
//! CAP-75) and the @zk-kit LeanIMT rule (hash pairs, promote a lone odd node). So no one — not
//! even the admin — can publish a root that does not correspond to the real membership: the
//! root is derived, not asserted. This removes the trust assumption of the off-chain-tree model.
//!
//! Note: built against soroban-sdk 28 because the Poseidon host primitive is only reachable
//! through `soroban-poseidon`, which currently targets sdk 28. The resulting root is identical
//! to what Semaphore's `Group`/LeanIMT computes, so it interoperates with the sdk-29 verifier.

use soroban_poseidon::poseidon_hash;
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env, Vec, U256,
};

#[contractevent]
pub struct GroupCreated {
    pub group_id: u32,
    pub admin: Address,
}

#[contractevent]
pub struct MemberAdded {
    pub group_id: u32,
    pub size: u32,
    pub root: U256,
}

#[cfg(test)]
#[allow(dead_code)]
mod params;
#[cfg(test)]
mod leanimt_fixture;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    GroupNotFound = 1,
    EmptyGroup = 2,
}

#[contracttype]
pub enum DataKey {
    GroupCount,
    Admin(u32),
    Leaves(u32),
}

/// circomlib-compatible Poseidon of two field elements (LeanIMT node hash).
fn poseidon2(env: &Env, a: &U256, b: &U256) -> U256 {
    poseidon_hash::<3, soroban_sdk::crypto::bn254::Bn254Fr>(
        env,
        &Vec::from_array(env, [a.clone(), b.clone()]),
    )
}

/// LeanIMT root of `leaves`: hash adjacent pairs bottom-up; a lone odd node is promoted as-is.
fn leanimt_root(env: &Env, leaves: &Vec<U256>) -> U256 {
    let mut nodes = leaves.clone();
    while nodes.len() > 1 {
        let mut next: Vec<U256> = Vec::new(env);
        let n = nodes.len();
        let mut i = 0u32;
        while i < n {
            if i + 1 < n {
                next.push_back(poseidon2(env, &nodes.get(i).unwrap(), &nodes.get(i + 1).unwrap()));
            } else {
                next.push_back(nodes.get(i).unwrap()); // promote lone node
            }
            i += 2;
        }
        nodes = next;
    }
    nodes.get(0).unwrap()
}

#[contract]
pub struct SemaphoreGroups;

#[contractimpl]
impl SemaphoreGroups {
    /// Create a group administered by `admin`; returns the new group id.
    pub fn create_group(env: Env, admin: Address) -> u32 {
        admin.require_auth();
        let id: u32 = env.storage().instance().get(&DataKey::GroupCount).unwrap_or(0);
        env.storage().persistent().set(&DataKey::Admin(id), &admin);
        env.storage().persistent().set(&DataKey::Leaves(id), &Vec::<U256>::new(&env));
        env.storage().instance().set(&DataKey::GroupCount, &(id + 1));
        GroupCreated { group_id: id, admin }.publish(&env);
        id
    }

    /// Append a member commitment (group admin only). Returns the new member count. The new
    /// root is recomputed on-chain and emitted — it cannot be forged.
    pub fn add_member(env: Env, group_id: u32, commitment: U256) -> Result<u32, Error> {
        let admin: Address =
            env.storage().persistent().get(&DataKey::Admin(group_id)).ok_or(Error::GroupNotFound)?;
        admin.require_auth();
        let mut leaves: Vec<U256> =
            env.storage().persistent().get(&DataKey::Leaves(group_id)).ok_or(Error::GroupNotFound)?;
        leaves.push_back(commitment);
        env.storage().persistent().set(&DataKey::Leaves(group_id), &leaves);
        let root = leanimt_root(&env, &leaves);
        MemberAdded { group_id, size: leaves.len(), root }.publish(&env);
        Ok(leaves.len())
    }

    /// The group's current Merkle root (errors if the group is empty).
    pub fn root(env: Env, group_id: u32) -> Result<U256, Error> {
        let leaves: Vec<U256> =
            env.storage().persistent().get(&DataKey::Leaves(group_id)).ok_or(Error::GroupNotFound)?;
        if leaves.is_empty() {
            return Err(Error::EmptyGroup);
        }
        Ok(leanimt_root(&env, &leaves))
    }

    /// Number of members in the group.
    pub fn member_count(env: Env, group_id: u32) -> Result<u32, Error> {
        let leaves: Vec<U256> =
            env.storage().persistent().get(&DataKey::Leaves(group_id)).ok_or(Error::GroupNotFound)?;
        Ok(leaves.len())
    }
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod test;
