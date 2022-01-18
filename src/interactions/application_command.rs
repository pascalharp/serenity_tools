use serenity::{
    async_trait,
    builder::CreateEmbed,
    client::Context,
    model::{
        channel::Message,
        interactions::{
            application_command::ApplicationCommandInteraction,
            InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    Result,
};

use crate::builder::CreateEmbedExt;

#[async_trait]
pub trait ApplicationCommandInteractionExt {
    async fn create_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        kind: InteractionResponseType,
        text: T,
        ephemeral: bool,
    ) -> Result<()>;

    async fn create_quick_error<T: ToString + Send>(
        &self,
        ctx: &Context,
        kind: InteractionResponseType,
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
impl ApplicationCommandInteractionExt for ApplicationCommandInteraction {
    async fn create_quick_info<T: ToString + Send>(
        &self,
        ctx: &Context,
        kind: InteractionResponseType,
        text: T,
        ephemeral: bool,
    ) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(kind);
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
        kind: InteractionResponseType,
        text: T,
        ephemeral: bool,
    ) -> Result<()> {
        self.create_interaction_response(ctx, |r| {
            r.kind(kind);
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
