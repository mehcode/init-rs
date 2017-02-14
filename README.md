# init
> Mark a function to run before main.

## Install

```toml
[dependencies]
init = "0.1"

[build-dependencies]
init = "0.1"
```

## Usage

`src/main.rs`
```rust
#![feature(proc_macro)]

extern crate init;
use init::init;

#[init]
fn init() {
    // [...]
}
```

`build.rs`
```rust
extern crate init;

fn main() {
  init::build();
}
```

## License

Init is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
