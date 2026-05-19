use thiserror::Error;

pub type Result<T> = core::result::Result<T, DaemonError>;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("D-Bus error: {0}")]
    Dbus(#[from] zbus::Error),
}
