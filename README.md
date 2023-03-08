# Peroxide with BLAS Example Repo

## Rust `blas` ecosystem

Peroxide uses `blas` and `lapack` crate in `O3` feature. But they are just wrapper crates, so we need source crates.

There are several source crates to provide concrete implementations of BLAS and LAPACK.

* [`accelerate-src`](https://github.com/blas-lapack-rs/accelerate-src)
* [`blis-src`](https://github.com/blas-lapack-rs/blis-src)
* [`intel-mkl-src`](https://github.com/termoshtt/rust-intel-mkl)
* [`netlib-src`](https://github.com/blas-lapack-rs/netlib-src)
* [`openblas-src`](https://github.com/blas-lapack-rs/openblas-src)

And they have `system` feature to skip building the bundled BLAS implementation.

## For Arch-linux

For specifically, I'll explain Arch-linux case.
In arch linux, you can install `openblas` or `blis`. (Also `blas` is there, but `blas-src` does not provide `system` feature.)

* Use OpenBLAS
    : Add `openblas-src` dependency to `Cargo.toml` as same as this repo's [Cargo.toml](./Cargo.toml)
* Use BLIS
    : Add `blis-src` dependency to `Cargo.toml` as same as this repo's [Cargo.toml](./Cargo.toml)

And add `extern crate <CRATE>` to your `lib.rs` or `main.rs`. The `<CRATE>` is one of above source crates.
In this repo, I used `blis-src`.

## References

* [blas-lapack-rs](https://github.com/blas-lapack-rs/blas-lapack-rs.github.io/wiki)
* [ndarray](https://github.com/rust-ndarray/ndarray#how-to-enable-blas-integration)

