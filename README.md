# rnpx
A faster way to run package.json scripts for pnpm/npm/yarn-classic projects.

Are you tired of pnpm taking 500ms to startup biome which then takes under 50ms? So was I, so I made this thing, thats basically it.

## Limitations because I wrote this in an hour
- Workspaces are not supported (currently)
- yarn pnp is not supported (might be annoying so very maybe on this)
- any fancy flags from npm/pnpm/yarn script running that you know and love are not here


## Installation
Do you use arch? Congratulations, use the provided `./arch-prepare.sh` and `./arch-pkg/PKGBUILD` to make your own package from a tagged commit, or use the released tarballs

Do you not use arch? Install rnpx from the crates.io registry using `cargo` (in turn packaged by (rustup)[https://rustup.rs]):
```
cargo install rnpx --locked
```
