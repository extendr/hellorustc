# `hellorustc`

This repository is a prototype for using `rustc` as a friendly compiler with R packages.

Essentially, there is an R build-system, which mainly consists of `make`, `clang`/`gcc`, and occasionally someone speaks of `autoconf`. On the other hand, Rust's build system is `cargo`. For now, people have tried to unify `cargo` with R's build system. This repository aims to join `rustc` and R's build system, instead.

Ultimately, the aim is to find a better way to work with R, Rust, `cargo`, and how to properly customize R-packages, such that they may build multiple native languages, not just single ones.

A major pre-requisite is to have `bindgen` installed, i.e.

```sh
cargo install bindgen-cli --force
```
