use crate::Family;
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Family {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "family=")?;
        match self {
            Family::Ipv4 => write!(f, "ipv4"),
            Family::Ipv6 => write!(f, "ipv6"),
        }
    }
}
