use std::{collections::VecDeque, sync::Arc};

#[derive(Default)]
pub struct Notifications(VecDeque<Notification>);

impl Notifications {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn add(&mut self, notification: Notification) {
        self.0.push_back(notification);
    }

    pub fn close(&mut self, id: u32) {
        self.0.retain(|n| n.id != id);
    }
}

#[derive(Debug, Default)]
pub struct Notification {
    id: u32,
    app_name: Arc<str>,
    summary: Arc<str>,
    replaces_id: u32,
    icon: Option<Arc<str>>,
    body: Option<Arc<str>>,
    actions: Vec<Arc<str>>,
    expire_timeout: i32,
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        NotificationBuilder::default()
    }
}

#[derive(Default)]
pub struct NotificationBuilder {
    id: u32,
    app_name: Arc<str>,
    summary: Arc<str>,
    expire_timeout: i32,
    body: Option<Arc<str>>,
    icon: Option<Arc<str>>,
    actions: Vec<Arc<str>>,
}

impl NotificationBuilder {
    pub fn new(id: u32, app_name: &str, summary: &str) -> Self {
        NotificationBuilder {
            id,
            app_name: Arc::from(app_name),
            summary: Arc::from(summary),
            ..Default::default()
        }
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(Arc::from(body));
        self
    }

    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(Arc::from(icon));
        self
    }

    pub fn build(self) -> Notification {
        Notification::default()
    }
}
