use serenity::model::interactions::message_component::MessageComponentInteraction;

use crate::components::{Button, ButtonParseError};

pub trait MessageComponentInteractionExt {
    fn parse_button(&self) -> Result<Button, ButtonParseError>;
}

impl MessageComponentInteractionExt for MessageComponentInteraction {
    fn parse_button(&self) -> Result<Button, ButtonParseError> {
        self.data.custom_id.parse()
    }
}
