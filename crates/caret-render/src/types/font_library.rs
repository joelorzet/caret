#[allow(unused_imports)]
use crate::prelude::*;

/// Every font installed on the system.
pub struct FontLibrary {
    pub(crate) db: fontdb::Database,
}
