# Magic Lantern + Rust
Gets `rustc` to emit arm assembly so we can run it with [Magic Lantern](https://www.magiclantern.fm/).

**TODO:**  
- Logic stress test to make sure code generation is correct
- Turn it into a Rust module

## Setup:
```
# Installs rust and rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv6m-none-eabi
```
