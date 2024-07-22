//! # Hello World
//!
//! A simple smart contract for creating greetings.

// When deploying a smart contract to a blockchain, contract size is very
// important consideration. The Rust standard library is very large, so the
// following line will ensure that it is not included in the build.
#![no_std]

// From our Soroban SDK, we import the following macros:
// - contract: marks a type as being the type contract functions attach to
// - contractimpl: exports the public functions defined in the implementation
// - vec: creates a `Vec` with the given items
// We also import the following types from the SDK:
// - Env: provides access to the environment the contract is executing within
// - String: a contiguous, growable array of `u8`s used to hold characters in
//   the alphabet (a-zA-Z0-9_)
// - Vec: a sequential and indexable growable collection
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

// Defining a "unit-like" struct in the following manner allows us to easily
// implement the `HelloContract` type (and any required traits) without the need
// to define any specifics yet. The `#[contract]` attribute macro marks our
// `HelloContract` type as the type our contract functions will be attached for.
#[contract]
pub struct HelloContract;

// We use `#[contractimpl]` to export the publicly accessible functions within the
// `impl` block. Meaning those functions will be invocable by other contracts,
// or directly by Stellar transactions, once deployed.
#[contractimpl]
/// Our implementation of the `HelloContract` smart contract.
// Contract functions live inside an `impl` (implementation) for the struct we
// defined earlier. Doing so associates those functions and/or methods with the
// `HelloContract` type, and allows them to be called from somewhere else. This
// manner of constructing functions within an `impl` block also allows us to
// organize and collect all the things we can do with an instance of
// `HelloContract` into one place.
impl HelloContract {
    /// Our publicly visible function (method) is called `hello`. This function
    /// will receive a `to` argument, and return a Vec made up of `String`s made
    /// from "Hello" and the supplied `to` value.
    ///
    /// # Arguments
    ///
    /// * `env` - the environment in which the contract is running (you don't
    ///   need to supply this yourself)
    /// * `to` - who we are greeting (in this case a `String`)
    pub fn hello(env: Env, to: String) -> Vec<String> {
        // We are creating and returning a `Vec` containing two `String` items:
        // ["Hello","friend"] (actual return value depends on what is supplied
        // as the `to` argument)
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

// This declaration will look for a file named `test.rs` and will insert its
// contents inside a module named `test` under this scope.
mod test;
