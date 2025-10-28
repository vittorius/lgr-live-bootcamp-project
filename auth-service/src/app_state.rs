use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, TwoFACodeStore, UserStore};

pub type UserStoreType<TStore> = Arc<RwLock<TStore>>;
pub type TokenStoreType<TStore> = Arc<RwLock<TStore>>;
pub type TwoFACodeStoreType<TStore> = Arc<RwLock<TStore>>;
    
#[derive(Clone)]
pub struct AppState<
    TUserStore: UserStore,
    TTokenStore: BannedTokenStore,
    TTwoFAStore: TwoFACodeStore,
> {
    pub user_store: UserStoreType<TUserStore>,
    pub banned_token_store: TokenStoreType<TTokenStore>,
    pub two_fa_code_store: TwoFACodeStoreType<TTwoFAStore>,
}

impl<TUserStore, TTokenStore, TTwoFAStore> AppState<TUserStore, TTokenStore, TTwoFAStore>
where
    TUserStore: UserStore,
    TTokenStore: BannedTokenStore,
    TTwoFAStore: TwoFACodeStore,
{
    pub fn new(
        user_store: UserStoreType<TUserStore>,
        banned_token_store: TokenStoreType<TTokenStore>,
        two_fa_code_store: TwoFACodeStoreType<TTwoFAStore>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
        }
    }
}
