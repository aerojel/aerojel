// #[derive(thiserror::Error, Debug)]
// pub enum StreamError {
//     #[error("")]
//     FailedToStart,
//     #[error("")]
//     FailedToFlush,
//     #[error("")]
//     FailedToPause,
// }

// #[derive(thiserror::Error, Debug)]
// pub enum PlayerError {
//     #[error("")]
//     FailedToSendCommand,
// }

// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("player error: {0}")]
//     PlayerError(#[from] PlayerError),
// }

// pub type Result<T> = std::result::Result<T, Error>;
