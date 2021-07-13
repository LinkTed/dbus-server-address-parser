use dbus_server_address_parser::Addresses;
use honggfuzz::fuzz;
use std::str::from_utf8;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(data) = from_utf8(data) {
                if let Ok(addresses_1) = data.parse::<Addresses>() {
                    let addresses_1_str = addresses_1.to_string();
                    match addresses_1_str.parse::<Addresses>() {
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
