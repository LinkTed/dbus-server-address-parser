use dbus_server_address_parser::Address;
use std::{convert::TryFrom, string::ToString};

fn main() {
    let address_str = "unix:abstract=/tmp/dbus-U8OSdmf7";
    // Decode address
    let address = Address::try_from(address_str).unwrap();
    // Encode address
    let address_string = address.to_string();

    println!("{}", address_string);
}
