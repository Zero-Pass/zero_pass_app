mod generate_area;
mod method_info_list;
mod unique_pass_field;
mod variable_pass_field;
mod toggle_theme_button;
pub mod widgets;

use crate::ZeroPass;

use self::widgets::ZeroPassTheme;

use super::Message;
use iced::{
    widget::{column, container},
    Length,
};

pub struct UI;

impl UI {
    pub fn build<'a>(zero_pass: &ZeroPass) -> iced::Element<'a, Message, ZeroPassTheme> {
        container(
            column![
                // Button::new(Text::new("X")).on_press(Message::CloseWindow),
                toggle_theme_button::toggle_theme_button(&zero_pass.theme),
                column![
                    unique_pass_field::unique_pass_field(&zero_pass.unique),
                    variable_pass_field::variable_pass_field(&zero_pass.variable),
                ]
                .spacing(10),
                method_info_list::method_info_list(&zero_pass.methods),
                generate_area::generate_area(&zero_pass.result)
            ]
            .spacing(30),
        )
        .padding(20)
        .height(Length::Fill)
        .center_y()
        .into()
    }
}
