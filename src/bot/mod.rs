mod api;
mod download;
mod token;

use crate::dispatching::dispatcher::Dispatcher;
use crate::bot::token::Token;

/// A Telegram bot used to send requests.
pub struct Bot {
    token: Token,
    dispatcher: Option<Dispatcher>,
    storage: Option<Stor>,
}

impl Bot {
    pub fn new<S>(convertable_to_token: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            token: Token::from(convertable_to_token),
            dispatcher: None,
            storage: None,
        }
    }
}

impl Bot {
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn set_dispather(&mut self, dispatcher: Dispatcher) {
        self.dispatcher = dispatcher
    }
    pub fn set_storage(&mut self, storage: Stor) {
        self.storage = storage
    }
}

