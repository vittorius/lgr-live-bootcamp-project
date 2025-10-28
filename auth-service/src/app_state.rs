use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore};

pub type UserStoreType<TStore> = Arc<RwLock<TStore>>;
pub type TokenStoreType<TStore> = Arc<RwLock<TStore>>;
pub type TwoFACodeStoreType<TStore> = Arc<RwLock<TStore>>;
pub type EmailClientType<TEmailClient> = Arc<TEmailClient>;

#[derive(Clone)]
pub struct AppState<
    TUserStore: UserStore,
    TTokenStore: BannedTokenStore,
    TTwoFAStore: TwoFACodeStore,
    TEmailClient: EmailClient,
> {
    pub user_store: UserStoreType<TUserStore>,
    pub banned_token_store: TokenStoreType<TTokenStore>,
    pub two_fa_code_store: TwoFACodeStoreType<TTwoFAStore>,
    pub email_client: EmailClientType<TEmailClient>,
}

impl<TUserStore, TTokenStore, TTwoFAStore, TEmailClient>
    AppState<TUserStore, TTokenStore, TTwoFAStore, TEmailClient>
where
    TUserStore: UserStore,
    TTokenStore: BannedTokenStore,
    TTwoFAStore: TwoFACodeStore,
    TEmailClient: EmailClient,
{
    pub fn new(
        user_store: UserStoreType<TUserStore>,
        banned_token_store: TokenStoreType<TTokenStore>,
        two_fa_code_store: TwoFACodeStoreType<TTwoFAStore>,
        email_client: EmailClientType<TEmailClient>,
    ) -> Self {
        Self {
            user_store,
            banned_token_store,
            two_fa_code_store,
            email_client,
        }
    }
}
