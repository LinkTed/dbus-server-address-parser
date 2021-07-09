use crate::Guid;

/// This represents a DBus server address with the prefix [`unixexec:`].
///
/// [`unixexec:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-exec
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unixexec {
    /// Path of the binary to execute.
    pub path: String,
    /// The program name to use when executing the binary. If omitted the same value as specified
    /// for `path` will be used.
    pub argv0: Option<String>,
    /// Arguments to pass to the binary.
    pub argv: Vec<String>,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Unixexec {
    pub fn is_connectable(&self) -> bool {
        true
    }

    pub fn is_listenable(&self) -> bool {
        false
    }
}
