use dbus_server_address_parser::Address;

fn main() {
    let addresses_str = "unix:abstract=/tmp/dbus-U8OSdmf7;tcp:host=127.0.0.1,port=30958";
    // Decode address
    let addresses = Address::decode(addresses_str).unwrap();
    // Encode address
    let addresses_string = Address::encode(&addresses);

    println!("{}", addresses_string);
}
