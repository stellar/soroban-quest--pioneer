// This configuration attribute instructs Cargo to only include this code when
// running tests.
#![cfg(test)]

// Since we include this `test.rs` file as a `mod` within our `lib.rs` file, the
// next line brings all of the "parent's items" (the code within `lib.rs`) into
// the current scope, and makes them available to use in our tests, even if they
// were not marked as public using `pub`.
use super::*;

// From our Soroban SDK, we import the following macros:
// - vec: creates a `Vec` with the given items
// We also import the following types from the SDK:
// - Env: provides access to the environment the contract is executing within
// - Symbol: a short (32 or fewer characters) string with a limited character
//   set [a-zA-Z0-9_]
use soroban_sdk::{vec, Env, Symbol};

// Here we add the `test` attribute to the `test()` function so Rust will know
// to build a test runner for it.
#[test]
// The name of our test function is `test()`.
fn test() {
    // We are creating an environment that will act the same as the production
    // environment our Soroban contract will run in.
    let env = Env::default();
    // We register the `HelloContract` contract within our environment, and
    // receive back the contract address
    let contract_address = env.register_contract(None, HelloContract);
    // We also construct a `HelloContractClient` that will be used to invoke the
    // contract functions. This client was created when we used the
    // `contractimpl` macro in `lib.rs`.
    let client = HelloContractClient::new(&env, &contract_address);

    // We invoke the `hello()` function from our `HelloContract`, providing the
    // short `Symbol` "Dev" as the `to` argument.
    let words = client.hello(&symbol_short!("Dev"));
    // Now, the actual test. We are asserting that the returned value from the
    // `hello()` function should be equal to a `Vec` we manually create with
    // our desired values: ["Hello","Dev"]
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}
