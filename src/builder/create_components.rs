use crate::components::Button;
use serenity::builder::{CreateActionRow, CreateComponents};

pub trait CreateActionRowExt {
    fn confirm_button(&mut self) -> &mut Self;
    fn abort_button(&mut self) -> &mut Self;
}

impl CreateActionRowExt for CreateActionRow {
    fn confirm_button(&mut self) -> &mut Self {
        self.add_button(Button::Confirm.create())
    }

    fn abort_button(&mut self) -> &mut Self {
        self.add_button(Button::Abort.create())
    }
}

pub trait CreateComponentsExt {
    fn confirm_abort_row(&mut self) -> &mut Self;
}

impl CreateComponentsExt for CreateComponents {
    fn confirm_abort_row(&mut self) -> &mut Self {
        self.create_action_row(|ar| ar.confirm_button().abort_button())
    }
}
