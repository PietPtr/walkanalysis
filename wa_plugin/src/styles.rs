use nih_plug_iced::{overlay::menu, *};

use crate::colors;

pub struct MyContainerStyle {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl Default for MyContainerStyle {
    fn default() -> Self {
        Self {
            text_color: Default::default(),
            background: Default::default(),
            border_radius: Default::default(),
            border_width: Default::default(),
            border_color: Default::default(),
        }
    }
}

impl container::StyleSheet for MyContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}

pub struct MyPicklistStyle {}

impl pick_list::StyleSheet for MyPicklistStyle {
    fn menu(&self) -> menu::Style {
        menu::Style {
            text_color: Color::BLACK,
            background: Background::Color(Color::WHITE),
            selected_text_color: Color::WHITE,
            selected_background: Background::Color(colors::GREEN),
            border_width: 1.0,
            border_color: Color::BLACK,
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::BLACK,
            placeholder_color: colors::GREY,
            background: Background::Color(Color::WHITE),
            border_radius: 2.0,
            border_width: 1.0,
            border_color: Color::BLACK,
            icon_size: 0.7,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::WHITE,
            background: Background::Color(colors::GREEN),
            ..Self::active(&self)
        }
    }
}

pub struct MyTogglerStyle {
    pub colored: bool,
}

impl Default for MyTogglerStyle {
    fn default() -> Self {
        Self { colored: false }
    }
}

impl toggler::StyleSheet for MyTogglerStyle {
    fn active(&self, is_active: bool) -> toggler::Style {
        toggler::Style {
            background: if self.colored {
                if is_active {
                    colors::RED
                } else {
                    colors::GREEN
                }
            } else {
                colors::GREY
            },
            background_border: None,
            foreground: Color::WHITE,
            foreground_border: None,
        }
    }

    fn hovered(&self, is_active: bool) -> toggler::Style {
        toggler::Style {
            foreground: Color::from_rgb(0.95, 0.95, 0.95),
            ..self.active(is_active)
        }
    }
}
