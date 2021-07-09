use crate::decode::{
    AutolaunchError, GuidError, LaunchdError, NonceTcpError, SystemdError, TcpError, UnixError,
    UnixexecError,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Unix decode error: {0}")]
    UnixError(#[from] UnixError),
    #[error("TCP decode error: {0}")]
    TcpError(#[from] TcpError),
    #[error("Launchd decode error: {0}")]
    LaunchdError(#[from] LaunchdError),
    #[error("Nonce-TCP decode error: {0}")]
    NonceTcpError(#[from] NonceTcpError),
    #[error("Unixexec decode error: {0}")]
    UnixexecError(#[from] UnixexecError),
    #[error("Systemd decode error: {0}")]
    SystemdError(#[from] SystemdError),
    #[error("Autolaunch decode error: {0}")]
    AutolaunchError(#[from] AutolaunchError),
    #[error("GUID decode error: {0}")]
    GuidError(#[from] GuidError),
    #[error("Unknown type")]
    UnknownType,
}
