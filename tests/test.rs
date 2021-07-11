use dbus_server_address_parser::Address;
use std::convert::TryFrom;

#[test]
fn test_data() {
    let data = include_str!("test.data");
    for l in data.lines() {
        match Address::decode(l) {
            Ok(addresses_1) => {
                let addresses_1_str = Address::encode(&addresses_1);
                match Address::decode(&addresses_1_str) {
                    Ok(addresses_2) => {
                        assert_eq!(addresses_1, addresses_2);
                    }
                    Err(e) => panic!("Could not decode again: {} {} {}", e, l, addresses_1_str),
                }
            }
            Err(e) => panic!("Could not decode: {} {}", e, l),
        }
    }
}

#[test]
fn test_error_data() {
    let data = include_str!("test_error.data");
    for l in data.lines() {
        match Address::decode(l) {
            Ok(addresses) => panic!("Could decode: {} {:?}", l, addresses),
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[test]
fn test_tcp_1() {
    let address = Address::try_from("tcp:host=localhost").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[test]
fn test_tcp_2() {
    let address = Address::try_from("tcp:host=localhost,port=123").unwrap();
    assert!(address.is_listenable());
    assert!(address.is_connectable());
}

#[test]
fn test_nonce_tcp_1() {
    let address = Address::try_from("nonce-tcp:host=localhost").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[test]
fn test_nonce_tcp_2() {
    let address =
        Address::try_from("nonce-tcp:host=localhost,port=123,noncefile=/tmp/nonce").unwrap();
    assert!(address.is_listenable());
    assert!(address.is_connectable());
}

#[test]
fn test_unix_runtime() {
    let address = Address::try_from("unix:runtime=yes").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[test]
fn test_unix_tmpdir() {
    let address = Address::try_from("unix:tmpdir=/tmp").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[test]
fn test_unix_dir() {
    let address = Address::try_from("unix:dir=/tmp").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[test]
fn test_unix_abstract() {
    let address = Address::try_from("unix:abstract=/tmp/dbus-U8OSdmf7").unwrap();
    assert!(address.is_listenable());
    assert!(address.is_connectable());
}

#[test]
fn test_launchd() {
    let address = Address::try_from("launchd:env=KEY%3dVALUE").unwrap();
    assert!(address.is_listenable());
    assert!(address.is_connectable());
}

#[test]
fn test_unixexec() {
    let address = Address::try_from("unixexec:path=/bin/example,argv0=example,argv1=a").unwrap();
    assert!(!address.is_listenable());
    assert!(address.is_connectable());
}

#[test]
fn test_systemd() {
    let address = Address::try_from("systemd:").unwrap();
    assert!(address.is_listenable());
    assert!(!address.is_connectable());
}

#[cfg(target_family = "windows")]
#[test]
fn test_autolaunch() {
    let address = Address::try_from("autolaunch:").unwrap();
    assert!(address.is_listenable());
    assert!(address.is_connectable());
}

#[cfg(target_family = "unix")]
#[test]
fn test_autolaunch() {
    let address = Address::try_from("autolaunch:").unwrap();
    assert!(!address.is_listenable());
    assert!(address.is_connectable());
}
