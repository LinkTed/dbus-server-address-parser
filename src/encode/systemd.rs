use super::guid::to_guid;
use crate::Systemd;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Systemd {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "systemd:")?;
        if let Some(guid) = &self.guid {
            write!(f, "guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }
}
