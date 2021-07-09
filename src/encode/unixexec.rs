use super::{escape::escape, guid::to_guid};
use crate::Unixexec;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Unixexec {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unixexec:path={}", escape(&self.path))?;

        if let Some(argv0) = &self.argv0 {
            write!(f, ",argv0={}", escape(argv0))?;
        }

        for (i, arg) in self.argv.iter().enumerate() {
            write!(f, ",argv{}={}", i + 1, escape(arg))?;
        }

        if let Some(guid) = &self.guid {
            write!(f, ",guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }
}
