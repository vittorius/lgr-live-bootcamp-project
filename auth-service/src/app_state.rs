use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, UserStore};

pub type UserStoreType<TStore> = Arc<RwLock<TStore>>;
pub type TokenStoreType<TStore> = Arc<RwLock<TStore>>;

#[derive(Clone)]
pub struct AppState<TUserStore: UserStore, TTokenStore: BannedTokenStore> {
    pub user_store: UserStoreType<TUserStore>,
    pub banned_token_store: TokenStoreType<TTokenStore>,
}

impl<TUserStore, TTokenStore> AppState<TUserStore, TTokenStore>
where
    TUserStore: UserStore,
    TTokenStore: BannedTokenStore,
{
    pub fn new(
        user_store: UserStoreType<TUserStore>,
        banned_token_store: TokenStoreType<TTokenStore>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
        }
    }
}
