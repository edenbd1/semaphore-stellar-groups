use crate::leanimt_fixture::{LEAVES, ROOT_AFTER};
use crate::params::TV;
use crate::{poseidon2, SemaphoreGroups, SemaphoreGroupsClient};
use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, U256};

fn u(env: &Env, b: &[u8; 32]) -> U256 {
    U256::from_be_bytes(env, &Bytes::from_array(env, b))
}

/// The on-chain Poseidon matches circomlib (shared node hash).
#[test]
fn poseidon_matches_circomlib() {
    let env = Env::default();
    for (a, b, e) in TV.iter() {
        assert_eq!(poseidon2(&env, &u(&env, a), &u(&env, b)), u(&env, e));
    }
}

/// Adding members on-chain yields exactly the @zk-kit / Semaphore LeanIMT root after each
/// insertion (odd and even sizes) — the root is computed on-chain, not trusted.
#[test]
fn onchain_root_matches_zkkit_leanimt() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(SemaphoreGroups, ());
    let client = SemaphoreGroupsClient::new(&env, &id);
    let gid = client.create_group(&admin);

    for i in 0..LEAVES.len() {
        let count = client.add_member(&gid, &u(&env, &LEAVES[i]));
        assert_eq!(count, (i as u32) + 1);
        assert_eq!(
            client.root(&gid),
            u(&env, &ROOT_AFTER[i]),
            "on-chain root must equal @zk-kit LeanIMT root after insertion {}",
            i + 1
        );
    }
    assert_eq!(client.member_count(&gid), LEAVES.len() as u32);
}
