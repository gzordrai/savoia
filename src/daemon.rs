use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

use tokio::sync::Mutex;
use zbus::{interface, zvariant::OwnedValue};

use crate::notification::{Notification, Notifications};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const SPEC_VERSION: &str = "1.2";

pub struct Daemon {
    next_id: AtomicU32,
    notifications: Mutex<Notifications>,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU32::new(1),
            notifications: Mutex::new(Notifications::new()),
        }
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl Daemon {
    /// Returns notification server capabilities.
    ///
    /// See [GetCapabilities spec](https://specifications.freedesktop.org/notification/latest/protocol.html#command-get-capabilities)
    async fn get_capabilities(&self) -> [&str; 1] {
        ["body"]
    }

    /// Receive a notification through D-Bus from an application.
    ///
    /// See [Notify spec](https://specifications.freedesktop.org/notification/latest/protocol.html#command-notify)
    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<String>,
        hints: HashMap<String, OwnedValue>,
        expire_timeout: i32,
    ) -> u32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let notification = Notification::builder().build();

        println!("New notification {id}");

        self.notifications.lock().await.add(notification);

        id
    }

    /// Close the requested notification by its id.
    ///
    /// See [CloseNotification spec](https://specifications.freedesktop.org/notification/latest/protocol.html#command-close-notification)
    async fn close_notification(&self, id: u32) {
        todo!()
    }

    /// Returns notification server information.
    ///
    /// See [GetServerInformation spec](https://specifications.freedesktop.org/notification/latest/protocol.html#command-get-server-information)
    async fn get_server_information(&self) -> (&str, &str, &str, &str) {
        (NAME, AUTHORS, VERSION, SPEC_VERSION)
    }
}
