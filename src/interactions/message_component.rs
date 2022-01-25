use serenity::{
    async_trait,
    client::Context,
    model::{interactions::{
        message_component::MessageComponentInteraction, InteractionResponseType, InteractionApplicationCommandCallbackDataFlags,
    }, channel::Message},
    Result, builder::CreateEmbed,
};

use crate::{components::{Button, ButtonParseError}, builder::CreateEmbedExt};
use std::result::Result as StdResult;

#[async_trait]
pub trait MessageComponentInteractionExt {
    fn parse_button(&self) -> StdResult<Button, ButtonParseError>;

    async fn deferred_update(&self, ctx: &Context) -> Result<()>;

    async fn deferred_reply(&self, ctx: &Context) -> Result<()>;

    async fn create_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<()>;

    async fn create_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<()>;

    async fn edit_quick_info<T: ToString + Send>(&self, ctx: &Context, text: T) -> Result<Message>;

    async fn edit_quick_error<T: ToString + Send>(&self, ctx: &Context, text: T)
        -> Result<Message>;

    async fn create_followup_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<Message>;

    async fn create_followup_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<Message>;

    async fn edit_followup_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        text: T,
    ) -> Result<Message>;

    async fn edit_followup_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        text: T,
    ) -> Result<Message>;
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

    async fn create_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource);
            r.interaction_response_data(|d| {
                if ephemeral {
                    d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                }
                d.add_embed(CreateEmbed::info_box(text))
            })
        })
        .await
    }

    async fn create_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource);
            r.interaction_response_data(|d| {
                if ephemeral {
                    d.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                }
                d.add_embed(CreateEmbed::error_box(text))
            })
        })
        .await
    }

    async fn edit_quick_info<T: ToString + Send>(&self, ctx: &Context, text: T) -> Result<Message> {
        self.edit_original_interaction_response(ctx, |d| {
            d.content("");
            d.set_embeds(Vec::new());
            d.components(|c| c);
            d.add_embed(CreateEmbed::info_box(text))
        })
        .await
    }

    async fn edit_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
    ) -> Result<Message> {
        self.edit_original_interaction_response(ctx, |d| {
            d.content("");
            d.set_embeds(Vec::new());
            d.components(|c| c);
            d.add_embed(CreateEmbed::error_box(text))
        })
        .await
    }

    async fn create_followup_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<Message> {
        self.create_followup_message(ctx, |m| {
            if ephemeral {
                m.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
            }
            m.add_embed(CreateEmbed::info_box(text))
        })
        .await
    }

    async fn create_followup_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        text: T,
        ephemeral: bool,
    ) -> Result<Message> {
        self.create_followup_message(ctx, |m| {
            if ephemeral {
                m.flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
            }
            m.add_embed(CreateEmbed::error_box(text))
        })
        .await
    }

    async fn edit_followup_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        text: T,
    ) -> Result<Message> {
        self.edit_followup_message(ctx, msg, |m| m.add_embed(CreateEmbed::info_box(text)))
            .await
    }

    async fn edit_followup_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        msg: &Message,
        text: T,
    ) -> Result<Message> {
        self.edit_followup_message(ctx, msg, |m| m.add_embed(CreateEmbed::error_box(text)))
            .await
    }
}
