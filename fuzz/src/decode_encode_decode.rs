use dbus_server_address_parser::Address;
use honggfuzz::fuzz;
use std::str::from_utf8;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(data) = from_utf8(data) {
                if let Ok(addresses_1) = Address::decode(data) {
                    let addresses_1_str = Address::encode(&addresses_1);
                    match Address::decode(&addresses_1_str) {
                        Ok(addresses_2) => {
                            if addresses_1 != addresses_2 {
                                panic!(
                                    "Addresses is not equal: {:?} != {:?}: {:?}",
                                    addresses_1, addresses_2, data
                                );
                            }
                        }
                        Err(e) => {
                            panic!(
                                "Could not decode server addresses: {:?}: {:?}: {:?}",
                                e, data, addresses_1
                            );
                        }
                    }
                }
            }
        });
    }
}
