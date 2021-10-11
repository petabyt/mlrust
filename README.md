# Magic Lantern + Rust
![demo screenshot](https://petabyt.dev/filedump/Screenshot%20at%202021-10-10%2022-26-04.png)

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
