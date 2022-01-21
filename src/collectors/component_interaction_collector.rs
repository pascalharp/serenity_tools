use std::{collections::{HashSet, HashMap}, time::Duration, hash::Hash, fmt::Display};

use serenity::{
    client::{bridge::gateway::ShardMessenger, Context}, collector::CollectComponentInteraction,
    model::{channel::{Message, ReactionType}, interactions::message_component::ButtonStyle}, async_trait, builder::{CreateButton, CreateEmbed, CreateActionRow},
    Result as SerenityResult,
};

use crate::builder::CreateActionRowExt;

pub trait MessageCollectorExt {
    /// This already filters for confirm and abort buttons only
    fn await_confirm_abort_interaction<'a>(
        &self,
        shard_messenger: &'a impl AsRef<ShardMessenger>,
    ) -> CollectComponentInteraction<'a>;
}

impl MessageCollectorExt for Message {
    fn await_confirm_abort_interaction<'a>(
        &self,
        shard_messenger: &'a impl AsRef<ShardMessenger>,
    ) -> CollectComponentInteraction<'a> {
        self.await_component_interaction(shard_messenger)
            .filter(|mci| {
                matches!(
                    mci.data.custom_id.as_ref(),
                    "_tools_button_confirm" | "_tools_button_abort"
                )
            })
    }
}

/// High abstractions
#[async_trait]
pub trait MessagePagerExt {
    async fn paged_selector<T, F>(
        &mut self,
        ctx: &Context,
        values: &[T],
        title: String,
        timeout: Duration,
        button: F
    ) -> SerenityResult<Option<HashSet<&T>>>
    where
        T: Display + Eq + Hash + Send + Sync,
        F: Fn(&T) -> (ReactionType, String) + Send + Sync;
}

fn paged_selector_embed<T: Display + Eq + Hash>(
    mut emb: CreateEmbed,
    values: &[T],
    selected: &HashSet<&T>,
    ) -> CreateEmbed {
    let role_fields = values.chunks(5 * 4);
    for (i, e) in role_fields.enumerate() {
        emb.field(
            format!("Page {}", i+1),
            e.iter().map(|t| {
                    format!(
                        "{} | {}",
                        if selected.contains(t) { "✅" } else { "⬛" },
                        t.to_string())
            }).collect::<Vec<_>>().join("\n"),
            true);
    }
     emb
}

#[async_trait]
impl MessagePagerExt for Message {
    async fn paged_selector<T, F>(
        &mut self,
        ctx: &Context,
        values: &[T],
        title: String,
        timeout: Duration,
        button: F
    ) -> SerenityResult<Option<HashSet<&T>>>
    where
        T: Display + Eq + Hash + Send + Sync,
        F: Fn(&T) -> (ReactionType, String) + Send + Sync {

            let mut base_emb = CreateEmbed::default();
            base_emb.title(title);

            let mut mapping: HashMap<String, &T> = HashMap::with_capacity(values.len());
            let mut curr_page: usize = 0;

            let paged_components = {
                // We can have 5 Buttons for each action row
                let value_chunks: Vec<_> = values.chunks(5).collect();
                // Total of 4 rows available for selection. Rest is confirm, abort, ...
                let row_chunks = value_chunks.chunks(4);
                // Create Action Rows
                let mut pages: Vec<Vec<CreateActionRow>> = Vec::with_capacity(row_chunks.len());
                for rows in row_chunks {
                    let mut new_page = Vec::new();
                    for row in rows {
                        let mut ar = CreateActionRow::default();
                        for b in row.iter() {
                            let (emoji, button_title) = button(b);
                            let mut button = CreateButton::default();
                            let custom_id = format!("_tools_selector_{}", &button_title);
                            button
                                .emoji(emoji)
                                .label(&button_title)
                                .style(ButtonStyle::Primary)
                                .custom_id(&custom_id);

                            mapping.insert(custom_id, b);

                            ar.add_button(button);
                        }
                        new_page.push(ar);
                    }
                    pages.push(new_page);
                }
                pages
            };

            if paged_components.is_empty() { return Ok(Some(HashSet::new())) }

            // keep track of what is selected
            let selected: HashSet<&T> = HashSet::new();

            let emb = paged_selector_embed(base_emb.clone(), values, &selected);

            self.edit(ctx, |m| {
                m.set_embed(emb);
                m.components(|c| {
                    c.set_action_rows(paged_components.get(curr_page).unwrap().to_vec());
                    c.create_action_row(|ar| {
                        // TODO next, prev page
                        ar.confirm_button().abort_button()
                    })
                });
                m
            }).await?;

            Ok(Some(selected))
        }
}
