//! Limitlxxs Crystal Optimizer. All the logic lives in the `crystal_pop` mixin
//! (see `horizon-mod.toml` and `src/mixins/crystal_pop.rs`), so there are no hooks here.
use mod_api::{self, Hooks};

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl Hooks for Mod {}
