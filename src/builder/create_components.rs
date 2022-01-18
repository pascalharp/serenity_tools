use serenity::{
    builder::{CreateActionRow, CreateButton},
    model::{channel::ReactionType, interactions::message_component::ButtonStyle},
};

struct ButtonInfo {
    text: &'static str,
    id: &'static str,
    emoji: char,
}

impl ButtonInfo {
    fn create(&self) -> CreateButton {
        let mut button = CreateButton::default();
        button.label(CONFIRM_BUTTON.text);
        button.custom_id(CONFIRM_BUTTON.id);
        button.emoji(ReactionType::from(CONFIRM_BUTTON.emoji));
        button
    }
}

macro_rules! button {
    ($name:ident,$text:literal,$emoji:literal) => {
        const $name: ButtonInfo = ButtonInfo {
            text: $text,
            id: concat!("_tools_", $text),
            emoji: $emoji,
        };
    };
}

button!(CONFIRM_BUTTON, "confirm", '✅');
button!(ABORT_BUTTON, "abort", '❌');

// confirm
pub trait CreateActionRowExt {
    fn confirm_button(&mut self) -> &mut Self;
    fn abort_button(&mut self) -> &mut Self;
}

impl CreateActionRowExt for CreateActionRow {
    fn confirm_button(&mut self) -> &mut Self {
        let mut button = CONFIRM_BUTTON.create();
        button.style(ButtonStyle::Success);
        self.add_button(button)
    }

    fn abort_button(&mut self) -> &mut Self {
        let mut button = ABORT_BUTTON.create();
        button.style(ButtonStyle::Danger);
        self.add_button(button)
    }
}
