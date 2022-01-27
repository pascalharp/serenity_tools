mod component_interaction_collector;

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    time::Duration,
};

pub use component_interaction_collector::MessageCollectorExt;

use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed},
    client::Context,
    futures::StreamExt,
    model::{
        channel::ReactionType,
        interactions::{
            application_command::ApplicationCommandInteraction,
            message_component::{ButtonStyle, MessageComponentInteraction},
        },
        prelude::Message,
    },
    Result as SerenityResult,
};
use tokio::{select, time::sleep};

use crate::{
    builder::CreateActionRowExt, components::Button, interactions::MessageComponentInteractionExt,
};

// Since ephemeral Messages cant be updated through Message
// this is a bit of a work around.
// Still need the message for the collector
pub enum UpdatAbleMessage<'a> {
    Message(&'a mut Message),
    ApplicationCommand(&'a ApplicationCommandInteraction, &'a mut Message),
    ComponentInteraction(&'a MessageComponentInteraction, &'a mut Message),
}

#[derive(Clone, Debug)]
pub struct PagedSelectorConfig {
    base_embed: CreateEmbed,
    unselected_emoj: ReactionType,
    selected_emoji: ReactionType,
    // How many buttons in each action row. Must be between 1 and 5
    items_rows: usize,
    // How many action rows per page. Must be between 1 and 4
    rows_pages: usize,
    // Gets reset after every input
    timeout: Duration,
    // minimum selection required
    min: usize,
}

impl Default for PagedSelectorConfig {
    fn default() -> Self {
        Self {
            base_embed: Default::default(),
            unselected_emoj: ReactionType::try_from("⬛").unwrap(),
            selected_emoji: ReactionType::try_from("✅").unwrap(),
            items_rows: 5,
            rows_pages: 4,
            timeout: Duration::from_secs(60),
            min: 0,
        }
    }
}

impl PagedSelectorConfig {
    pub fn set_base_embed(&mut self, base_embed: CreateEmbed) -> &mut Self {
        self.base_embed = base_embed;
        self
    }

    pub fn set_unselected_emoji(&mut self, emoji: ReactionType) -> &mut Self {
        self.unselected_emoj = emoji;
        self
    }

    pub fn set_selected_emoji(&mut self, emoji: ReactionType) -> &mut Self {
        self.selected_emoji = emoji;
        self
    }

    pub fn set_items_per_row(&mut self, count: usize) -> &mut Self {
        self.items_rows = count;
        self
    }

    pub fn set_rows_per_page(&mut self, count: usize) -> &mut Self {
        self.rows_pages = count;
        self
    }

    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn min_select(&mut self, min: usize) -> &mut Self {
        self.min = min;
        self
    }
}

impl<'a> UpdatAbleMessage<'a> {
    pub async fn update(
        &mut self,
        ctx: &Context,
        embeds: Vec<CreateEmbed>,
        ars: Vec<CreateActionRow>,
    ) -> SerenityResult<()> {
        match self {
            Self::Message(msg) => {
                msg.edit(ctx, |m| {
                    m.set_embeds(embeds);
                    m.components(|c| c.set_action_rows(ars))
                })
                .await
            }
            Self::ApplicationCommand(aci, _) => {
                aci.edit_original_interaction_response(ctx, |m| {
                    m.set_embeds(embeds);
                    m.components(|c| c.set_action_rows(ars))
                })
                .await?;
                Ok(())
            }
            Self::ComponentInteraction(mci, _) => {
                mci.edit_original_interaction_response(ctx, |m| {
                    m.set_embeds(embeds);
                    m.components(|c| c.set_action_rows(ars))
                })
                .await?;
                Ok(())
            }
        }
    }

    pub fn msg(&self) -> &Message {
        match self {
            Self::Message(msg) => msg,
            Self::ApplicationCommand(_, msg) => msg,
            Self::ComponentInteraction(_, msg) => msg,
        }
    }

    pub fn release(self) {}

