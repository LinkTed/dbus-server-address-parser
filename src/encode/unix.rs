use super::{escape::escape, guid::to_guid};
use crate::{Unix, UnixType};
use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Unix {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "unix:")?;
        self.r#type.fmt(f)?;
        if let Some(guid) = &self.guid {
            write!(f, ",guid={}", to_guid(guid))
        } else {
            Ok(())
        }
    }
}

impl Display for UnixType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            UnixType::Path(path) => write!(f, "path={}", escape(path)),
            UnixType::Dir(dir) => write!(f, "dir={}", escape(dir)),
            UnixType::Tmpdir(tmp_dir) => write!(f, "tmpdir={}", escape(tmp_dir)),
            UnixType::Abstract(abstract_) => write!(f, "abstract={}", escape(abstract_)),
            UnixType::Runtime => write!(f, "runtime=yes"),
        }
    }
}
