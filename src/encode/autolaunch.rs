#[cfg(target_family = "windows")]
use super::escape::escape;
use super::guid::to_guid;
use crate::Autolaunch;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Autolaunch {
    #[cfg(target_family = "windows")]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "autolaunch:")?;
        let mut not_first = false;
        if let Some(scope) = &self.scope {
            write!(f, "scope={}", escape(scope))?;
            not_first = true;
        }
        if let Some(guid) = &self.guid {
            if not_first {
                write!(f, ",")?;
            }
            write!(f, "guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }

    #[cfg(target_family = "unix")]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "autolaunch:")?;
        if let Some(guid) = &self.guid {
            write!(f, "guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }
}
