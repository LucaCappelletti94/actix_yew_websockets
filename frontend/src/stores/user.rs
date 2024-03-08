use commons::users::User;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone, Debug)]
/// The following macro will make sure that the store is saved across sessions.
// #[store(storage = "session", storage_tab_sync)]
pub struct UserState {
    pub user: Option<User>,
}

impl UserState {
    pub fn is_logged_in(&self) -> bool {
        self.user.is_some()
    }

    pub fn is_not_logged_in(&self) -> bool {
        self.user.is_none()
    }

    pub fn get_user(&self) -> Option<User> {
        self.user.clone()
    }
}
