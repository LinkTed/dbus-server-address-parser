use super::{escape::escape, guid::to_guid};
use crate::Tcp;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Tcp {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "tcp:")?;
        let mut not_first = false;
        if let Some(host) = &self.host {
            write!(f, "host={}", escape(host))?;
            not_first = true;
        }

        if let Some(bind) = &self.bind {
            if not_first {
                write!(f, ",")?;
            }
            write!(f, "bind={}", escape(bind))?;
            not_first = true;
        }

        if let Some(port) = self.port {
            if not_first {
                write!(f, ",")?;
            }
            write!(f, "port={}", port)?;
            not_first = true;
        }

        if let Some(family) = &self.family {
            if not_first {
                write!(f, ",")?;
            }
            family.fmt(f)?;
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
}
