use iced::border::Radius;
use iced::widget::button::{self, Appearance};
use iced::Background;

use super::{ZeroPassTheme, ACCENT, PRIMARY, SECONDARY, TEXT};

#[derive(Debug, Clone, Default)]
pub enum ButtonStyle {
    #[default]
    Generate,
    AddMethod,
    RemoveMethod,
}

impl button::StyleSheet for ZeroPassTheme {
    type Style = ButtonStyle;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            ButtonStyle::Generate => generate_button_active_style(),
            ButtonStyle::AddMethod => add_method_button_active_style(),
            ButtonStyle::RemoveMethod => remove_method_button_active_style(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        match style {
            ButtonStyle::Generate => generate_button_hovered_style(),
            ButtonStyle::AddMethod => add_method_button_hovered_style(),
            ButtonStyle::RemoveMethod => remove_method_button_hovered_style(),
        }
    }
}

fn generate_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(PRIMARY)),
        border: iced::Border {
            color: PRIMARY,
            radius: Radius::from(4),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn add_method_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY)),
        border: iced::Border {
            color: SECONDARY,
            radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn remove_method_button_active_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY)),
        border: iced::Border {
            color: SECONDARY,
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
            ..Default::default()
        },
        ..Appearance::default()
    }
}

fn generate_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(PRIMARY)),
        border: iced::Border {
            color: PRIMARY,
            width: 3.0,
            radius: Radius::from(4),
        },
        shadow: iced::Shadow {
            color: PRIMARY,
            blur_radius: 10.0,
            offset: iced::Vector { x: 0.0, y: 0.0 },
        },
        ..Appearance::default()
    }
}

fn add_method_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY)),
        border: iced::Border {
            color: ACCENT,
            width: 2.0,
            radius: Radius::from([4.0, 0.0, 0.0, 4.0]),
        },
        ..Appearance::default()
    }
}

fn remove_method_button_hovered_style() -> Appearance {
    Appearance {
        background: Some(Background::Color(SECONDARY)),
        border: iced::Border {
            color: ACCENT,
            width: 2.0,
            radius: Radius::from([0.0, 4.0, 4.0, 0.0]),
        },
        ..Appearance::default()
    }
}