    pub async fn paged_selector<'b, 'c, T, F>(
        &'c mut self,
        ctx: &Context,
        config: PagedSelectorConfig,
        values: &'b [T],
        button: F,
    ) -> SerenityResult<Option<HashSet<&'b T>>>
    where
        T: Display + Eq + Hash + Send + Sync,
        F: Fn(&T) -> (ReactionType, String) + Send + Sync,
    {
        let mut mapping: HashMap<String, &T> = HashMap::with_capacity(values.len());
        let mut curr_page: usize = 0;

        let paged_components = {
            // We can have up to 5 Buttons for each action row
            let value_chunks: Vec<_> = values.chunks(config.items_rows).collect();
            // Total of 4 rows available for selection. Rest is confirm, abort, ...
            let row_chunks = value_chunks.chunks(config.rows_pages);
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

        if paged_components.is_empty() {
            return Ok(Some(HashSet::new()));
        }

        // keep track of what is selected
        let mut selected: HashSet<&T> = HashSet::new();

        let emb = vec![paged_selector_embed(&config, values, &selected, curr_page)];
        let mut ar = paged_components.get(curr_page).unwrap().to_vec();
        let mut sar = CreateActionRow::default();
        let mut conf_button = Button::Confirm.create();
        if selected.len() < config.min {
            conf_button.disabled(true);
        }
        sar.add_button(conf_button).abort_button();
        if curr_page > 0 {
            sar.prev_button();
        }
        if curr_page < paged_components.len() - 1 {
            sar.next_button();
        }
        ar.push(sar);
        self.update(ctx, emb, ar).await?;

        let mut interactions = self.msg().await_component_interactions(ctx).await;

        loop {
            // using select instead of collector timeout to reset
            // timeout after button click
            select! {
                react = interactions.next() => {
                    // Should always be some
                    let react = match react {
                        Some(r) => r,
                        None => return Ok(None),
                    };

                    react.defer(ctx).await?;

                    match react.parse_button() {
                        // a default button
                        Ok(b) => match b {
                            Button::Confirm => break,
                            Button::Abort => return Ok(None),
                            Button::Next => curr_page += 1,
                            Button::Previous => curr_page -= 1,
                        },
                        // Selected an item
                        Err(_) => {
                            let selected_t = mapping.get(&react.data.custom_id).unwrap();
                            if !selected.remove(selected_t) { selected.insert(selected_t); };
                        }
                    }

                    let emb = vec![paged_selector_embed(&config, values, &selected, curr_page)];
                    let mut ar = paged_components.get(curr_page).unwrap().to_vec();
                    let mut sar = CreateActionRow::default();
                    let mut conf_button = Button::Confirm.create();
                    if selected.len() < config.min {
                        conf_button.disabled(true);
                    }
                    sar.add_button(conf_button).abort_button();
                    if curr_page > 0 {
                        sar.prev_button();
                    }
                    if curr_page < paged_components.len() - 1 {
                        sar.next_button();
                    }
                    ar.push(sar);
                    self.update(ctx, emb, ar).await?;
                },
                _ = sleep(config.timeout) => return Ok(None),
            }
        }

        interactions.stop();
        // remove components
        let emb = vec![paged_selector_embed(&config, values, &selected, curr_page)];
        let ars = Vec::new();
        self.update(ctx, emb, ars).await?;

        Ok(Some(selected))
    }
}

fn paged_selector_embed<T: Display + Eq + Hash>(
    config: &PagedSelectorConfig,
    values: &[T],
    selected: &HashSet<&T>,
    curr_page: usize,
) -> CreateEmbed {
    let mut emb = config.base_embed.clone();
    let role_fields = values.chunks(config.items_rows * config.rows_pages);
    for (i, e) in role_fields.enumerate() {
        emb.field(
            format!(
                "Page {}{}",
                i + 1,
                if i == curr_page { " (current)" } else { "" }
            ),
            e.iter()
                .map(|t| {
                    format!(
                        "{} | {}",
                        if selected.contains(t) {
                            &config.selected_emoji
                        } else {
                            &config.unselected_emoj
                        },
                        t.to_string()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
            true,
        );
    }
    emb
}
