use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to delete a chat photo. Photos can't be changed for private
/// chats. The bot must be an administrator in the chat for this to work and
/// must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#deletechatphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatPhoto {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
}

#[async_trait::async_trait]
impl Request for DeleteChatPhoto {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(self.bot.client(), self.bot.token(), "deleteChatPhoto", &self).await
    }
}

impl DeleteChatPhoto {
    pub(crate) fn new<C>(bot: Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
}
