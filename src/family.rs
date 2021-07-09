/// This represents a socket family for [`tcp:`] and [`nonce-tcp:`].
///
/// [`tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-tcp-sockets
/// [`nonce-tcp:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-nonce-tcp-sockets
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Family {
    Ipv4,
    Ipv6,
}
