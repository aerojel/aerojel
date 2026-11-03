#[derive(Debug)]
pub enum Error {
    InvalidId,
    InvalidAction(&'static str),
    ActionNotAvailable,
    ActionNotPermitted,
    // TODO
}