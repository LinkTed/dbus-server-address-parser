use crate::Guid;

/// This represents a DBus server address with the prefix [`unix:`].
///
/// [`unix:`]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-unix-domain-sockets-addresses
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unix {
    pub r#type: UnixType,
    /// The GUID of the Address.
    pub guid: Option<Guid>,
}

impl Unix {
    pub fn is_connectable(&self) -> bool {
        self.r#type.is_connectable()
    }

    pub fn is_listenable(&self) -> bool {
        self.r#type.is_listenable()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnixType {
    /// Path of the unix domain socket.
    Path(String),
    /// Directory in which a socket file with a random file name starting with `dbus-` will be
    /// created by the server.
    Dir(String),
    /// The same as `Dir`, except that on platforms with abstract sockets, the server may attempt
    /// to create an abstract socket whose name starts with this directory instead of a path-based
    /// socket.
    Tmpdir(String),
    /// Unique string in the abstract namespace, often syntactically resembling a path but
    /// unconnected to the filesystem namespace.
    Abstract(String),
    /// `XDG_RUNTIME_DIR`
    Runtime,
}

impl UnixType {
    pub fn is_connectable(&self) -> bool {
        match self {
            UnixType::Path(_) => true,
            UnixType::Dir(_) => false,
            UnixType::Tmpdir(_) => false,
            UnixType::Abstract(_) => true,
            UnixType::Runtime => false,
        }
    }

    pub fn is_listenable(&self) -> bool {
        true
    }
}
