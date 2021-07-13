use dbus_server_address_parser::Address;
use std::string::ToString;

fn main() {
    let address_str = "unix:abstract=/tmp/dbus-U8OSdmf7";
    // Decode address
    let address = address_str.parse::<Address>().unwrap();
    // Encode address
    let address_string = address.to_string();

    println!("{}", address_string);
}
