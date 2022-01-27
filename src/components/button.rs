use std::str::FromStr;

use serenity::{
    builder::CreateButton,
    model::{channel::ReactionType, interactions::message_component::ButtonStyle},
};

pub(crate) struct ButtonInfo {
    text: &'static str,
    id: &'static str,
    emoji: &'static str,
}

impl ButtonInfo {
    pub(crate) fn create(&self) -> CreateButton {
        let mut button = CreateButton::default();
        button.label(self.text);
        button.custom_id(self.id);
        button.style(ButtonStyle::Primary);
        button.emoji(ReactionType::try_from(self.emoji).unwrap());
        button
    }
}

macro_rules! button {
    ($name:ident,$text:literal,$emoji:literal) => {
        pub(crate) const $name: ButtonInfo = ButtonInfo {
            text: $text,
            id: concat!("_tools_button_", $text),
            emoji: $emoji,
        };
    };
}

button!(CONFIRM_BUTTON, "confirm", "✅");
button!(ABORT_BUTTON, "abort", "❌");
button!(NEXT_BUTTON, "next", "⏩");
button!(PREV_BUTTON, "previous", "⏪");

pub enum Button {
    Confirm,
    Abort,
    Next,
    Previous,
}

impl Button {
    pub fn create(&self) -> CreateButton {
        match self {
            Self::Confirm => {
                let mut b = CONFIRM_BUTTON.create();
                b.style(ButtonStyle::Success);
                b
            }
            Self::Abort => {
                let mut b = ABORT_BUTTON.create();
                b.style(ButtonStyle::Danger);
                b
            }
            Self::Next => {
                let mut b = NEXT_BUTTON.create();
                b.style(ButtonStyle::Primary);
                b
            }
            Self::Previous => {
                let mut b = PREV_BUTTON.create();
                b.style(ButtonStyle::Primary);
                b
            }
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::Confirm => CONFIRM_BUTTON.id,
            Self::Abort => ABORT_BUTTON.id,
            Self::Next => NEXT_BUTTON.id,
            Self::Previous => PREV_BUTTON.id,
        }
    }
}

#[derive(Debug)]
pub struct ButtonParseError(String);

impl std::fmt::Display for ButtonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown custom button id: {}", self)
    }
}

impl std::error::Error for ButtonParseError {}

// Can this be macro'd?
impl FromStr for Button {
    type Err = ButtonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "_tools_button_confirm" => Ok(Button::Confirm),
            "_tools_button_abort" => Ok(Button::Abort),
            "_tools_button_next" => Ok(Button::Next),
            "_tools_button_previous" => Ok(Button::Previous),
            _ => Err(ButtonParseError(s.to_string())),
        }
    }
}
