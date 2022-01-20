use serenity::{
    async_trait,
    client::Context,
    model::interactions::{
        message_component::MessageComponentInteraction, InteractionResponseType,
    },
    Result,
};

use crate::components::{Button, ButtonParseError};
use std::result::Result as StdResult;

#[async_trait]
pub trait MessageComponentInteractionExt {
    fn parse_button(&self) -> StdResult<Button, ButtonParseError>;

    async fn deferred_update(&self, ctx: &Context) -> Result<()>;

    async fn deferred_reply(&self, ctx: &Context) -> Result<()>;
}

#[async_trait]
impl MessageComponentInteractionExt for MessageComponentInteraction {
    fn parse_button(&self) -> StdResult<Button, ButtonParseError> {
        self.data.custom_id.parse()
    }
    async fn deferred_update(&self, ctx: &Context) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await
    }

    async fn deferred_reply(&self, ctx: &Context) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await
    }
}
