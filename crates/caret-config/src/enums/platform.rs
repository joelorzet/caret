/// Which keyboard the person in front of us actually has.
///
/// Only the names differ: a binding parses the same everywhere, so a config
/// written on one machine still works on another.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Platform {
    MacOs,
    Windows,
    Other,
}
