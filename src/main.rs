mod daemon;
mod error;
mod notification;

use std::time::Duration;

use zbus::connection::Builder;

use crate::{daemon::Daemon, error::Result};

#[tokio::main]
async fn main() -> Result<()> {
    let _conn = Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", Daemon::new())?
        .build()
        .await?;

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
