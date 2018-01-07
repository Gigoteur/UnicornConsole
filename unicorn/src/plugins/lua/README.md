# rust-lua53 [![Build Status](https://travis-ci.org/jcmoyer/rust-lua53.svg?branch=master)](https://travis-ci.org/jcmoyer/rust-lua53) [![Documentation](https://docs.rs/lua/badge.svg)](https://docs.rs/lua)
Aims to be complete Rust bindings for Lua 5.3 and beyond. Currently, `master`
is tracking Lua `5.3.3`.

Requires a Unix-like environment. On Windows, [MSYS2](https://msys2.github.io/)
is supported.

You will need:
- wget (fetch on FreeBSD/Dragonfly, curl on MacOS)
- tar
- make
- gcc

### Using crates.io

Add this to your `Cargo.toml`:

```
[dependencies]
lua = "*"
```

### Using git

Add this to your `Cargo.toml`:

```
[dependencies.lua]
git = "https://github.com/jcmoyer/rust-lua53"
```

# Example

```rust
extern crate lua;

fn main() {
  let mut state = lua::State::new();
  state.open_libs();
  state.do_string("print('hello world!')");
}
```

# License
Licensed under the MIT License, which is the same license Lua is distributed
under. Refer to `LICENSE.md` for more information.
