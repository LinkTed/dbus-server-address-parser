# dbus-server-address-parser

A library to encode and decode [DBus server address](https://dbus.freedesktop.org/doc/dbus-specification.html#addresses).
[![Build status](https://github.com/LinkTed/dbus-server-address-parser/workflows/Continuous%20Integration/badge.svg)](https://github.com/LinkTed/dbus-server-address-parser/actions?query=workflow%3A%22Continuous+Integration%22)
[![Dependency status](https://deps.rs/repo/github/linkted/dbus-server-address-parser/status.svg)](https://deps.rs/repo/github/linkted/dbus-server-address-parser)
[![Code coverage](https://codecov.io/gh/LinkTed/dbus-server-address-parser/branch/master/graph/badge.svg)](https://codecov.io/gh/LinkTed/dbus-server-address-parser)
[![Latest version](https://img.shields.io/crates/v/dbus-server-address-parser.svg)](https://crates.io/crates/dbus-server-address-parser)
[![License](https://img.shields.io/crates/l/dbus-server-address-parser.svg)](https://opensource.org/licenses/BSD-3-Clause)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
dbus-server-address-parser = "~0.0.0"
```

## Example

```rust
use dbus_server_address_parser::Address;

fn main() {
    let addresses_str = "unix:abstract=/tmp/dbus-U8OSdmf7;tcp:host=127.0.0.1,port=30958";
    // Decode address
    let addresses = Address::decode(addresses_str).unwrap();
    // Encode address
    let addresses_string = Address::encode(&addresses);

    println!("{}", addresses_string);
}
```

