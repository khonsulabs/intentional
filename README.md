# intentional

A crate for intentionally performing "questionable" operations.

## Why?

[Clippy][clippy] is an amazing tool, so much so that this crate's creator uses
`#[warn(clippy::pedantic)]` on nearly all of his projects. There are a few lints
that are enabled by Pedantic that in some situations have no way to work around
except to use the operation being warned against.

This crate provides APIs that are ways to perform the warned operations
explicitly. Take for example converting an `f32` to a `u32`. This triggers two
lints enabled in pedantic:

- [`cast_possible_truncation`](https://rust-lang.github.io/rust-clippy/master/index.html#/cast_possible_truncation)
- [`cast_sign_loss`](https://rust-lang.github.io/rust-clippy/master/index.html#/cast_sign_loss)

The problem with converting between `f32` and `u32` is that there is no way to
disable these lints on a per-expression basis, which leads to disabling the
warning across a wider range of code than necessary. This may lead to future
bugs when code is refactored and the `as` usage is no longer correct.

This crate attempts to offer ways for the developer to be explicit about their
intentions and minimize the number of disabled clippy lints.

[clippy]: https://github.com/rust-lang/rust-clippy
