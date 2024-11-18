# `hellorustc`

This repository is a prototype for using `rustc` as a friendly compiler with R packages.

Essentially, there is an R build-system, which mainly consists of `make`, `clang`/`gcc`, and occasionally someone speaks of `autoconf`. On the other hand, Rust's build system is `cargo`. For now, people have tried to unify `cargo` with R's build system. This repository aims to join `rustc` and R's build system, instead.

Ultimately, the aim is to find a better way to work with R, Rust, `cargo`, and how to properly customize R-packages, such that they may build multiple native languages, not just single ones.

One goal here is to not use any C-files at all in this project, and simply lean on `rustc` for generating necessary C-ABI boilerplate. This is a contrived restriction, and may be abandoned later.

Currently, the [`src/Makevars`](./src/Makevars) currently builds *any* single Rust source file `.rs` as its own individual file, and links that to the R-package.

- [ ] One has to add the rust-file to the list of files in `OBJECTS` in `Makevars`. If there is a ~~kewl~~ makefile way to glob select files to do that, please let me know. For now, one also has to link the object `.o` together with the `.rs` file in the `Makevars` file. But invoking `rustc` is done for you.

This R package generates bindings for R's C-API on its own and stores it in the package itself. How to include these files is shown in [`src/hello.rs`](./src/hello.rs) and [`src/rust_add.rs`](./src/rust_add.rs).

A major pre-requisite is to have `bindgen` installed, i.e.

```sh
cargo install bindgen-cli --force
```