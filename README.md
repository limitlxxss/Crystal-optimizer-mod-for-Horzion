# Limitlxxs Crystal Optimizer

Client-side native mod for [Horizon](https://playhorizon.dev) (Minecraft 26.3).
When you hit an end crystal, it is removed on your screen right away instead of
waiting for the server to confirm. The attack packet is still sent normally, and
the server's own removal arrives later and is simply ignored.

## Install

Easiest: install it from the launcher's **Mods** page (once it is listed in the
registry), then enable it there.

By hand: put the `.hmod` file in `horizon-native-mods/` inside your game folder
(the one with `options.txt`), then add this line to `horizon-mods.txt`:

```
mod.crystal_optimizer:true
```

The launcher builds native mods for your instance, so you don't need Rust installed.

## Build from source

You need `cargo-horizon-mod` and the Horizon SDK (the `.horizon/sdk` path that
`Cargo.toml` points to is fetched for you).

```
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/crystal_optimizer.hmod
```

## Layout

- `horizon-mod.toml`: mod metadata and the `crystal_pop` mixin definition
- `src/mixins/crystal_pop.rs`: the mixin code
- `src/lib.rs`: empty mod entry point

## License

MIT, see `LICENSE`.
