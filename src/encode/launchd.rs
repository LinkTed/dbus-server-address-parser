use super::{escape::escape, guid::to_guid};
use crate::Launchd;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Launchd {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "launchd:env={}", escape(&self.env))?;
        if let Some(guid) = &self.guid {
            write!(f, ",guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }
}
