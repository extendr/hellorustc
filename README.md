# `hellorustc`
<!-- badges: start -->
[![R-CMD-check](https://github.com/CGMossa/hellorustc/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/CGMossa/hellorustc/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

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

And to use `bindgen`, one needs to install and configure `clang`. On a Mac, this is already setup.

## Learnings

This was done as a means to learn more about _what_ variables are available for us
in `Makevars`, and how they could be used to accomplish various things.

Another is to learn about the inner workings of `cargo`, through investigating what it
takes to make `rustc` produce the right artefact. I would not recommend this experience in "production".

Also, note that the way rust crates are linked here is not the usual `staticlib` way that is seen in extendr, savvy, roxido, etc.

`_rust_eh_personality` is missing. This is a sentence that comes up often. The issue is
that Rust has a `std`, with which _exception handling (`eh`)_ is implemented. This
means if you use elaborate unwinding in your `rustc`, you'd somehow have to link or
build `std`.

- [ ] Find a way to link to Rust's exception handling / unwinding. For now `-Cpanic=abort`, i.e. the C-way. This means that `extern "C"` is the only supported ABI, and that `C-unwind` cannot be supported by only calling `rustc`.
