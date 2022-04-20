use serenity::{
    client::bridge::gateway::ShardMessenger, collector::CollectComponentInteraction,
    model::channel::Message,
};

pub trait MessageCollectorExt {
    /// This already filters for confirm and abort buttons only
    fn await_confirm_abort_interaction(
        &self,
        shard_messenger: & impl AsRef<ShardMessenger>,
    ) -> CollectComponentInteraction;
}

impl MessageCollectorExt for Message {
    fn await_confirm_abort_interaction(
        &self,
        shard_messenger: & impl AsRef<ShardMessenger>,
    ) -> CollectComponentInteraction {
        self.await_component_interaction(shard_messenger)
            .filter(|mci| {
                matches!(
                    mci.data.custom_id.as_ref(),
                    "_tools_button_confirm" | "_tools_button_abort"
                )
            })
    }
}
