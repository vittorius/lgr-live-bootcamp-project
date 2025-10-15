use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::UserStore;

pub type UserStoreType<TStore> = Arc<RwLock<TStore>>;

#[derive(Clone)]
pub struct AppState<TStore: UserStore> {
    pub user_store: UserStoreType<TStore>,
}

impl<TStore> AppState<TStore>
where
    TStore: UserStore,
{
    pub fn new(user_store: UserStoreType<TStore>) -> Self {
        Self { user_store }
    }
}
