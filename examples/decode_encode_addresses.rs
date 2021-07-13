use dbus_server_address_parser::Addresses;

fn main() {
    let addresses_str = "unix:abstract=/tmp/dbus-U8OSdmf7;tcp:host=127.0.0.1,port=30958";
    // Decode address
    let addresses = addresses_str.parse::<Addresses>().unwrap();
    // Encode address
    let addresses_string = addresses.to_string();

    println!("{}", addresses_string);
}
